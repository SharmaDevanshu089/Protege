// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use tauri::Manager;
use window_vibrancy::apply_mica;
mod handle_error;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let handle = app.handle().clone();
            let window = app.get_webview_window("main").unwrap();
            #[cfg(target_os = "windows")]
            {
                //TODO: Add a fallback for unsupported platforms
                let _ = apply_mica(&window, None).expect("Unsupported Platform");
                //TODO :Add option for mica on unsupported platform
            }
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
