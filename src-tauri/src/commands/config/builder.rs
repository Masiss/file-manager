use anyhow::Result;
use dirs::{config_dir, download_dir, home_dir};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};
use toml::{self};

use crate::commands::{config, error::Error};
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub except_list: Vec<String>,
    pub quick_access: Vec<String>,
}
pub fn get_config_path(filename: &str) -> PathBuf {
    config_dir()
        .expect("Cant find config dir")
        .join("file-manager")
        .join(filename)
}
#[tauri::command]
pub fn get_quick_access() -> Result<Vec<String>, Error> {
    let config_path = get_config_path("config.toml");
    let config_file = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config_file).unwrap_or_default();
    Ok(config.quick_access)
}
#[tauri::command]
pub fn generate_config() {
    let config_file = "config.toml";
    let config_path = get_config_path(config_file);
    if !config_path.exists()
        && let Some(parent) = config_path.parent()
    {
        fs::create_dir_all(parent).expect("GENCONFIG :: cant create parent folder");
    }
    let mut file = File::create(&config_path).expect("GENCONFIG :: Không thể tạo file config.toml");
    let except_list = vec![
        // Windows
        "$Recycle.Bin",
        "System Volume Information",
        "Recovery",
        "PerfLogs",
        "AppData", // thường chứa cache lớn
        "ProgramData",
        "Windows",
        "Program Files",
        "Program Files (x86)",
        // Linux
        "/proc",
        "/sys",
        "/dev",
        "/run",
        "/tmp",
        "/var/run",
        "/var/lock",
        "packages",
        "/var/cache",
        "/var/log",
        "/var/spool",
        "/var/tmp",
        // macOS
        "/System",
        "/Library",
        "/Applications",
        "/private/var",
        "/private/tmp",
        "/Volumes",
        "/cores",
        "/.Spotlight-V100",
        "/.fseventsd",
        // Common large / noisy folders (cross-platform)
        "node_modules",
        ".git",
        ".svn",
        ".hg",
        "__pycache__",
        "Common",
        ".venv",
        "venv",
        ".cache",
        ".gradle",
        ".idea",
        ".vscode",
        "target", // Rust cargo
        "build",
        "dist",
        "out",
        ".next", // Next.js
        ".nuxt",
        ".yarn",
        "vendor", // PHP/Ruby
        "bower_components",
        ".terraform",
        ".docker",
        "Cache",
        "Caches",
        "Logs",
    ];
    let home_dir = home_dir().unwrap().display().to_string();
    let download_dir = download_dir().unwrap().display().to_string();
    let quick_access_list = vec![home_dir, download_dir];
    let config: String = toml::to_string(&Config {
        except_list: except_list.iter().map(|s| s.to_string()).collect(),
        quick_access: quick_access_list,
    })
    .expect("GEN CONFIG :: CANT CONVERT TO TOML");
    file.write_all(config.as_bytes()).unwrap()
}
