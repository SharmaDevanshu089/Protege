// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod handle_error;

use tauri::Manager;
use window_vibrancy::apply_mica;
use windows_registry::*;
fn main() {
    protege_lib::run();
}
