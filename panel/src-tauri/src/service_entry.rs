use crate::config::{self, AppConfig};
use crate::system::wifi;
use aerosmart_shared::serial::{AcknowledgementConfig, ArchivedSerialMessage, SerialMessage};
use anyhow::Context;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::{get, post},
    Router,
};
// use clap::Parser; // No longer needed
use log::{error, info, warn};
use std::{
    net::SocketAddr,
    process::Command,
    str::FromStr,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::{broadcast, mpsc, Notify, RwLock},
    time::timeout,
};
use tokio_serial::SerialPortBuilderExt;
use tower_http::cors::CorsLayer;

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
enum ActivationStatus {
    Idle,                   // Waiting for activation command
    Connecting,             // connecting to serial
    Handshaking,            // Waiting for ping
    WaitingForFirstMessage, // Ping/Pong done, waiting for data
    Active,                 // Success
    Failed(String),
}

#[derive(Clone)]
struct AppState {
    tx: broadcast::Sender<String>,
    cmd_tx: mpsc::Sender<SerialMessage>,
    #[allow(unused)]
    config: AppConfig,
    activation_signal: Arc<Notify>,
    restart_signal: Arc<Notify>,
    activation_status: Arc<RwLock<ActivationStatus>>,
}

pub async fn run() -> anyhow::Result<()> {
    // Logging is handled by Tauri or tracing_subscriber outside

    // Load configuration
    config::create_default_config_file_if_missing();
    let config = config::load_config();
    info!("Service Config Loaded: {:?}", config);

    // Channels
    // Broadcast: Serial -> WebSockets (Telemetry JSON)
    let (tx, _rx) = broadcast::channel::<String>(100);
    // MPSC: WebSockets -> Serial (Command SerialMessage)
    let (cmd_tx, mut cmd_rx) = mpsc::channel::<SerialMessage>(100);

    let activation_signal = Arc::new(Notify::new());
    let restart_signal = Arc::new(Notify::new());
    // If onboarding is enabled, we start in Idle and wait for signal.
    // If disabled, we treat it as already "Active" in terms of flow, but effectively we just start connecting.
    // We'll set it to Connecting initially if not onboarding.
    let initial_status = if config.rules.enable_onboarding {
        ActivationStatus::Idle
    } else {
        ActivationStatus::Connecting
    };
    let activation_status = Arc::new(RwLock::new(initial_status));

    let state = AppState {
        tx: tx.clone(),
        cmd_tx: cmd_tx.clone(),
        config: config.clone(),
        activation_signal: activation_signal.clone(),
        restart_signal: restart_signal.clone(),
        activation_status: activation_status.clone(),
    };

    // Spawn Serial Task
    let serial_config = config.serial.clone();
    let tx_clone = tx.clone();
    let signal_clone = activation_signal.clone();
    let restart_signal_clone = restart_signal.clone();
    let status_clone = activation_status.clone();

    tokio::spawn(async move {
        // Retry loop for serial connection
        loop {
            // Check if we need to wait for activation
            let should_wait = {
                let s = status_clone.read().await;
                *s == ActivationStatus::Idle
            };

            if should_wait {
                info!("Serial Task: Waiting for activation signal...");
                signal_clone.notified().await;
                info!("Serial Task: Activation signal received!");
            }

            // Update status to Connecting
            {
                let mut s = status_clone.write().await;
                *s = ActivationStatus::Connecting;
            }

            // Create a channel to signal forced restart from the task
            let (abort_tx, mut abort_rx) = mpsc::channel::<()>(1);
            let restart_signal_inner = restart_signal_clone.clone();

            // Spawn a watcher for restart signal
            let watcher_handle = tokio::spawn(async move {
                restart_signal_inner.notified().await;
                let _ = abort_tx.send(()).await;
            });

            tokio::select! {
                res = serial_task(
                    serial_config.clone(),
                    tx_clone.clone(),
                    &mut cmd_rx,
                    status_clone.clone(),
                ) => {
                    // Task finished (likely error)
                    watcher_handle.abort();
                    if let Err(e) = res {
                        error!(
                            "Serial task failed: {:?}. Retrying in {}s...",
                            e, serial_config.retry_interval_secs
                        );
                        {
                            let mut s = status_clone.write().await;
                            *s = ActivationStatus::Failed(e.to_string());
                        }
                        tokio::time::sleep(Duration::from_secs(serial_config.retry_interval_secs)).await;
                    }
                }
                _ = abort_rx.recv() => {
                    // Forced restart
                    warn!("Serial task aborted by restart signal. Restarting...");
                    // Loop will continue and restart connection
                }
            }
        }
    });

    // Setup Axum Router
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .route("/api/wifi/scan", get(wifi::scan_handler))
        .route("/api/wifi/connect", post(wifi::connect_handler))
        .route("/api/wifi/disconnect", post(wifi::disconnect_handler))
        .route("/api/wifi/status", get(wifi::status_handler))
        .route("/api/wifi/test", get(wifi::test_handler))
        .route("/api/activation/start", post(start_activation))
        .route("/api/activation/status", get(get_activation_status))
        .route("/api/activation/restart", post(restart_activation))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr_str = format!("{}:{}", config.server.host, config.server.port);
    let addr =
        SocketAddr::from_str(&addr_str).unwrap_or_else(|_| SocketAddr::from(([0, 0, 0, 0], 3000)));

    info!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn serial_task(
    config: config::SerialConfig,
    tx: broadcast::Sender<String>,
    cmd_rx: &mut mpsc::Receiver<SerialMessage>,
    status: Arc<RwLock<ActivationStatus>>,
) -> anyhow::Result<()> {
    info!(
        "Opening serial port {} @ {}...",
        config.port, config.baud_rate
    );
    let mut port = tokio_serial::new(&config.port, config.baud_rate)
        .open_native_async()
        .context("Failed to open serial port")?;

    #[cfg(unix)]
    port.set_exclusive(false)
        .context("Failed to set exclusive mode")?;

    info!("Serial port opened. Waiting for Handshake (Ping)...");

    {
        let mut s = status.write().await;
        *s = ActivationStatus::Handshaking;
    }

    // --- Phase 1: Handshake (Length-Prefixed) ---
    // We expect the firmware to send: [u32 len] [payload]
    // The payload should be AcknowledgementData.
    loop {
        let len = match timeout(
            Duration::from_secs(config.handshake_timeout_secs),
            port.read_u32_le(),
        )
        .await
        {
            Ok(Ok(len)) => len as usize,
            Ok(Err(e)) => {
                warn!("Handshake read error: {:?}", e);
                // Send a PING anyway if we are timing out, maybe the device is waiting?
                // Actually the requirement says: "10s no packet automatically send Ping return to handshake process"
                // This loop IS the handshake process.
                // But wait, the requirement says "10s No packet automatically send Ping return to handshake process".
                // This likely means if we are in Main Loop and don't receive anything for 10s, we should restart handshake.
                // OR it means during handshake if we don't get anything, we should send something?
                // Usually handshake is: Device sends Ping -> Host sends Pong.
                // If Host doesn't receive Ping, it can't do anything.
                // UNLESS the roles are swapped or we want to trigger the device?
                // Re-reading: "10s 无回包自动发送 Ping 回到握手流程"
                // It seems to mean: In the main loop, if no packet received for 10s, treat as disconnected and restart handshake.
                // Let's implement that in Phase 2.
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
        port.read_exact(&mut buf)
            .await
            .context("Failed to read handshake payload")?;

        let valid_ping = matches!(
            rkyv::access::<ArchivedSerialMessage, rkyv::rancor::Error>(&buf).and_then(|archived| {
                rkyv::deserialize::<SerialMessage, rkyv::rancor::Error>(archived)
            }),
            Ok(SerialMessage::AcknowledgementData(_))
        );

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

            // let bytes = serialize_message(&pong)?;
            let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&pong)
                .map_err(|e| anyhow::anyhow!("Serialization error: {:?}", e))?;

            info!("Pong sent: {:?}", bytes);

            // Send Length Prefix (u32 little-endian)
            let len = bytes.len() as u32;
            port.write_all(&len.to_le_bytes())
                .await
                .context("Failed to write handshake length prefix")?;

            port.write_all(&bytes).await?;
            info!("Pong sent. Handshake complete. Entering Main Loop.");

            {
                let mut s = status.write().await;
                *s = ActivationStatus::WaitingForFirstMessage;
            }
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
            // Added timeout for 10s silence detection
            res = timeout(Duration::from_secs(10), port.read_u32_le()) => {
                let len = match res {
                    Ok(Ok(l)) => l as usize,
                    Ok(Err(e)) => {
                        return Err(anyhow::anyhow!("Serial read error: {:?}", e));
                    }
                    Err(_) => {
                        // Timeout: 10s silence
                        // Requirement: "10s No packet automatically send Ping return to handshake process"
                        // This implies we should fail this task so the outer loop restarts it (which restarts handshake).
                        // Or we can try to send a Ping here if the protocol supports Host->Device Ping.
                        // Assuming "return to handshake process" means we should restart the connection/handshake flow.
                        warn!("10s silence detected. Restarting handshake...");
                        return Err(anyhow::anyhow!("Serial timeout (10s silence)"));
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

                info!("RX [{} bytes]: {:02X?}", len, buf);
                if len == 96 {
                    info!("Confirmed packet size: 96 bytes (Exact Match)");
                } else {
                    warn!("Packet size mismatch: expected 96, got {}", len);
                }

                // 3. Deserialize & Broadcast
                match rkyv::access::<ArchivedSerialMessage, rkyv::rancor::Error>(&buf) {
                    Ok(archived) => {
                        match rkyv::deserialize::<SerialMessage, rkyv::rancor::Error>(archived) {
                            Ok(native) => {
                                info!("RX Deserialized: {:?}", native);
                                // Update status if waiting
                                {
                                    // Use try_write to avoid blocking if not needed, or just write.
                                    // Optimization: only write if needed.
                                    // However, we need to read to check.
                                    // Let's just blindly upgrade if in WaitingForFirstMessage state.
                                    // This lock contention might be high if we do it every packet?
                                    // Since this is 100Hz or so, it might be fine.
                                    // But better to check a boolean flag?
                                    // Or just check once.
                                    // I'll assume we can check every time for now or optimize later.
                                    // Actually, let's optimize: only check if we suspect we are waiting.
                                    // But we don't have local state.
                                    // Let's just do it.
                                    let mut s = status.write().await;
                                    if *s == ActivationStatus::WaitingForFirstMessage {
                                        *s = ActivationStatus::Active;
                                        info!("Activation Complete: First message received.");
                                    }
                                }

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
                info!("TX Command: {:?}", msg);
                let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&msg)
                    .map_err(|e| anyhow::anyhow!("Serialization error: {:?}", e))?;

                // Send Length Prefix (u32 little-endian)
                // Added for optimization to help the firmware know exactly how many bytes to read.
                let len = bytes.len() as u32;
                port.write_all(&len.to_le_bytes()).await.context("Failed to write command length prefix")?;

                port.write_all(&bytes).await?;
                info!("TX [{} bytes]: {:02X?}", len, bytes);
            }
        }
    }
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
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

async fn start_activation(State(state): State<AppState>) -> impl IntoResponse {
    info!("Activation requested.");

    // 1. Sync Time
    if let Err(e) = sync_time().await {
        error!("Failed to sync time: {:?}", e);
    }

    // 2. Trigger Signal
    {
        let mut s = state.activation_status.write().await;
        if *s == ActivationStatus::Idle {
            *s = ActivationStatus::Connecting;
            state.activation_signal.notify_one();
            return axum::Json(serde_json::json!({ "status": "started" }));
        }
    }

    axum::Json(serde_json::json!({ "status": "already_running_or_active" }))
}

async fn restart_activation(State(state): State<AppState>) -> impl IntoResponse {
    info!("Restart activation requested.");
    state.restart_signal.notify_one();
    axum::Json(serde_json::json!({ "status": "restarting" }))
}

async fn get_activation_status(State(state): State<AppState>) -> impl IntoResponse {
    let s = state.activation_status.read().await;
    axum::Json(s.clone())
}

async fn sync_time() -> anyhow::Result<()> {
    info!("Syncing hardware clock...");

    // Try to write system time to hardware clock.
    // This assumes the OS has already synced via NTP.
    let output = tokio::process::Command::new("hwclock")
        .arg("-w")
        .output()
        .await?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("hwclock failed: {:?}", output));
    }

    info!("Hardware clock synced successfully.");
    Ok(())
}
