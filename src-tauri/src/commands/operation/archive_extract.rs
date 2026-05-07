use crate::commands::operation::operation::{
    AppState, TaskInfo, TaskProgress, generate_cancellation_token,
};
use compress_tools::ArchiveContents;
use compress_tools::tokio_support::ArchiveIteratorBuilder;
use futures_util::StreamExt;
use sevenz_rust2::*;
use std::io::Write;
use std::path::{Path, PathBuf};
use tar::Builder;
use tauri::{AppHandle, Emitter, State};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::select;
use tokio_util::sync::CancellationToken;
use walkdir::WalkDir;
use zip::{CompressionMethod, write::SimpleFileOptions};

fn is_dir_entry(stat: &compress_tools::stat) -> bool {
    (stat.st_mode as i32 & libc::S_IFMT) == libc::S_IFDIR
}

/// Resolve destination path — if dest is a directory, append src filename with given extension
fn resolve_dest(src: &str, dest: &str, ext: &str) -> Result<PathBuf, String> {
    let dest_path = Path::new(dest);
    if dest_path.is_dir() {
        let file_name = Path::new(src)
            .file_name()
            .ok_or_else(|| format!("Cannot get filename from: {}", src))?;
        Ok(dest_path.join(file_name).with_extension(ext))
    } else if dest_path.exists() {
        Err(format!("Destination file already exists: {}", dest))
    } else {
        Ok(dest_path.to_path_buf())
    }
}

fn collect_entries(src_list: &[String]) -> Result<Vec<(PathBuf, PathBuf)>, String> {
    let mut entries = vec![];
    for src_str in src_list {
        let base = Path::new(src_str)
            .parent()
            .ok_or_else(|| format!("Cannot get parent of: {}", src_str))?;
        for entry in WalkDir::new(src_str).into_iter().filter_map(|e| e.ok()) {
            let relative = entry.path().strip_prefix(base).map_err(|e| e.to_string())?;
            entries.push((entry.path().to_path_buf(), relative.to_path_buf()));
        }
    }
    Ok(entries)
}

pub async fn create_tar(
    src_list: Vec<String>,
    dest_dir: String,
    task_id: String,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let dest_path = resolve_dest(&src_list[0], &dest_dir, "tar")?;

    let token = generate_cancellation_token(&task_id, state).await.unwrap();
    tokio::task::spawn_blocking(move || {
        let dest_file = std::fs::File::create(&dest_path)
            .map_err(|e| format!("Cannot create tar file: {}", e))?;

        let mut tar = Builder::new(dest_file);
        let entries = collect_entries(&src_list)?;
        let total = entries.len() as u64;

        for (index, (abs, relative)) in entries.iter().enumerate() {
            if token.is_cancelled() {
                return Err(format!("Cancelled"));
            }
            if abs.is_dir() {
                tar.append_dir(&relative, &abs)
                    .map_err(|e| format!("Cannot append dir {:?}: {}", relative, e))?;
            } else {
                let mut f = std::fs::File::open(&abs)
                    .map_err(|e| format!("Cannot open {:?}: {}", abs, e))?;
                tar.append_file(&relative, &mut f)
                    .map_err(|e| format!("Cannot append file {:?}: {}", relative, e))?;
            }
            app.emit(
                "task-progressing",
                TaskProgress {
                    task_id: task_id.clone(),
                    value: index as u64,
                    total: total,
                    done: index == entries.len() - 1,
                },
            )
            .unwrap();
        }

        tar.finish()
            .map_err(|e| format!("Cannot finish tar: {}", e))
    })
    .await
    .map_err(|e| e.to_string())?
}

