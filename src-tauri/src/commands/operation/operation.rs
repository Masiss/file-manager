use crate::commands::fs::disk::get_disk;
use crate::commands::operation::archive_extract::{
    create_sevenzip, create_tar, create_zip, decompress, decompress_7z,
};

use anyhow::anyhow;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use trash::Error::RestoreCollision;
use trash::TrashContext;
use trash::os_limited::{list, restore_all};

use std::collections::HashMap;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;
use tauri::{App, Emitter};
use tauri::{AppHandle, State};
use tokio::fs::{self, File, try_exists};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::select;
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;
use unicode_normalization::UnicodeNormalization;
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum CopyResult {
    #[serde(rename = "ok")]
    Ok,
    #[serde(rename = "conflict")]
    Conflict { file_list: Vec<String> },
}
#[tauri::command]
pub async fn check_exist(source_list: Vec<String>, dest_dir: String) -> Result<CopyResult, String> {
    let mut conflicts: Vec<String> = vec![];
    for source_path in &source_list {
        let file_name = Path::new(source_path)
            .file_name()
            .and_then(|f| f.to_str())
            .ok_or_else(|| format!("can not get filename from: {source_path}"))?;
        println!("{:?}", file_name);
        let dest_path = Path::new(&dest_dir).join(file_name);

        println!("{:?}", dest_path);
        match try_exists(&dest_path).await {
            Ok(true) => conflicts.push(dest_path.to_string_lossy().to_string()),
            Ok(false) => {}
            Err(e) => return Err(format!("Error checking file existence: {e}")),
        }
    }
    println!("{:?}", conflicts);
    if !conflicts.is_empty() {
        Ok(CopyResult::Conflict {
            file_list: conflicts,
        })
    } else {
        Ok(CopyResult::Ok)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TaskProgress {
    pub task_id: String,
    pub value: u64,
    pub total: u64,
    pub done: bool,
}

#[derive(Default)]
pub struct AppState {
    pub task_list: Mutex<HashMap<String, CancellationToken>>,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct TaskInfo {
    pub src_list: Vec<String>,
    pub dest_dir: String,
    pub task_id: String,
}
enum TaskType {
    Copy {
        task_info: TaskInfo,
    },
    Cut {
        task_info: TaskInfo,
    },
    Archive {
        task_info: TaskInfo,
        format: String,
        password: Option<String>,
    },
    Extract {
        task_info: TaskInfo,
        password: Option<String>,
    },
}
#[tauri::command]
pub async fn cancel(task_id: String, state: State<'_, AppState>) -> Result<(), String> {
    let task_list = state.task_list.lock().await;
    if let Some(token) = task_list.get(&task_id) {
        token.cancel();
    }
    Ok(())
}

#[tauri::command]
pub async fn create_file(directory: String, file_name: Option<String>) -> Result<String, String> {
    let dir_path = Path::new(&directory);

    if !dir_path.exists() {
        return Err(format!("Cant find parent of {}", &directory));
    }
    let file_path = if let Some(name) = file_name {
        let path = dir_path.join(&name);
        if path.exists() {
            return Err(format!("File name {} has been existed!", &name));
        }
        path
    } else {
        let name = "New file.txt";
        let mut path = dir_path.join(name);
        if path.exists() {
            let mut counter = 1;
            loop {
                path = dir_path.join(format!("New file {}.txt", &counter));
                if !path.exists() {
                    break;
                }
                counter += 1;
            }
        }
        path
    };
    tokio::fs::File::create_new(&file_path)
        .await
        .map_err(|e| format!("Error while creating file : {}", e))?;

    Ok(file_path.to_string_lossy().to_string())
}
#[tauri::command]
pub async fn create_dir(dir_path: String) -> Result<String, String> {
    let path = Path::new(&dir_path);
    if path.parent().is_none() {
        return Err(format!("Cant find parent of {}", &dir_path));
    }
    tokio::fs::create_dir(path)
        .await
        .map_err(|e| format!("Error on creating directory {}: {}", &dir_path, e))?;
    Ok(format!("Created directory {}!", &dir_path))
}
#[tauri::command]
pub async fn delete_file(
    task_id: String,
    path_list: Vec<String>,
    permanent: bool,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let token = generate_cancellation_token(&task_id, state)
        .await
        .map_err(|e| format!("Error on generate cancellation token : {}", e))?;
    let mut deleted_filename: Vec<OsString> = vec![];
    for (index, path_str) in path_list.iter().enumerate() {
        if token.is_cancelled() {
            if !permanent {
                let items = list()
                    .unwrap()
                    .into_iter()
                    .filter(|x| deleted_filename.contains(&x.name));
                if let Err(RestoreCollision {
                    path,
                    mut remaining_items,
                }) = restore_all(items)
                {
                    // keep all except the one(s) that couldn't be restored
                    remaining_items.retain(|e| e.original_path() != path);
                    restore_all(remaining_items).unwrap();
                }
            }
            return Err("Cancelled".to_string());
        }
        let path = Path::new(&path_str);
        if !path.exists() {
            return Err(format!("File {} is not exist!", &path_str));
        }
        deleted_filename.push(path.file_name().unwrap().to_os_string());

        if permanent {
            fs::remove_file(&path_str)
                .await
                .map_err(|e| format!("Failed to delete file {} : {}", &path_str, e))?;
        } else {
            trash::delete(path_str)
                .map_err(|e| format!("Failed to delete file {}: {}", &path_str, e))?;
        }
        app.emit(
            "task-progressing",
            TaskProgress {
                task_id: task_id.to_string(),
                value: (index + 1) as u64,
                total: path_list.len() as u64,
                done: false,
            },
        )
        .unwrap();
    }
    Ok(())
}

pub fn get_sid() -> Result<String, String> {
    let output = Command::new("whoami")
        .args(["/user", "/fo", "csv", "/nh"])
        .output()
        .ok()
        .unwrap();
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = stdout.trim().split(",").collect();
        let sid = parts[1].trim().replace('"', "").to_string();
        return Ok(sid);
    }
    Err(format!("sid not found : {}", output.status))
}
pub fn filetime_to_datetime(filetime: u64) -> String {
    let unix_ns = filetime.saturating_sub(116_444_736_000_000_000);
    let unix_sec = (unix_ns / 10_000_000) as i64;
    let unix_time = DateTime::from_timestamp(unix_sec, 0).unwrap();
    unix_time
        .with_timezone(&Local)
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrashFile {
    name: String,
    path: String,
    deletation_time: String,
    size: i64,
    content_file: String,
    original_path: String,
}
pub fn parse_i_trask_file(file: &str) -> anyhow::Result<TrashFile> {
    match std::fs::read(file) {
        Ok(content) => {
            if content.len() < 0x18 {
                return Err(anyhow!("File too small: {} bytes", content.len()));
            }
            let file_size = u64::from_le_bytes(content[0x08..0x10].try_into()?);
            let delete_time = u64::from_le_bytes(content[0x10..0x18].try_into()?);
            let dt = filetime_to_datetime(delete_time);

            let file_name_bytes: Vec<u16> = content[0x1C..]
                .chunks_exact(2)
                .map(|c| u16::from_le_bytes([c[0], c[1]]))
                .take_while(|&c| c != 0) // strip null terminator
                .collect();

            let file_path = String::from_utf16(&file_name_bytes)?
                .to_string()
                .nfc()
                .collect::<String>();

            let path = Path::new(&file);
            let parent = path.parent().unwrap();
            let file_name = Path::new(&file_path)
                .file_name()
                .unwrap_or(path.file_name().unwrap())
                .to_string_lossy()
                .to_string();

            let content_file_name = parent.join(
                path.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .replace("$I", "$R"),
            );
            let trash_item = TrashFile {
                name: file_name,
                deletation_time: dt,
                size: file_size as i64,
                path: file.to_string(),
                content_file: content_file_name.to_string_lossy().to_string(),
                original_path: file_path,
            };

            Ok(trash_item)
        }
        Err(e) => Err(anyhow!("Error on parsing file {} : {}", &file, e)),
    }
}

#[tauri::command]
pub async fn load_trash_metadata(path_list: Vec<String>) -> Result<Vec<TrashFile>, String> {
    let mut trash_list: Vec<TrashFile> = vec![];
    for file in path_list {
        match parse_i_trask_file(&file) {
            Ok(trash_item) => trash_list.push(trash_item),
            Err(e) => continue,
        }
    }

    Ok(trash_list)
}

#[tauri::command]
pub async fn get_trash_bin() -> Result<Vec<String>, String> {
    let mut items: Vec<String> = vec![];
    let mut disks: Vec<PathBuf> = vec![];
    let os = std::env::consts::OS;
    if os == "macos" {
        let trask_dir = "./Trash";
        disks.push(Path::new(trask_dir).to_path_buf());
    } else if os == "windows" {
        let recycle_bin = "$RECYCLE.BIN";
        let sid = get_sid().map_err(|e| format!("Error on getting sid : {}", e))?;
        disks = get_disk()
            .map_err(|e| format!("Error on getting disk : {}", e))?
            .into_iter()
            .map(|disk| Path::new(&disk.mount_point).join(recycle_bin).join(&sid))
            .collect();
    }
    for disk in disks {
        println!("{:?}", &disk);
        let mut trash_items: Vec<String> = WalkDir::new(&disk)
            .max_depth(1)
            .min_depth(1)
            .into_iter()
            .filter_map(|e| {
                let entry = e.ok()?;
                if entry
                    .file_name()
                    .to_string_lossy()
                    .to_string()
                    .starts_with("$I")
                {
                    Some(entry.path().to_string_lossy().to_string())
                } else {
                    None
                }
            })
            .collect();
        items.append(&mut trash_items);
    }
    Ok(items)
}

pub async fn generate_cancellation_token(
    task_id: &str,
    state: State<'_, AppState>,
) -> anyhow::Result<CancellationToken> {
    let token = CancellationToken::new();
    state
        .task_list
        .lock()
        .await
        .insert(task_id.to_string(), token.clone());
    Ok(token)
}
pub async fn copy_files(
    src_list: Vec<String>,
    dest_dir: String,
    task_id: String,
    app: AppHandle,
    state: State<'_, AppState>,
) -> anyhow::Result<Vec<PathBuf>> {
    let token = generate_cancellation_token(&task_id, state).await?;
    // .map_err(|e|format!("Err on generating cancellation token: {}",e))?;

    let dest = Path::new(&dest_dir);
    let mut copied_list = vec![];
    for path_str in src_list {
        //get folder items/file
        for entry in WalkDir::new(&path_str).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            let relative = path.file_name().and_then(|e| e.to_str()).unwrap();
            let dest_file = dest.join(relative);

            if let Some(parent) = dest_file.parent() {
                tokio::fs::create_dir_all(parent).await?;
            }

            let mut reader = File::open(path).await?;
            let mut writer = File::create(&dest_file).await?;
            let total = reader.metadata().await?.len();

            let mut buf = vec![0u8; 256 * 1024]; // 256KB chunks
            let mut copied = 0u64;
            let file_name = entry.file_name().to_string_lossy().to_string();
            //start copy
            copied_list.push(path.to_path_buf());
            loop {
                select! {
                    biased;

                  _ = token.cancelled() => {
                       for file in copied_list{

                           tokio::fs::remove_file(file).await?;
                       }
                       app.emit("task-close","").ok();
                       writer.flush().await?;
                       return Err(anyhow!("cancelled"))

                   }

                result = reader.read(&mut buf) =>{
                   let n = result?;
                   if n == 0 {
                       break;
                   }
                   writer
                       .write_all(&buf[..n])
                       .await?;
                   copied += n as u64;
                   println!("copying");
                   app.emit(
                       "task-progressing",
                       TaskProgress {
                       task_id: task_id.to_string(),
                           value:copied,
                           total,
                           done: false,
                       },
                   )
                   .ok();
                   }
                }
            }
            //copy done
            writer.flush().await?;
            app.emit(
                "task-progressing",
                TaskProgress {
                    task_id: task_id.clone(),
                    value: copied,
                    total,
                    done: true,
                },
            )
            .unwrap();
            println!("copy done!");
        }
    }

    Ok(copied_list)
}
#[tauri::command]
pub async fn copy(
    task_info: TaskInfo,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    copy_files(
        task_info.src_list,
        task_info.dest_dir,
        task_info.task_id,
        app,
        state,
    )
    .await
    .map_err(|e| format!("Copy file error: {e}"))?;
    Ok(())
}

#[tauri::command]
pub async fn cut(
    task_info: TaskInfo,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let copied_list: Vec<PathBuf> = copy_files(
        task_info.src_list,
        task_info.dest_dir,
        task_info.task_id,
        app,
        state,
    )
    .await
    .map_err(|e| format!("Copy file error: {e}"))?;
    println!("{:?}", copied_list);
    for file in copied_list {
        if file.is_dir() {
            fs::remove_dir_all(file)
                .await
                .map_err(|e| format!("Remove dir after copy error: {e}"))?;
        } else {
            fs::remove_file(file)
                .await
                .map_err(|e| format!("Remove file after copy error: {e}"))?;
        }
    }
    Ok(())
}
#[tauri::command]
pub async fn rename(source_str: String, new_name: String) -> Result<(), String> {
    let source_path = Path::new(&source_str);
    let new_path = &source_path
        .parent()
        .ok_or("Cant get parent of renaming file")?
        .join(new_name);
    tokio::fs::rename(source_path, new_path)
        .await
        .map_err(|e| format!("Rename file error: {e}"))?;
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArchiveFormat {
    Zip,
    Tar,
    SevenZ,
}
impl ArchiveFormat {
    pub fn from_path(path: &Path) -> Option<Self> {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("zip") => Some(ArchiveFormat::Zip),
            Some("tar") => Some(ArchiveFormat::Tar),
            Some("7z") => Some(ArchiveFormat::SevenZ),
            _ => None,
        }
    }
    pub fn from_string_to_format(format: &str) -> Option<Self> {
        match format {
            "7z" => Some(ArchiveFormat::SevenZ),
            "zip" => Some(ArchiveFormat::Zip),
            "tar" => Some(ArchiveFormat::Tar),
            _ => None,
        }
    }
}

#[tauri::command]
pub async fn archive(
    task_info: TaskInfo,
    password: Option<String>,
    format: String,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    match ArchiveFormat::from_string_to_format(&format) {
        Some(ArchiveFormat::Zip) => create_zip(
            task_info.src_list,
            task_info.dest_dir,
            task_info.task_id,
            app,
            state,
        )
        .await
        .map_err(|e| format!("Error on archiving zip : {}", e)),
        Some(ArchiveFormat::Tar) => create_tar(
            task_info.src_list,
            task_info.dest_dir,
            task_info.task_id,
            app,
            state,
        )
        .await
        .map_err(|e| format!("Error on archiving tar : {}", e)),
        Some(ArchiveFormat::SevenZ) => create_sevenzip(
            task_info.src_list,
            task_info.dest_dir,
            task_info.task_id,
            app,
            state,
        )
        .await
        .map_err(|e| format!("Error on archiving 7z : {}", e)),
        None => return Err("Unsupport file method".to_string()),
    }?;
    Ok(())
}

#[tauri::command]
pub async fn extract(
    task_info: TaskInfo,
    password: Option<String>,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    for file in task_info.src_list {
        match ArchiveFormat::from_path(Path::new(&file)) {
            Some(ArchiveFormat::Zip) => {
                decompress(
                    vec![file.clone()],
                    task_info.dest_dir.clone(),
                    task_info.task_id.clone(),
                    app.clone(),
                    state.clone(),
                )
                .await
                .map_err(|e| format!("Error on extracting file {} : {}", &file, e))?;
                continue;
            }
            Some(ArchiveFormat::Tar) => {
                decompress(
                    vec![file.clone()],
                    task_info.dest_dir.clone(),
                    task_info.task_id.clone(),
                    app.clone(),
                    state.clone(),
                )
                .await
                .map_err(|e| format!("Error on extracting file {} : {}", &file, e))?;
                continue;
            }
            Some(ArchiveFormat::SevenZ) => {
                decompress_7z(
                    vec![file.clone()],
                    task_info.dest_dir.clone(),
                    task_info.task_id.clone(),
                    app.clone(),
                    state.clone(),
                )
                .await
                .map_err(|e| format!("Error on extracting file {} : {}", &file, e))?;
                continue;
            }
            None => return Err(format!("file {} is not match any supported method", &file)),
        }
    }

    Ok(())
}

// #[tauri::command]
// pub async fn start_task(
//     task_type: TaskType,
//     app: AppHandle,
//     state: State<'_, AppState>,
// ) -> Result<(), String> {
//     match task_type {
//         TaskType::Copy {
//             source_list,
//             dest_dir,
//             task_id,
//             cancel_token,
//         } => copy(source_list, dest_dir, task_id, app, state).await?,
//         TaskType::Cut {
//             source_list,
//             dest_dir,
//             task_id,
//             cancel_token,
//         } => cut(source_list, dest_dir, task_id, app, state).await?,
//         TaskType::Archive {
//             source_list,
//             dest_dir,
//             task_id,
//             cancel_token,
//             archive_type,
//             archive_level,
//             password,
//         } => {
//             archive(
//                 task_id,
//                 source_list,
//                 dest_dir,
//                 archive_type,
//                 archive_level,
//                 password,
//                 app,
//                 state,
//             )
//             .await?
//         }
//         TaskType::Extract {
//             source_list,
//             dest_dir,
//             task_id,
//             cancel_token,
//             password,
//         } => extract(task_id, source_list, dest_dir, password, app, state).await?,
//     }
//
//     Ok(())
// }
