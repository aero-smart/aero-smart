use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
#[cfg(target_os = "linux")]
use std::process::Stdio;
#[cfg(target_os = "linux")]
use tokio::process::Command;

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

#[cfg(target_os = "linux")]
fn parse_scan_output(stdout: Vec<u8>) -> anyhow::Result<Vec<WifiNetwork>> {
    let stdout_str = String::from_utf8(stdout)?;
    info!("nmcli scan output raw:\n{}", stdout_str);

    let mut networks = Vec::new();

    for line in stdout_str.lines() {
        // Format: IN-USE:SSID:SIGNAL:SECURITY
        // The -t mode uses ':' as separator and escapes ':' in values with '\'.
        // However, raw split works for most cases unless SSID has ':'
        // A robust parser handles escaping, but here we'll do a basic fix if needed.
        // Let's assume standard output for now.

        // nmcli output might be messy if SSID is empty.
        // :SSID:SIGNAL:SECURITY
        // If SSID is empty, it looks like ::SIGNAL:SECURITY

        let parts: Vec<&str> = line.split(':').collect();
        // We need at least 4 parts.
        // If SSID has colons, this split is wrong.
        // But for "Innoxsz-Public", it's fine.

        // Wait, look at the log:
        //  :Innoxsz-Guest:100:
        //  *:Innoxsz-Public:100:WPA1 WPA2

        // Line 1: " :Innoxsz-Guest:100:"
        // Split: [" ", "Innoxsz-Guest", "100", ""] -> len 4
        // Part 0: " " (space?) No, wait.
        // Log says: " :jiangyin14:100:WPA2 WPA3"
        // It seems there is a space before the colon? Or is it empty?
        // "IN-USE" field: "*" or " ".
        // If it is " ", then split might be: [" ", "jiangyin14", "100", "WPA2 WPA3"]

        // If line starts with ':', then part[0] is empty string "".
        // If line starts with " :", then part[0] is " ".
        // Let's look at the log carefully:
        // Feb 13 00:45:26 ...:  :jiangyin14:100:WPA2 WPA3
        // There is a space.

        // If I use `nmcli -t`, fields are separated by `:`.
        // The IN-USE field is either `*` or ` ` (space) or empty?
        // Actually, `nmcli -t` usually produces `*:SSID...` or `:SSID...` (empty string for false).
        // BUT, the log shows a space: ` :jiangyin14...`
        // Maybe the log formatting added a space?
        // "Feb 13 00:45:26 ... [INFO] nmcli scan output raw:"
        // " :jiangyin14..."
        // It's possible the log prefix alignment makes it look like a space, or it IS a space.

        // Let's trim the line first? No, if IN-USE is space, trimming might remove it?
        // But IN-USE is significant.
        // Let's rely on the position.

        // Recover logic:
        // Iterate backwards?
        // Security is last. Signal is second to last.
        // But SSID can be anything.

        // Let's try to parse flexibly.
        // We know Signal is a number.

        // If split by ':', we get N parts.
        // Last part: Security
        // Second last: Signal (u8)
        // First part: IN-USE
        // Everything in between: SSID (joined by :)

        if parts.len() < 4 {
            continue;
        }

        let in_use_str = parts[0].trim();
        let in_use = in_use_str == "*";

        let security = parts[parts.len() - 1].to_string();

        let signal_str = parts[parts.len() - 2];
        let signal = signal_str.parse::<u8>().unwrap_or(0);

        // SSID is parts[1..len-2] joined by ":"
        let ssid = parts[1..parts.len() - 2].join(":");

        if ssid.is_empty() {
            continue;
        }

        // Fix for "backslash escaped colon" if nmcli does that?
        // For now, simple join is likely correct for simple SSIDs.
        // If SSID contained ":", nmcli -t escapes it as "\:".
        // Our split would separate it.
        // e.g. "My\:Wifi" -> ["My\", "Wifi"]
        // Join back -> "My\:Wifi". We might want to unescape.
        let ssid = ssid.replace("\\:", ":");

        networks.push(WifiNetwork {
            ssid,
            signal,
            security,
            in_use,
        });
    }

    info!("Total networks parsed: {}", networks.len());

    // Deduplicate by SSID, preferring the one in use or stronger signal
    networks.sort_by(|a, b| {
        if a.in_use != b.in_use {
            return b.in_use.cmp(&a.in_use); // Connected first
        }
        b.signal.cmp(&a.signal) // Then stronger signal
    });
    networks.dedup_by(|a, b| a.ssid == b.ssid);

    info!("Total networks after deduplication: {}", networks.len());

    Ok(networks)
}

