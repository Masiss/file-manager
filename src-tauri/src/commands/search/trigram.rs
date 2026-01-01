use chrono::Utc;
use dirs::config_dir;
use prost::Message;
use rayon::prelude::*;

use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    fs::{self, File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    sync::OnceLock,
};
use toml::{self};
use walkdir::{DirEntry, WalkDir};

mod index {
    include!(concat!(env!("OUT_DIR"), "/file_manager.index.rs"));
}
use index::{DocumentInfo, DocumentStore, InvertedIndex, Metadata, Posting, Vocab, VocabEntry};

use crate::commands::fs::directory::{File as FileStruct, create_file_from_path, load_file};
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    except_list: Vec<String>,
}
fn get_config_path(filename: &str) -> PathBuf {
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
static EXCEPT_LIST: OnceLock<Vec<String>> = OnceLock::new();
fn get_except_list() -> &'static Vec<String> {
    EXCEPT_LIST.get_or_init(|| {
        let config_file = "config.toml";
        let mut config_file =
            File::open(get_config_path(config_file)).expect("Failed to get config file");
        let mut content = String::new();
        config_file
            .read_to_string(&mut content)
            .expect("Cant read config file");
        let config: Config = toml::from_str(&content).expect("Failed to convert TOML to str");
        config
            .except_list
            .iter()
            .map(|e| e.to_lowercase())
            .collect()
    })
}
fn is_in_except(entry: &DirEntry) -> bool {
    let name = entry.file_name().to_str().unwrap().to_lowercase();
    if name.ends_with(".bin")
        || name.ends_with(".dll")
        || name.ends_with(".tmp")
        || name.ends_with(".log")
    {
        return true;
    }
    let except_list = get_except_list();
    entry
        .path()
        .components()
        .filter_map(|c| c.as_os_str().to_str())
        .any(|name| except_list.contains(&name.to_string().to_lowercase()))
}

#[tauri::command]
pub fn generate_index() {
    let walker = WalkDir::new("\\").into_iter();
    let mut trigrams: HashMap<String, Vec<u64>> = HashMap::new();
    let mut docs: Vec<DocumentInfo> = Vec::new();
    for (index, entry) in walker
        .filter_entry(|e| !is_in_except(e))
        .flatten()
        .enumerate()
    {
        let doc_id = index;
        let path = entry.path().to_str().unwrap();
        let splited_path = split_path(path.to_string());
        splited_path.iter().for_each(|trigram| {
            trigrams
                .entry(trigram.to_string())
                .and_modify(|doc_list| doc_list.push(doc_id as u64))
                .or_insert(vec![doc_id as u64]);
        });
        docs.push(DocumentInfo {
            path: path.to_string(),
        });
    }
    let mut vocab_entries: Vec<VocabEntry> = Vec::new();
    let mut index_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(get_config_path("index.pb"))
        .expect("Cant create index file");
    let mut vocab_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(get_config_path("vocab_index.pb"))
        .expect("Cant create vocab file");
    let mut document_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(get_config_path("document_index.pb"))
        .expect("Cant create document file");
    let mut metadata_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(get_config_path("metadata_index.pb"))
        .expect("Cant create metadata file");
    for (trigram, doc_id) in trigrams {
        let mut current_bytes = index_file.seek(SeekFrom::Current(0)).unwrap();
        let postings: Vec<Posting> = doc_id
            .into_iter()
            .map(|id| index::Posting { doc_id: id })
            .collect::<Vec<Posting>>();
        let inverted_index: InvertedIndex = index::InvertedIndex { postings };
        let mut index_buf = inverted_index.encode_length_delimited_to_vec();
        index_file.write_all(&index_buf);
        let new_pos = index_file.seek(SeekFrom::Current(0)).unwrap();
        let postings_length = new_pos - current_bytes;

        vocab_entries.push(VocabEntry {
            trigram,
            postings_offset: current_bytes,
            postings_length,
        });
    }

    let metadata: Metadata = index::Metadata {
        total_documents: docs.len() as u64,
        total_unique_trigrams: vocab_entries.len() as u64,
        build_time: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };
    metadata_file.write_all(&metadata.encode_to_vec());
    let vocab: Vocab = index::Vocab {
        entries: vocab_entries,
    };
    let document: DocumentStore = index::DocumentStore { docs };
    document_file.write_all(&document.encode_to_vec());
    let vocab_encoded = vocab.encode_to_vec();
    vocab_file.write_all(&vocab_encoded);
    index_file.flush().unwrap();
    vocab_file.flush().unwrap();
    document_file.flush().unwrap();
    metadata_file.flush().unwrap();
}
fn split_path(path: String) -> Vec<String> {
    // 2 spaces prefix and 1 space suffix
    let padded_path = format!("  {} ", path);
    let lower_padded = padded_path.to_ascii_lowercase();
    lower_padded
        .chars()
        .collect::<Vec<char>>()
        .windows(3)
        .map(|e| e.iter().collect::<String>())
        .collect()
}

