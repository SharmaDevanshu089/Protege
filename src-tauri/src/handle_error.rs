use tauri::AppHandle;
// This is created to handle errors in my rust program and specifically to popup errors,
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_dialog::MessageDialogKind;

pub fn create_dialog(app: tauri::AppHandle, message: &str) -> () {
    let diag = app
        .dialog()
        .message(message)
        .kind(MessageDialogKind::Error)
        .title("Error")
        .blocking_show();
}
