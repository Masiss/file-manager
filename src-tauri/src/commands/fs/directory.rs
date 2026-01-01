use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::{self, Metadata};
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
pub fn create_file_from_path(path: &Path, metadata: Metadata) -> File {
    File {
        name: path.file_name().unwrap().to_string_lossy().into_owned(),
        path: path.to_string_lossy().into_owned(),
        created_at: format_systemtime(metadata.created().unwrap()),
        size: metadata.len(),
        last_modified: format_systemtime(metadata.modified().unwrap()),
        file_type: FileType::from(metadata.file_type()),
        accessed: format_systemtime(metadata.accessed().unwrap()),
    }
}
#[tauri::command]
pub fn open_file(path: String) {
    let buf: PathBuf = PathBuf::from(path);
    open::that_detached(buf);
}
