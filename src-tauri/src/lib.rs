// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use tauri::Manager;
use window_vibrancy::apply_mica;
mod handle_error;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, TrayIconBuilder, TrayIconEvent};
// use tauri::Manager; // Required for .app_handle() and .exit()
use tauri::image::Image;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            #[cfg(target_os = "windows")]
            {
                //TODO: Add a fallback for unsupported platforms
                let _ = apply_mica(&window, None).expect("Unsupported Platform");
                //TODO :Add option for mica on unsupported platform
            }

            // let handle = app.handle().clone();
            let icon_bytes = include_bytes!("../icons/icon.png");
            let tray_image = Image::from_bytes(icon_bytes).expect("Failed to parse PNG");

            // ---------------------------------------------------------
            // 2. CREATE THE MENU ITEMS
            // ---------------------------------------------------------
            let quit_i = MenuItem::with_id(app, "quit", "Quit App", true, None::<&str>)?;
            let show_i = MenuItem::with_id(app, "show", "Show Window", true, None::<&str>)?;

            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

            // ---------------------------------------------------------
            // 3. BUILD THE TRAY
            // ---------------------------------------------------------
            let _tray = TrayIconBuilder::new()
                .icon(tray_image) // Use the PNG we loaded above
                .menu(&menu)
                .tooltip("My App Name")
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        println!("Quit clicked. Bye!");
                        app.exit(0);
                    }
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    // Optional: Click the icon itself to show window
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
