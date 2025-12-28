// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod commands;
use crate::commands::fs::{directory, disk};
use tauri::ipc::Invoke;
fn all_commands() -> fn(Invoke) -> bool {
    tauri::generate_handler![disk::load_disk, directory::load_file]
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(all_commands())
        .run(tauri::generate_context!())
        .expect({
            let error_msg = "error while running tauri application";
            // log_critical!(error_msg);
            &error_msg.to_string()
        });
    // file_manager_lib::run()
}
