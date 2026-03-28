use crate::commands::error::Error;
use crate::commands::fs::disk::{get_disk, load_disk};
use anyhow::Result;
use chrono::{DateTime, Utc};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs::{self, DirEntry, Metadata};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct File {
    name: String,
    path: String,
    created_at: String,
    pub last_modified: String,
    size: u64,
    file_type: FileType,
    accessed: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
enum FileType {
    Directory,
    File,
    SymbolicLink,
}
impl From<std::fs::FileType> for FileType {
    fn from(filetype: fs::FileType) -> Self {
        if filetype.is_dir() {
            FileType::Directory
        } else if filetype.is_file() {
            FileType::File
        } else {
            FileType::SymbolicLink
        }
    }
}
fn format_systemtime(time: SystemTime) -> String {
    let datetime: DateTime<Utc> = time.into();
    datetime.format("%d/%m/%Y %H:%M:%S").to_string()
}

#[tauri::command]
pub fn load_file(current_path: String) -> String {
    let path = Path::new(&current_path);
    let mut file_list: Vec<File> = Vec::new();
    if let Ok(metadata) = fs::metadata(path)
        && metadata.is_dir()
        && let Ok(entries) = fs::read_dir(&current_path)
    {
        for entry in entries.filter_map(Result::ok) {
            if let Ok(metadata) = entry.metadata() {
                file_list.push(File {
                    name: String::from(entry.file_name().to_string_lossy()),
                    path: String::from(entry.path().to_string_lossy()),
                    created_at: format_systemtime(metadata.created().unwrap()),
                    size: metadata.len(),
                    last_modified: format_systemtime(metadata.modified().unwrap()),
                    file_type: FileType::from(metadata.file_type()),
                    accessed: format_systemtime(metadata.accessed().unwrap()),
                })
            }
        }
    }
    serde_json::to_string(&file_list).unwrap()
}
#[tauri::command]
pub fn load_path(current_path: String) -> String {
    let path = Path::new(&current_path);

    // let metadata = match fs::metadata(path) {
    //     Ok(m) if m.is_dir() => m,
    //     _ => return serde_json::to_string(&"").unwrap(),
    // };
    let entries = match fs::read_dir(&current_path) {
        Ok(entries) => entries,
        _ => return serde_json::to_string(&"").unwrap(),
    };
    let mut entries: Vec<DirEntry> = entries.filter_map(Result::ok).collect();

    entries.sort_unstable_by(|a, b| {
        let a_is_dir = a.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
        let b_is_dir = b.file_type().map(|ft| ft.is_dir()).unwrap_or(false);

        match (b_is_dir, a_is_dir) {
            (true, false) => std::cmp::Ordering::Greater,
            (false, true) => std::cmp::Ordering::Less,
            _ => a
                .file_name()
                .to_ascii_lowercase()
                .cmp(&b.file_name().to_ascii_lowercase()),
        }
    });
    let path_list: Vec<String> = entries
        .into_par_iter()
        .map(|entry| entry.path().to_string_lossy().into_owned())
        .collect();
    serde_json::to_string(&path_list).unwrap()
}
#[tauri::command]
pub fn load_metadata(path_list: Vec<String>) -> Result<String, Error> {
    let paths: Vec<PathBuf> = path_list.into_par_iter().map(PathBuf::from).collect();
    let result: Vec<File> = paths
        .into_par_iter()
        .filter_map(|path| create_file_from_path(path.as_path()).ok())
        .collect();
    let encoded = serde_json::to_string(&result)?;
    Ok(encoded)
}
fn create_file_from_entry(entry: DirEntry) -> Result<File> {
    let metadata = entry.metadata()?;
    let file = File {
        name: String::from(entry.file_name().to_string_lossy()),
        path: String::from(entry.path().to_string_lossy()),
        created_at: format_systemtime(metadata.created()?),
        size: metadata.len(),
        last_modified: format_systemtime(metadata.modified()?),
        file_type: FileType::from(metadata.file_type()),
        accessed: format_systemtime(metadata.accessed()?),
    };
    Ok(file)
}
pub fn create_file_from_path(path: &Path) -> Result<File> {
    let metadata: Metadata = path.metadata()?;
    let file = File {
        name: path.file_name().unwrap().to_string_lossy().into_owned(),
        path: path.to_string_lossy().into_owned(),
        created_at: format_systemtime(metadata.created()?),
        size: metadata.len(),
        last_modified: format_systemtime(metadata.modified()?),
        file_type: FileType::from(metadata.file_type()),
        accessed: format_systemtime(metadata.accessed()?),
    };
    Ok(file)
}
#[tauri::command]
pub fn open_file(path: String) -> Result<(), Error> {
    let buf: PathBuf = PathBuf::from(path);
    open::that_detached(buf)?;
    Ok(())
}
