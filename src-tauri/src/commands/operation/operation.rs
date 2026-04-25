use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Emitter, Manager, State};
use tokio::fs::{File, try_exists};
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
    let mut result: Vec<String> = vec![];
    for source_path in source_list {
        let file_name = Path::new(&source_path)
            .file_name()
            .and_then(|f| f.to_str())
            .ok_or("cant find file")?;
        println!("{}", file_name);
        let dest_path: String = Path::new(&dest_dir)
            .join(file_name)
            .to_string_lossy()
            .to_string();
        println!("{}", dest_path);
        match try_exists(&dest_path).await {
            Ok(true) => result.push(dest_path),
            Ok(false) => {}
            Err(e) => return Err(format!("Error checking file existence: {e}")),
        }
    }
    println!("{:?}", result);
    if !result.is_empty() {
        Ok(CopyResult::Conflict { file_list: result })
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
    // state.task_info.lock().await.cancel_token = Some(token);
    *state.task_info.lock().await = TaskInfo {
        source_list,
        dest_dir,
        task_id,
        cancel_token: Some(token),
    };

    Ok(())
}
#[tauri::command]
pub async fn copy(
    source_list: Vec<String>,
    dest_dir: String,
    task_id: String,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
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

            let mut reader = File::open(path).await.map_err(|e| e.to_string())?;
            let mut writer = File::create(&dest_file).await.map_err(|e| e.to_string())?;
            let total = reader.metadata().await.map_err(|e| e.to_string())?.len();

            let mut buf = vec![0u8; 256 * 1024]; // 256KB chunks
            let mut copied = 0u64;

            app.emit(
                "copy-progressing",
                CopyProgress {
                    task_id: task_id.clone(),
                    file: path_str.to_string(),
                    copied,
                    total,
                    done: false,
                },
            )
            .unwrap();
            //start copy
            copied_list.push(dest_file);
            loop {
                select! {
                  _ = token.cancelled() => {
                       for file in copied_list{

                           tokio::fs::remove_file(file).await;
                       }
                       app.emit("copy-close","").ok();
                       return Err("cancelled".to_string())

                   }

                result = reader.read(&mut buf) =>{
                   let n = result.map_err(|e| e.to_string())?;
                   if n == 0 {
                       break;
                   }
                   writer
                       .write_all(&buf[..n])
                       .await
                       .map_err(|e| e.to_string())?;
                   copied += n as u64;
                   println!("copying");
                   app.emit(
                       "copy-progressing",
                       CopyProgress {
                       task_id: task_id.clone(),
                           file: path_str.to_string(),
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
            writer.flush().await.map_err(|e| e.to_string())?;
            app.emit(
                "copy-progressing",
                CopyProgress {
                    task_id: task_id.clone(),
                    file: path_str.to_string(),
                    copied,
                    total,
                    done: true,
                },
            )
            .unwrap();
            println!("copy done!");
        }
    }

    Ok(())
}
