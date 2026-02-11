use std::collections::HashMap;

use dbus::arg::Variant;
use networkmanager::types::DeviceType;
use networkmanager::{
    devices::{Any, Device, Wireless},
    NetworkManager,
};

pub async fn connect_wifi(ssid: &str, password: &str) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "linux")]
    {
        Command::new("nmcli")
        .args(&["device", "wifi", "connect", ssid, "password", password])
        .output()?;
    };
    #[cfg(not(target_os = "linux"))]
    {
        use tracing::{info, error};

        info!("Connecting to WiFi SSID: {}, Password: {}", ssid, password);
        error!("WiFi connection is only supported on Linux.");
    };
    Ok(())
}
