use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tracing::info;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig {
    pub serial: SerialConfig,
    pub server: ServerConfig,
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RulesConfig {
    pub debug_mode: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            serial: SerialConfig {
                port: "/dev/tty.usbmodem1234".to_string(),
                baud_rate: 915200,
                handshake_timeout_secs: 2,
                retry_interval_secs: 5,
            },
            server: ServerConfig {
                port: 3000,
                host: "0.0.0.0".to_string(),
            },
            rules: RulesConfig { debug_mode: false },
        }
    }
}

pub fn load_config() -> AppConfig {
    let config_path = "AeroSmart.toml";

    if Path::new(config_path).exists() {
        info!("Loading configuration from {}", config_path);
        match fs::read_to_string(config_path) {
            Ok(contents) => match toml::from_str(&contents) {
                Ok(config) => return config,
                Err(e) => info!("Failed to parse config file: {}. Using defaults.", e),
            },
            Err(e) => info!("Failed to read config file: {}. Using defaults.", e),
        }
    } else {
        info!("Config file {} not found. Creating default.", config_path);
        let default_config = AppConfig::default();
        match toml::to_string_pretty(&default_config) {
            Ok(toml_str) => {
                // Add comments to the TOML string manually for better UX since toml crate doesn't support preserving comments well on serialization
                // Or better, just write a predefined string with comments.
                // But for now, let's write what we can.
                if let Err(e) = fs::write(config_path, toml_str) {
                    info!("Failed to write default config file: {}", e);
                }
            }
            Err(e) => info!("Failed to serialize default config: {}", e),
        }
        return default_config;
    }

    AppConfig::default()
}

pub fn create_default_config_file_if_missing() {
    let config_path = "AeroSmart.toml";
    if !Path::new(config_path).exists() {
        let content = r#"# AeroSmart Service 配置文件
# 修改此文件后需要重启服务生效

# [serial] 串口通信配置
[serial]
# 串口端口名称 (例如: /dev/tty.usbmodem1234, COM3, /dev/ttyUSB0)
port = "/dev/tty.usbmodem1234"

# 波特率 (默认: 915200)
baud_rate = 915200

# 握手超时时间 (秒)
# 在启动时等待下位机响应 Ping 的时间
handshake_timeout_secs = 2

# 失败重试间隔 (秒)
# 当串口连接断开或初始化失败时，等待多久后重试
retry_interval_secs = 5

# [server] WebSocket 服务配置
[server]
# 监听端口
port = 3000

# 监听地址 (0.0.0.0 表示允许局域网访问，127.0.0.1 仅限本机)
host = "0.0.0.0"

# [rules] 高级规则配置
[rules]
# 是否开启调试模式 (会输出更多日志)
debug_mode = false
"#;
        if let Err(e) = fs::write(config_path, content) {
            info!("Failed to create default config file: {}", e);
        } else {
            info!("Created default configuration file: {}", config_path);
        }
    }
}
