use anyhow::Result;
use dirs::{config_dir, download_dir, home_dir};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
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
static CONFIG_FILE: &str = "config.toml";
pub fn get_config() -> anyhow::Result<Config> {
    let config_path = get_config_path(CONFIG_FILE);
    let config_file = fs::read_to_string(config_path)?;

    let config: Config = toml::from_str(&config_file).unwrap_or_default();
    Ok(config)
}
#[tauri::command]
pub fn get_quick_access() -> Result<Vec<String>, String> {
    let config = get_config().map_err(|e| format!("Error on getting config : {}", e))?;
    Ok(config.quick_access)
}
#[tauri::command]
pub fn remove_quick_access(path: String) -> Result<String, String> {
    if Path::new(&path).exists() {
        let mut config = get_config().map_err(|e| format!("Error on getting config : {}", e))?;
        config.quick_access.retain(|x| x != &path);
        let _ = fs::write(
            get_config_path("config.toml"),
            toml::to_string(&config)
                .map_err(|e| format!("Error while writing new config: {}", e))?,
        );
        return Ok(format!("Remove {} from quick access!", &path));
    }
    Err(format!("Cant find path {}", &path))
}
#[tauri::command]
pub fn add_quick_access(new_path: String) -> Result<String, String> {
    if Path::new(&new_path).exists() {
        let mut config = get_config().map_err(|e| format!("Error on getting config : {}", e))?;
        if let Some(found) = config.quick_access.iter().find(|&x| x == &new_path) {
            return Err(format!("{} existed in quick access", &new_path));
        }
        config.quick_access.push(new_path.clone());
        let _ = fs::write(
            get_config_path("config.toml"),
            toml::to_string(&config)
                .map_err(|e| format!("Error while adding quick access in config.toml: {}", e))?,
        );
        return Ok(format!("Added {} to quick access!", &new_path));
    }
    Err(format!("Cant find path {}", &new_path))
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