#[cfg(target_os = "linux")]
async fn scan_networks() -> anyhow::Result<Vec<WifiNetwork>> {
    info!("Starting WiFi scan (Linux)...");
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

    info!("Executing command: nmcli {:?}", args);
    let mut output = Command::new("nmcli").args(args).output().await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        warn!(
            "Scan failed (might be disabled): {}. Attempting to enable WiFi...",
            stderr
        );

        // Try to enable wifi
        info!("Executing: nmcli radio wifi on");
        let _ = Command::new("nmcli")
            .args(&["radio", "wifi", "on"])
            .output()
            .await;

        // Try to unblock rfkill
        info!("Executing: rfkill unblock wifi");
        let _ = Command::new("rfkill")
            .args(&["unblock", "wifi"])
            .output()
            .await;

        // Wait a bit for interface to come up
        info!("Waiting 3s for interface to come up...");
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;

        // Retry scan
        info!("Retrying scan: nmcli {:?}", args);
        output = Command::new("nmcli").args(args).output().await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("nmcli failed after retry: {}", stderr);
            return Err(anyhow::anyhow!("nmcli failed: {}", stderr));
        }
    }

    info!("Scan command successful, parsing output...");
    parse_scan_output(output.stdout)
}

#[cfg(not(target_os = "linux"))]
async fn scan_networks() -> anyhow::Result<Vec<WifiNetwork>> {
    info!("Handling scan request on non-linux OS");
    debug!("Non-linux platform detected in scan_networks");
    warn!("WiFi scanning requested but not supported on this OS (non-linux).");
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
    // Step 1: Find the connected wifi device
    // nmcli -t -f DEVICE,TYPE,STATE,CONNECTION device status
    let output = Command::new("nmcli")
        .args(&[
            "-t",
            "-f",
            "DEVICE,TYPE,STATE,CONNECTION",
            "device",
            "status",
        ])
        .output()
        .await?;

    let stdout = String::from_utf8(output.stdout)?;

    let mut active_dev = None;
    let mut active_ssid = None;

    for line in stdout.lines() {
        // Format: wlan0:wifi:connected:MySSID
        // Note: Connection name might contain colons, escaped.
        // But for status, it's usually the connection ID.
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() >= 4 {
            let dev = parts[0];
            let type_ = parts[1];
            let state = parts[2];
            let connection = parts[3];

            if type_ == "wifi" && state == "connected" {
                active_dev = Some(dev.to_string());
                active_ssid = Some(connection.to_string());
                break;
            }
        }
    }

    if let Some(dev) = active_dev {
        // Step 2: Get IP address for the device
        // nmcli -t -f IP4.ADDRESS device show <dev>
        let ip_output = Command::new("nmcli")
            .args(&["-t", "-f", "IP4.ADDRESS", "device", "show", &dev])
            .output()
            .await?;

        let ip_stdout = String::from_utf8(ip_output.stdout)?;
        // Output format: IP4.ADDRESS:192.168.1.100/24
        // Or multiple lines. We take the first non-empty one.

        let mut ip = None;
        for line in ip_stdout.lines() {
            if let Some(val) = line.strip_prefix("IP4.ADDRESS:") {
                if !val.is_empty() {
                    ip = Some(val.to_string());
                    break;
                }
            } else if !line.is_empty() {
                // Sometimes it might just be the value if field selection behavior varies?
                // But standard behavior with -f KEY is KEY:VALUE in show mode (terse).
                // Let's try to parse flexibly.
                if line.contains(':') {
                    let parts: Vec<&str> = line.splitn(2, ':').collect();
                    if parts.len() == 2 && !parts[1].is_empty() {
                        ip = Some(parts[1].to_string());
                        break;
                    }
                } else {
                    // Just value?
                    ip = Some(line.to_string());
                    break;
                }
            }
        }

        return Ok(WifiStatus {
            connected: true,
            ssid: active_ssid,
            ip,
        });
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