pub async fn create_sevenzip(
    src_list: Vec<String>,
    dest_dir: String,
    task_id: String,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let dest_path = resolve_dest(&src_list[0], &dest_dir, "7z")?;
    let token = generate_cancellation_token(&task_id, state).await.unwrap();

    tokio::task::spawn_blocking(move || {
        let entries = collect_entries(&src_list).map_err(|e| e.to_string())?;
        println!("total files: {}", entries.len());

        let mut sz = ArchiveWriter::create(&dest_path)
            .map_err(|e| format!("Cannot create 7z archive: {}", e))?;

        sz.set_content_methods(vec![EncoderConfiguration::new(EncoderMethod::ZSTD)]);
        let total = entries.len() as u64;
        let mut value = 0;

        for (abs, relative) in entries {
            if token.is_cancelled() {
                return Err(format!("Cancelled"));
            }
            if abs == dest_path {
                continue;
            }

            let relative_str = relative.to_string_lossy().to_string();
            println!("Archiving: {}", relative_str);

            let entry = ArchiveEntry::from_path(&abs, relative_str);
            let f = if abs.is_file() {
                std::fs::File::open(&abs).ok()
            } else {
                None
            };

            sz.push_archive_entry(entry, f)
                .map_err(|e| format!("Cannot add entry {:?}: {}", abs, e))?;
            value += 1;
            app.emit(
                "task-progressing",
                TaskProgress {
                    task_id: task_id.clone(),
                    value: value,
                    total: total,
                    done: false,
                },
            )
            .unwrap();
        }

        sz.finish()
            .map_err(|e| format!("Cannot finish archive: {}", e))?;
        app.emit(
            "task-progressing",
            TaskProgress {
                task_id: task_id.clone(),
                value: value,
                total: total,
                done: true,
            },
        )
        .unwrap();
        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?
}

pub async fn create_zip(
    src_list: Vec<String>,
    dest_dir: String,
    task_id: String,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let dest_path = resolve_dest(&src_list[0], &dest_dir, "zip")?;
    let token = generate_cancellation_token(&task_id, state).await.unwrap();

    tokio::task::spawn_blocking(move || {
        let entries = collect_entries(&src_list)?;

        let file =
            std::fs::File::create(&dest_path).map_err(|e| format!("Cannot create zip: {}", e))?;
        let mut zip = zip::ZipWriter::new(file);
        let options = SimpleFileOptions::default().compression_method(CompressionMethod::Zstd);

        let mut buf = vec![0u8; 256 * 1024];
        let mut file_done = 0usize;
        let mut bytes_copied = 0u64;
        let total = entries.len();
        for (abs, relative) in entries {
            if token.is_cancelled() {
                return Err(format!("Cancelled"));
            }
            let path_str = relative
                .to_str()
                .map(str::to_owned)
                .ok_or_else(|| format!("{:?} is Non UTF-8", relative))?;

            if abs.is_file() {
                file_done += 1;
                zip.start_file(&path_str, options)
                    .map_err(|e| format!("Cannot start zip entry {}: {}", path_str, e))?;

                let mut f = std::fs::File::open(&abs)
                    .map_err(|e| format!("Cannot open {:?}: {}", abs, e))?;

                loop {
                    use std::io::Read;
                    let n = f.read(&mut buf).map_err(|e| e.to_string())?;
                    if n == 0 {
                        break;
                    }
                    zip.write_all(&buf[..n]).map_err(|e| e.to_string())?;
                    bytes_copied += n as u64;
                }
            } else if !relative.as_os_str().is_empty() {
                zip.add_directory(&path_str, options)
                    .map_err(|e| format!("Cannot add dir {}: {}", path_str, e))?;
            }
            app.emit(
                "task-progressing",
                TaskProgress {
                    task_id: task_id.clone(),
                    value: file_done as u64,
                    total: total as u64,
                    done: false,
                },
            )
            .unwrap();
        }

        zip.finish()
            .map_err(|e| format!("Cannot finish zip: {}", e))?;
        app.emit(
            "task-progressing",
            TaskProgress {
                task_id: task_id.clone(),
                value: file_done as u64,
                total: total as u64,
                done: true,
            },
        )
        .unwrap();
        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?
}

pub async fn decompress_7z(
    src_list: Vec<String>,
    dest_dir: String,
    task_id: String,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let dest_path = Path::new(&dest_dir);
    if !dest_path.is_dir() {
        return Err(format!("Destination must be a directory: {}", dest_dir));
    }
    let token = generate_cancellation_token(&task_id, state).await.unwrap();

    tokio::task::spawn_blocking(move || {
        let dest = Path::new(&dest_dir);
        for src_str in &src_list {
            let mut reader = std::fs::File::open(src_str)
                .map_err(|e| format!("Cannot open {}: {}", src_str, e))?;

            let archive = Archive::read(&mut reader, &Password::empty())
                .map_err(|e| format!("Cannot read archive: {}", e))?;

            let mut archive_reader =
                ArchiveReader::from_archive(archive.clone(), &mut reader, Password::empty());
            let total = archive.files.len();
            let mut value = 0;

            for entry in &archive.files {
                if token.is_cancelled() {
                    return Err(format!("Cancelled"));
                }
                if entry.is_directory() {
                    std::fs::create_dir_all(dest.join(&entry.name))
                        .map_err(|e| format!("Cannot create dir {}: {}", entry.name, e))?;
                } else {
                    let out = dest.join(&entry.name);
                    if let Some(parent) = out.parent() {
                        std::fs::create_dir_all(parent).map_err(|e| {
                            format!("Cannot create parent for {}: {}", entry.name, e)
                        })?;
                    }
                    let data = archive_reader
                        .read_file(&entry.name)
                        .map_err(|e| format!("Cannot read {}: {}", entry.name, e))?;
                    std::fs::File::create(&out)
                        .and_then(|mut f| f.write_all(&data))
                        .map_err(|e| format!("Cannot write {}: {}", entry.name, e))?;
                }
                value += 1;
                app.emit(
                    "task-progressing",
                    TaskProgress {
                        task_id: task_id.clone(),
                        value: value as u64,
                        total: total as u64,
                        done: false,
                    },
                )
                .unwrap();
            }
            app.emit(
                "task-progressing",
                TaskProgress {
                    task_id: task_id.clone(),
                    value: value as u64,
                    total: total as u64,
                    done: true,
                },
            )
            .unwrap();
        }
        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?
}

pub async fn decompress(
    src_list: Vec<String>,
    dest_dir: String,
    task_id: String,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    for src_file in src_list {
        let token = generate_cancellation_token(&task_id, state.clone())
            .await
            .unwrap();

        let dest_path = Path::new(&dest_dir);
        if !dest_path.is_dir() {
            return Err(format!("Destination not found: {}", dest_dir));
        }

        let source = File::open(&src_file)
            .await
            .map_err(|e| format!("Cannot open archive {}: {}", src_file, e))?;

        let mut iter = ArchiveIteratorBuilder::new(source).build();
        let mut current_writer: Option<BufWriter<File>> = None;
        let mut value = 0;

        while let Some(content) = iter.next().await {
            if token.is_cancelled() {
                return Err(format!("Canncelled"));
            }
            match content {
                ArchiveContents::StartOfEntry(name, stat) => {
                    // Flush previous entry
                    if let Some(mut w) = current_writer.take() {
                        w.flush().await.map_err(|e| e.to_string())?;
                    }

                    let out_path = dest_path.join(&name);

                    if is_dir_entry(&stat) {
                        tokio::fs::create_dir_all(&out_path)
                            .await
                            .map_err(|e| format!("Cannot create dir {}: {}", name, e))?;
                        continue;
                    }

                    if let Some(parent) = out_path.parent() {
                        tokio::fs::create_dir_all(parent)
                            .await
                            .map_err(|e| format!("Cannot create parent for {}: {}", name, e))?;
                    }

                    let file = File::create(&out_path)
                        .await
                        .map_err(|e| format!("Cannot create {}: {}", name, e))?;

                    println!("Extracting: {}", name);
                    current_writer = Some(BufWriter::new(file));
                }

                ArchiveContents::DataChunk(data) => {
                    if let Some(w) = &mut current_writer {
                        w.write_all(&data)
                            .await
                            .map_err(|e| format!("Write error: {}", e))?;
                    }
                }

                ArchiveContents::EndOfEntry => {
                    if let Some(mut w) = current_writer.take() {
                        w.flush().await.map_err(|e| format!("Flush error: {}", e))?;
                    }
                }

                ArchiveContents::Err(e) => {
                    if let Some(mut w) = current_writer.take() {
                        w.flush().await.ok();
                    }
                    return Err(format!("Archive error: {:?}", e));
                }
            }
            value += 1;
            app.emit(
                "task-progressing",
                TaskProgress {
                    task_id: task_id.clone(),
                    value: value as u64,
                    total: 0 as u64,
                    done: false,
                },
            )
            .unwrap();
        }

        // Flush last entry if EndOfEntry wasn't emitted
        if let Some(mut w) = current_writer.take() {
            w.flush()
                .await
                .map_err(|e| format!("Final flush error: {}", e))?;
        }
        app.emit(
            "task-progressing",
            TaskProgress {
                task_id: task_id.clone(),
                value: value as u64,
                total: 0 as u64,
                done: true,
            },
        )
        .unwrap();
    }

    Ok(())
}
