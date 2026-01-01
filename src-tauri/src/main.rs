// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod commands;
use crate::commands::{
    fs::{directory, disk},
    search::trigram::{self, generate_config, generate_index},
};

use tauri::ipc::Invoke;
fn all_commands() -> fn(Invoke) -> bool {
    tauri::generate_handler![
        disk::load_disk,
        directory::load_file,
        directory::open_file,
        trigram::generate_index,
        trigram::search,
        trigram::generate_config
    ]
}
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Example: Spawn an async task, manage state, etc.
            tauri::async_runtime::spawn(async move {
                generate_config();
                generate_index();
            });
            Ok(())
        })
        .invoke_handler(all_commands())
        .run(tauri::generate_context!())
        .expect("Error on running tauri");
    // file_manager_lib::run()
}
