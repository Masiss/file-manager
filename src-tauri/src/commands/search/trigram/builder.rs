use chrono::Utc;
use prost::Message;
mod index {
    include!(concat!(env!("OUT_DIR"), "/file_manager.index.rs"));
}
use super::utils::split_path;
use crate::commands::config::builder::{Config, get_config_path};
use index::{DocumentInfo, DocumentStore, InvertedIndex, Metadata, Posting, Vocab, VocabEntry};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    sync::OnceLock,
};
use walkdir::{DirEntry, WalkDir};
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
