use prost::Message;
use rayon::prelude::*;
use std::{
    collections::{BTreeMap, HashMap},
    fs::{self, File},
    io::{Read, Seek, SeekFrom},
    path::Path,
};
mod index {
    include!(concat!(env!("OUT_DIR"), "/file_manager.index.rs"));
}
use super::utils::split_path;
use crate::commands::{config::builder::get_config_path, error::Error};
use index::{DocumentInfo, DocumentStore, InvertedIndex, Metadata, Posting, Vocab, VocabEntry};
#[tauri::command]
pub fn search(input: String) -> Result<String, Error> {
    let splited_input = split_path(input.clone());
    let vocab_bytes = fs::read(get_config_path("vocab_index.pb"))?;
    let vocab: Vocab = index::Vocab::decode(&vocab_bytes[..])?;
    let vocab_entries: Vec<VocabEntry> = vocab.entries;
    let document_bytes = fs::read(get_config_path("document_index.pb"))?;
    let document = index::DocumentStore::decode(&document_bytes[..])?;

    let mut index_file = File::open(get_config_path("index.pb"))?;

    let doc: HashMap<String, VocabEntry> = vocab_entries
        .into_iter()
        .map(|entry| (entry.trigram.clone(), entry))
        .collect();
    let mut doc_id: Vec<Vec<u64>> = Vec::new();
    let mut buffer = Vec::with_capacity(1024 * 1024);
    splited_input.into_iter().for_each(|trigram| {
        if let Some(entry) = doc.get(&trigram) {
            index_file
                .seek(SeekFrom::Start(entry.postings_offset))
                .unwrap();
            buffer.resize(entry.postings_length as usize, 0);
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

    let result: Vec<String> = doc_list
        .par_iter()
        .filter_map(|(id, freq)| {
            let doc_info = match document.docs.get(*id as usize) {
                Some(info) => info,
                None => {
                    eprintln!("Warning: Document ID {} not found", id);
                    return None;
                }
            };
            let path = Path::new(&doc_info.path);
            Some(path.display().to_string())
        })
        .collect();
    // result.sort_by(|a, b| {
    //     b.last_modified.cmp(&a.last_modified) // mới hơn lên trước
    // });
    let encoded = serde_json::to_string(&result)?;
    Ok(encoded)
}
