// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use tauri::Manager;
use window_vibrancy::apply_mica;
mod handle_error;
mod resume;

use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, TrayIconBuilder, TrayIconEvent};
// use tauri::Manager; // Required for .app_handle() and .exit()
use tauri::image::Image;

// Commands that will be exposed via the invoke_handler must be defined in the same
// crate/module where the handler macro is called. Previously this command lived in
// `main.rs`, which is a different binary crate – the library build could not see it,
// leading to the `cannot find macro __cmd__select_vault_folder` error.  Move it here
// (or import it) so the macro can generate the command registration correctly.

#[tauri::command]
async fn select_vault_folder(app_handle: tauri::AppHandle) -> String {
    loop {
        // 1. Open the dialog
        let folder = rfd::FileDialog::new()
            .set_title("Select your Lockr Vault Folder")
            .pick_folder();

        // 2. Handle the choice
        match folder {
            Some(path) => {
                // User picked a folder!
                let path_str = path.to_string_lossy().to_string();
                
                // Validate if it has protege.toml
                match resume::load_project_env(path_str.clone()) {
                    Ok(_) => return path_str,
                    Err(e) => {
                        // If error, show dialog and continue the loop
                        handle_error::create_dialog(app_handle.clone(), &e);
                        continue;
                    }
                }
            }
            None => {
                // User clicked Cancel.
                // The loop restarts, instantly opening the dialog again.
                continue;
            }
        }
    }
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
        .invoke_handler(tauri::generate_handler![
            select_vault_folder,
            resume::load_project_env,
            resume::resume_last_project
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
