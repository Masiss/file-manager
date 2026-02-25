use std::{fs, io::Result, path::Path};

use crate::commands::fs::directory::{self, File};
#[tauri::command]
pub fn check_path(path: String) -> bool {
    let path_test = Path::new(&path);
    if path_test.exists() {
        return true;
    }
    false
}
