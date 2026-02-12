mod config;
mod service_entry;

use config::AppConfig;

#[tauri::command]
fn get_app_config() -> AppConfig {
    config::load_config()
}

#[tauri::command]
fn save_app_config(config: AppConfig) -> Result<(), String> {
    config::save_config(&config)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_app_config, save_app_config])
        .setup(|app| {
            app.handle().plugin(
                tauri_plugin_log::Builder::default()
                    .targets([
                        tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout),
                        tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir {
                            file_name: None,
                        }),
                        tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Webview),
                    ])
                    .level(log::LevelFilter::Info)
                    .build(),
            )?;

            // Start Service
            tauri::async_runtime::spawn(async move {
                if let Err(e) = service_entry::run().await {
                    eprintln!("Service error: {:?}", e);
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
