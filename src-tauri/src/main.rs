// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod commands;
use crate::commands::{
    config::builder::{generate_config, get_quick_access},
    fs::{directory, disk},
    search::trigram::{builder::generate_index, search::search},
    utils::check_path,
};

use tauri::ipc::Invoke;
fn all_commands() -> fn(Invoke) -> bool {
    tauri::generate_handler![
        disk::load_disk,
        directory::load_file,
        directory::open_file,
        directory::load_metadata,
        directory::load_path,
        get_quick_access,
        generate_index,
        search,
        generate_config,
        check_path
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