#[tauri::command]
pub fn search(input: String) -> String {
    let splited_input = split_path(input.clone());
    let vocab_bytes = fs::read(get_config_path("vocab_index.pb")).unwrap();
    let vocab: Vocab = index::Vocab::decode(&vocab_bytes[..]).unwrap();
    let vocab_entries: Vec<VocabEntry> = vocab.entries;
    let document_bytes = fs::read(get_config_path("document_index.pb")).unwrap();
    let document = index::DocumentStore::decode(&document_bytes[..]).unwrap();

    let mut index_file = File::open(get_config_path("index.pb")).unwrap();

    let doc: HashMap<String, VocabEntry> = vocab_entries
        .into_iter()
        .map(|entry| (entry.trigram.clone(), entry))
        .collect();
    let mut doc_id: Vec<Vec<u64>> = Vec::new();
    splited_input.into_iter().for_each(|trigram| {
        if let Some(entry) = doc.get(&trigram) {
            index_file
                .seek(SeekFrom::Start(entry.postings_offset))
                .unwrap();
            let mut buffer = vec![0u8; entry.postings_length as usize];
            index_file.read_exact(&mut buffer).unwrap();
            let inverted_index: InvertedIndex =
                index::InvertedIndex::decode_length_delimited(&buffer[..]).unwrap();
            let postings = inverted_index.postings;
            let docs = postings
                .into_iter()
                .map(|posting| posting.doc_id)
                .collect::<Vec<u64>>();
            doc_id.push(docs);
        }
    });
    let mut doc_scores: BTreeMap<u64, f64> = BTreeMap::new();
    let mut doc_list: BTreeMap<u64, i32> = BTreeMap::new();
    let flatten_doc: Vec<u64> = doc_id.into_iter().flatten().collect();
    flatten_doc.into_iter().for_each(|id| {
        doc_list
            .entry(id)
            .and_modify(|value| *value += 1)
            .or_insert(1);
    });
    let input_lower = input.to_lowercase();
    for (&doc_id, score) in doc_scores.iter_mut() {
        let doc_info = &document.docs[doc_id as usize];
        let name_lower = doc_info.path.to_lowercase();

        if name_lower.contains(&input_lower) {
            *score += 30.0; // exact match boost mạnh
        }
        if name_lower.ends_with(&input_lower) {
            *score += 10.0; // suffix (ví dụ gõ "pdf" → .pdf lên top)
        }
        if name_lower.starts_with(&input_lower) {
            *score += 15.0; // prefix mạnh nhất
        }
    }

    // Sort theo score giảm dần
    let mut scored_results: Vec<(u64, f64)> = doc_scores.into_iter().collect();
    scored_results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let mut result: Vec<FileStruct> = doc_list
        .par_iter()
        .map(|(id, freq)| {
            let doc_info = document.docs.get(*id as usize).unwrap();
            let path = Path::new(&doc_info.path);
            let metadata = path.metadata().unwrap();
            let file: FileStruct = create_file_from_path(path, metadata);
            file
        })
        .collect();
    result.sort_by(|a, b| {
        b.last_modified.cmp(&a.last_modified) // mới hơn lên trước
    });
    serde_json::to_string(&result).unwrap()
}
