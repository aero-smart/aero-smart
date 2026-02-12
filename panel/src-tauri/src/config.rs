use log::info;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig {
    pub serial: SerialConfig,
    pub server: ServerConfig,
    #[serde(default)]
    pub rules: RulesConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SerialConfig {
    pub port: String,
    pub baud_rate: u32,
    pub handshake_timeout_secs: u64,
    pub retry_interval_secs: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RulesConfig {
    #[serde(default)]
    pub debug_mode: bool,
    #[serde(default = "default_true")]
    pub enable_onboarding: bool,
}

impl Default for RulesConfig {
    fn default() -> Self {
        Self {
            debug_mode: false,
            enable_onboarding: true,
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        #[cfg(target_os = "linux")]
        let default_port = "/dev/ttyUSB0".to_string();
        #[cfg(target_os = "windows")]
        let default_port = "COM3".to_string();
        #[cfg(not(any(target_os = "linux", target_os = "windows")))]
        let default_port = "/dev/tty.usbmodem1234".to_string();

        Self {
            serial: SerialConfig {
                port: default_port,
                baud_rate: 915200,
                handshake_timeout_secs: 2,
                retry_interval_secs: 5,
            },
            server: ServerConfig {
                port: 3000,
                host: "0.0.0.0".to_string(),
            },
            rules: RulesConfig::default(),
        }
    }
}

fn get_config_path() -> PathBuf {
    if cfg!(target_os = "linux") {
        if let Ok(home) = std::env::var("HOME") {
            return PathBuf::from(home).join(".AeroSmart.toml");
        }
    }
    PathBuf::from("AeroSmart.toml")
}

pub fn load_config() -> AppConfig {
    let config_path = get_config_path();

    if config_path.exists() {
        info!("Loading configuration from {:?}", config_path);
        match fs::read_to_string(&config_path) {
            Ok(contents) => match toml::from_str(&contents) {
                Ok(config) => {
                    // Normalize config file by writing back (fills in default values for missing fields)
                    if let Err(e) = save_config(&config) {
                        info!("Failed to normalize config file: {}", e);
                    }
                    return config;
                }
                Err(e) => info!("Failed to parse config file: {}. Using defaults.", e),
            },
            Err(e) => info!("Failed to read config file: {}. Using defaults.", e),
        }
    } else {
        info!("Config file {:?} not found. Creating default.", config_path);
        let default_config = AppConfig::default();
        match toml::to_string_pretty(&default_config) {
            Ok(toml_str) => {
                // Add comments to the TOML string manually for better UX since toml crate doesn't support preserving comments well on serialization
                // Or better, just write a predefined string with comments.
                // But for now, let's write what we can.
                if let Err(e) = fs::write(&config_path, toml_str) {
                    info!("Failed to write default config file: {}", e);
                }
            }
            Err(e) => info!("Failed to serialize default config: {}", e),
        }
        return default_config;
    }

    AppConfig::default()
}

pub fn save_config(config: &AppConfig) -> Result<(), String> {
    let config_path = get_config_path();
    let toml_str =
        toml::to_string_pretty(config).map_err(|e| format!("Failed to serialize config: {}", e))?;

    fs::write(&config_path, toml_str).map_err(|e| format!("Failed to write config file: {}", e))?;

    info!("Configuration saved to {:?}", config_path);
    Ok(())
}

pub fn create_default_config_file_if_missing() {
    let config_path = get_config_path();
    if !config_path.exists() {
        #[cfg(target_os = "linux")]
        let default_port = "/dev/ttyUSB0";
        #[cfg(target_os = "windows")]
        let default_port = "COM3";
        #[cfg(not(any(target_os = "linux", target_os = "windows")))]
        let default_port = "/dev/tty.usbmodem1234";

        let content = format!(
            r#"# AeroSmart Service Configuration File
# Restart the application after modifying this file to apply changes.

# [serial] Serial communication configuration
[serial]
# Serial port name (e.g., /dev/tty.usbmodem1234, COM3, /dev/ttyUSB0)
port = "{}"

# Baud rate (default: 915200)
baud_rate = 915200

# Handshake timeout (seconds)
# Time to wait for the lower machine to respond to Ping during startup
handshake_timeout_secs = 2

# Retry interval on failure (seconds)
# How long to wait before retrying when the serial connection is disconnected or initialization fails
retry_interval_secs = 5

# [server] WebSocket service configuration
[server]
# Listening port
port = 3000

# Listening address (0.0.0.0 allows LAN access, 127.0.0.1 is localhost only)
host = "0.0.0.0"

# [rules] Advanced rules configuration
[rules]
# Enable debug mode (outputs more logs)
debug_mode = false

# Enable onboarding screen on startup
enable_onboarding = true
"#,
            default_port
        );
        if let Err(e) = fs::write(&config_path, content) {
            info!("Failed to create default config file: {}", e);
        } else {
            info!("Created default configuration file: {:?}", config_path);
        }
    }
}
