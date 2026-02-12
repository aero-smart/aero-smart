use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use std::process::Stdio;
use tokio::process::Command;
use tracing::{error, info};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WifiNetwork {
    pub ssid: String,
    pub signal: u8,
    pub security: String,
    pub in_use: bool,
}

#[derive(Debug, Deserialize)]
pub struct WifiConnectRequest {
    pub ssid: String,
    pub password: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct WifiStatus {
    pub connected: bool,
    pub ssid: Option<String>,
    pub ip: Option<String>,
}

use serde_json::json;

pub async fn scan_handler() -> Response {
    match scan_networks().await {
        Ok(networks) => Json(networks).into_response(),
        Err(e) => {
            error!("Failed to scan wifi: {}", e);
            let body = Json(json!({ "error": e.to_string() }));
            (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
        }
    }
}

pub async fn connect_handler(Json(req): Json<WifiConnectRequest>) -> Response {
    match connect_network(&req.ssid, req.password.as_deref()).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => {
            error!("Failed to connect to wifi: {}", e);
            let body = Json(json!({ "error": e.to_string() }));
            (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
        }
    }
}

pub async fn disconnect_handler() -> Response {
    match disconnect_network().await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => {
            error!("Failed to disconnect wifi: {}", e);
            let body = Json(json!({ "error": e.to_string() }));
            (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
        }
    }
}

pub async fn status_handler() -> Response {
    match get_status().await {
        Ok(status) => Json(status).into_response(),
        Err(e) => {
            error!("Failed to get wifi status: {}", e);
            let body = Json(json!({ "error": e.to_string() }));
            (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
        }
    }
}

pub async fn test_handler() -> Response {
    match test_connectivity().await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => {
            error!("Connectivity test failed: {}", e);
            let body = Json(json!({ "error": e.to_string() }));
            (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
        }
    }
}

// --- Implementation ---

async fn test_connectivity() -> anyhow::Result<()> {
    // Try to reach bilibili.com
    // Use reqwest to send a HEAD request or GET request
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()?;

    let res = client.get("https://www.bilibili.com").send().await?;

    if res.status().is_success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Failed to reach bilibili.com, status: {}",
            res.status()
        ))
    }
}

fn parse_scan_output(stdout: Vec<u8>) -> anyhow::Result<Vec<WifiNetwork>> {
    let stdout = String::from_utf8(stdout)?;
    let mut networks = Vec::new();

    for line in stdout.lines() {
        // Format: IN-USE:SSID:SIGNAL:SECURITY
        // Note: SSID might contain colons, but usually nmcli escapes them or we can split carefully.
        // The -t mode uses ':' as separator and escapes ':' in values with '\'.
        // For simplicity, we assume simple split for now, or use a regex.
        // A better way is to split by unescaped colons.

        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() < 4 {
            continue;
        }

        let in_use = parts[0] == "*";
        let ssid = parts[1].to_string();
        if ssid.is_empty() {
            continue;
        }
        let signal = parts[2].parse::<u8>().unwrap_or(0);
        let security = parts[3].to_string();

        networks.push(WifiNetwork {
            ssid,
            signal,
            security,
            in_use,
        });
    }

    // Deduplicate by SSID, preferring the one in use or stronger signal
    networks.sort_by(|a, b| b.signal.cmp(&a.signal));
    networks.dedup_by(|a, b| a.ssid == b.ssid);

    Ok(networks)
}

