use aerosmart_shared::serial::{
    AcknowledgementConfig, AcknowledgementData, ArchivedSerialMessage, SerialMessage,
};
use anyhow::Context;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use clap::Parser;
use futures::{sink::SinkExt, stream::StreamExt};
use rkyv::{
    api::low::to_bytes_in_with_alloc,
    rancor::Failure,
    ser::{allocator::SubAllocator, writer::Buffer},
    util::Align,
};
use serde::{Deserialize, Serialize};
use std::{
    mem::MaybeUninit,
    net::SocketAddr,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::{broadcast, mpsc},
    time::timeout,
};
use tokio_serial::{SerialPortBuilderExt, SerialStream};
use tower_http::cors::CorsLayer;
use tracing::{error, info, warn};

/// AeroSmart Service
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Serial port name (e.g., /dev/tty.usbmodem1234 or COM3)
    #[arg(short, long, default_value = "/dev/tty.usbmodem1234")]
    port: String,

    /// Baud rate
    #[arg(short, long, default_value_t = 915200)]
    baud: u32,
}

#[derive(Clone)]
struct AppState {
    tx: broadcast::Sender<String>,
    cmd_tx: mpsc::Sender<SerialMessage>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    info!("Starting AeroSmart Service on port {} @ {}", args.port, args.baud);

    // Channels
    // Broadcast: Serial -> WebSockets (Telemetry JSON)
    let (tx, _rx) = broadcast::channel::<String>(100);
    // MPSC: WebSockets -> Serial (Command SerialMessage)
    let (cmd_tx, mut cmd_rx) = mpsc::channel::<SerialMessage>(100);

    let state = AppState {
        tx: tx.clone(),
        cmd_tx: cmd_tx.clone(),
    };

    // Spawn Serial Task
    let port_name = args.port.clone();
    let baud_rate = args.baud;
    tokio::spawn(async move {
        // Retry loop for serial connection
        loop {
            if let Err(e) = serial_task(port_name.clone(), baud_rate, tx.clone(), &mut cmd_rx).await {
                error!("Serial task failed: {:?}. Retrying in 5s...", e);
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    });

    // Setup Axum Router
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn serial_task(
    port_name: String,
    baud_rate: u32,
    tx: broadcast::Sender<String>,
    cmd_rx: &mut mpsc::Receiver<SerialMessage>,
) -> anyhow::Result<()> {
    info!("Opening serial port {}...", port_name);
    let mut port = tokio_serial::new(&port_name, baud_rate)
        .open_native_async()
        .context("Failed to open serial port")?;

    #[cfg(unix)]
    port.set_exclusive(false)
        .context("Failed to set exclusive mode")?;

    info!("Serial port opened. Waiting for Handshake (Ping)...");

    // --- Phase 1: Handshake (Length-Prefixed) ---
    // We expect the firmware to send: [u32 len] [payload]
    // The payload should be AcknowledgementData.
    loop {
        let len = match timeout(Duration::from_secs(2), port.read_u32_le()).await {
            Ok(Ok(len)) => len as usize,
            Ok(Err(e)) => {
                 warn!("Handshake read error: {:?}", e);
                 continue;
            }
            Err(_) => {
                // Timeout, waiting for firmware
                continue;
            }
        };

        if len > 1024 {
            warn!("Invalid handshake packet length: {}", len);
            // Try to flush/recover? For now just continue reading.
            continue;
        }

        let mut buf = vec![0u8; len];
        port.read_exact(&mut buf).await.context("Failed to read handshake payload")?;

        let valid_ping = match rkyv::access::<ArchivedSerialMessage, rkyv::rancor::Error>(&buf) {
            Ok(archived) => match archived {
                 ArchivedSerialMessage::AcknowledgementData(_) => true,
                 _ => false,
            },
            Err(_) => false,
        };

        if valid_ping {
            info!("Received Ping. Sending Pong...");
            
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;

            let pong = SerialMessage::AcknowledgementConfig(AcknowledgementConfig {
                ack: true,
                unix_timestamp_ms: now,
            });

            let bytes = serialize_message(&pong)?;
            port.write_all(&bytes).await?;
            info!("Pong sent. Handshake complete. Entering Main Loop.");
            break;
        } else {
            warn!("Received non-ping message during handshake");
        }
    }

    // --- Phase 2: Main Loop ---
    loop {
        tokio::select! {
            // Uplink: Serial -> WebSocket
            // 1. Read Length (u32)
            res = port.read_u32_le() => {
                let len = match res {
                    Ok(l) => l as usize,
                    Err(e) => {
                        return Err(anyhow::anyhow!("Serial read error: {:?}", e));
                    }
                };
                
                if len > 4096 {
                     warn!("Oversized packet: {}. Skipping...", len);
                     // In a real stream, we would need to re-sync. 
                     // Since we read exactly 4 bytes, we might be misaligned.
                     // But with length-prefix, if we are aligned, we stay aligned.
                     continue;
                }

                // 2. Read Payload
                let mut buf = vec![0u8; len];
                port.read_exact(&mut buf).await.context("Failed to read payload")?;

                // 3. Deserialize & Broadcast
                match rkyv::access::<ArchivedSerialMessage, rkyv::rancor::Error>(&buf) {
                    Ok(archived) => {
                        match rkyv::deserialize::<SerialMessage, rkyv::rancor::Error>(archived) {
                            Ok(native) => {
                                let json = serde_json::to_string(&native)?;
                                let _ = tx.send(json);
                            }
                            Err(e) => {
                                warn!("Deserialize error: {:?}", e);
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Rkyv access error: {:?}", e);
                    }
                }
            }

            // Downlink: WebSocket -> Serial
            Some(msg) = cmd_rx.recv() => {
                let bytes = serialize_message(&msg)?;
                port.write_all(&bytes).await?;
            }
        }
    }
}

fn serialize_message(message: &SerialMessage) -> anyhow::Result<Vec<u8>> {
    let mut output = Align([MaybeUninit::<u8>::uninit(); 256]);
    let mut alloc = [MaybeUninit::<u8>::uninit(); 256];

    let bytes = to_bytes_in_with_alloc::<_, _, Failure>(
        message,
        Buffer::from(&mut *output),
        SubAllocator::new(&mut alloc),
    )
    .map_err(|e| anyhow::anyhow!("Serialization error: {:?}", e))?;

    // Add Length Prefix
    let len = bytes.len() as u32;
    let mut framed = Vec::with_capacity(4 + bytes.len());
    framed.extend_from_slice(&len.to_le_bytes());
    framed.extend_from_slice(&bytes);

    Ok(framed)
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    let mut rx = state.tx.subscribe();

    loop {
        tokio::select! {
            Ok(msg) = rx.recv() => {
                if socket.send(Message::Text(msg)).await.is_err() {
                    break;
                }
            }

            Some(Ok(msg)) = socket.recv() => {
                if let Message::Text(text) = msg {
                     // Parse JSON to SerialMessage
                     match serde_json::from_str::<SerialMessage>(&text) {
                        Ok(serial_msg) => {
                            if let Err(e) = state.cmd_tx.send(serial_msg).await {
                                error!("Failed to send command: {:?}", e);
                                break;
                            }
                        }
                        Err(e) => {
                            error!("JSON parse error: {:?} | {}", e, text);
                        }
                     }
                }
            }
        }
    }
}
