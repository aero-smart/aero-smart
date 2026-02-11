mod config;
mod service_entry;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Start Service
            tauri::async_runtime::spawn(async move {
                if let Err(e) = service_entry::run().await {
                    eprintln!("Service error: {:?}", e);
                }
            });

            app.handle().plugin(
                tauri_plugin_log::Builder::default()
                    .targets([
                        tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout),
                        tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir { file_name: None }),
                        tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Webview),
                    ])
                    .level(log::LevelFilter::Info)
                    .build(),
            )?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
