use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};
use toml::{self};
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub except_list: Vec<String>,
}
pub fn get_config_path(filename: &str) -> PathBuf {
    config_dir()
        .expect("Cant find config dir")
        .join("file_manager")
        .join(filename)
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
    let config: String = toml::to_string(&Config {
        except_list: except_list.iter().map(|s| s.to_string()).collect(),
    })
    .expect("GEN CONFIG :: CANT CONVERT TO TOML");
    file.write_all(config.as_bytes()).unwrap()
}
