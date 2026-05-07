use crate::commands::operation::archive_extract::{
    create_sevenzip, create_tar, create_zip, decompress, decompress_7z,
};

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tauri::{App, Emitter};
use tauri::{AppHandle, State};
use tokio::fs::{self, File, try_exists};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::select;
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;
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