#[cfg(target_os = "linux")]
async fn scan_networks() -> anyhow::Result<Vec<WifiNetwork>> {
    // nmcli -t -f IN-USE,SSID,SIGNAL,SECURITY device wifi list
    let args = &[
        "-t",
        "-f",
        "IN-USE,SSID,SIGNAL,SECURITY",
        "device",
        "wifi",
        "list",
        "--rescan",
        "yes",
    ];

    let mut output = Command::new("nmcli")
        .args(args)
        .output()
        .await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        info!(
            "Scan failed (might be disabled): {}. Attempting to enable WiFi...",
            stderr
        );

        // Try to enable wifi
        let _ = Command::new("nmcli")
            .args(&["radio", "wifi", "on"])
            .output()
            .await;

        // Try to unblock rfkill
        let _ = Command::new("rfkill")
            .args(&["unblock", "wifi"])
            .output()
            .await;

        // Wait a bit for interface to come up
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;

        // Retry scan
        output = Command::new("nmcli")
            .args(args)
            .output()
            .await?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "nmcli failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
    }

    parse_scan_output(output.stdout)
}

#[cfg(not(target_os = "linux"))]
async fn scan_networks() -> anyhow::Result<Vec<WifiNetwork>> {
    // Return empty list or error on non-linux
    // For development convenience, we might return a dummy list if allowed,
    // but user said "No simulated data". So we return error or empty.
    // However, to allow frontend dev, maybe I should return error "Not supported on this OS".
    Err(anyhow::anyhow!("WiFi scanning not supported on this OS"))
}

#[cfg(target_os = "linux")]
async fn connect_network(ssid: &str, password: Option<&str>) -> anyhow::Result<()> {
    let mut args = vec!["device", "wifi", "connect", ssid];
    if let Some(pwd) = password {
        args.push("password");
        args.push(pwd);
    }

    let output = Command::new("nmcli").args(&args).output().await?;

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "Failed to connect: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

#[cfg(not(target_os = "linux"))]
async fn connect_network(_ssid: &str, _password: Option<&str>) -> anyhow::Result<()> {
    Err(anyhow::anyhow!("WiFi connection not supported on this OS"))
}

#[cfg(target_os = "linux")]
async fn disconnect_network() -> anyhow::Result<()> {
    // Disconnect current interface. Assuming 'wlan0' or finding the active one.
    // Safer: nmcli device disconnect wlan0
    // Or find the active connection.

    // First find the active wifi device
    let output = Command::new("nmcli")
        .args(&["-t", "-f", "DEVICE,TYPE,STATE", "device", "status"])
        .output()
        .await?;

    let stdout = String::from_utf8(output.stdout)?;
    let mut wifi_dev = None;

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() >= 3 && parts[1] == "wifi" && parts[2] == "connected" {
            wifi_dev = Some(parts[0]);
            break;
        }
    }

    if let Some(dev) = wifi_dev {
        let output = Command::new("nmcli")
            .args(&["device", "disconnect", dev])
            .output()
            .await?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "Failed to disconnect: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
    } else {
        return Err(anyhow::anyhow!("No active wifi connection found"));
    }

    Ok(())
}

#[cfg(not(target_os = "linux"))]
async fn disconnect_network() -> anyhow::Result<()> {
    Err(anyhow::anyhow!(
        "WiFi disconnection not supported on this OS"
    ))
}

#[cfg(target_os = "linux")]
async fn get_status() -> anyhow::Result<WifiStatus> {
    // nmcli -t -f TYPE,STATE,CONNECTION device status
    let output = Command::new("nmcli")
        .args(&[
            "-t",
            "-f",
            "TYPE,STATE,CONNECTION,IP4.ADDRESS",
            "device",
            "status",
        ])
        .output()
        .await?;

    let stdout = String::from_utf8(output.stdout)?;

    for line in stdout.lines() {
        // wifi:connected:MyWifi:192.168.1.100/24
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() >= 3 && parts[0] == "wifi" {
            if parts[1] == "connected" {
                return Ok(WifiStatus {
                    connected: true,
                    ssid: Some(parts[2].to_string()),
                    ip: parts.get(3).map(|s| s.to_string()),
                });
            }
        }
    }

    Ok(WifiStatus {
        connected: false,
        ssid: None,
        ip: None,
    })
}

#[cfg(not(target_os = "linux"))]
async fn get_status() -> anyhow::Result<WifiStatus> {
    Err(anyhow::anyhow!("WiFi status not supported on this OS"))
}
