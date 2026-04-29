use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::fmt::format;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Emitter, Manager, State};
use tokio::fs::{self, File, try_exists};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
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
struct CopyProgress {
    task_id: String,
    file: String,
    copied: u64,
    total: u64,
    done: bool,
}

#[derive(Default)]
pub struct AppState {
    task_info: Mutex<TaskInfo>,
}

#[derive(Default, Clone)]
struct TaskInfo {
    source_list: Vec<String>,
    dest_dir: String,
    task_id: String,
    cancel_token: Option<CancellationToken>,
}
#[tauri::command]
pub async fn cancel(task_id: String, state: State<'_, AppState>) -> Result<(), String> {
    let task = state.task_info.lock().await;
    if let Some(token) = task.cancel_token.as_ref() {
        token.cancel();
        println!("cancelled");
    }

    Ok(())
}
pub async fn start_task(
    source_list: Vec<String>,
    dest_dir: String,
    task_id: String,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let token = CancellationToken::new();
    *state.task_info.lock().await = TaskInfo {
        source_list,
        dest_dir,
        task_id,
        cancel_token: Some(token),
    };

    Ok(())
}
pub async fn copy_files(
    source_list: Vec<String>,
    dest_dir: String,
    task_id: String,
    app: AppHandle,
    state: State<'_, AppState>,
) -> anyhow::Result<Vec<PathBuf>> {
    let token = CancellationToken::new();
    state.task_info.lock().await.cancel_token = Some(token.clone());
    let dest = Path::new(&dest_dir);
    let mut copied_list = vec![];
    for path_str in source_list {
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
            copied_list.push(dest_file);
            loop {
                select! {
                    biased;

                  _ = token.cancelled() => {
                       for file in copied_list{

                           tokio::fs::remove_file(file).await?;
                       }
                       app.emit("copy-close","").ok();
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
                       "copy-progressing",
                       CopyProgress {
                       task_id: task_id.to_string(),
                           file: file_name.clone(),
                           copied,
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
                "copy-progressing",
                CopyProgress {
                    task_id: task_id.clone(),
                    file: file_name.clone(),
                    copied,
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
    source_list: Vec<String>,
    dest_dir: String,
    task_id: String,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    copy_files(source_list, dest_dir, task_id, app, state)
        .await
        .map_err(|e| format!("Copy file error: {e}"))?;
    Ok(())
}

#[tauri::command]
pub async fn cut(
    source_list: Vec<String>,
    dest_dir: String,
    task_id: String,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let copied_list: Vec<PathBuf> = copy_files(source_list, dest_dir, task_id, app, state)
        .await
        .map_err(|e| format!("Copy file error: {e}"))?;
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
