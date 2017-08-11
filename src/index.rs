//use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

use serde::Deserialize;
use serde_json;
use serde_json::{Value, Error};

use document::Document;

pub struct IndexerError {
    pub message: &'static str
}

impl IndexerError {
    fn new(msg: &'static str) -> IndexerError {
        IndexerError { message: msg }
    }
}

pub struct Index {
    pub n_terms: usize,
    pub n_documents: usize,
    terms: Vec<&'static str>,
    documents: Vec<Box<Document>>,
    term_doc_idx: Vec<Vec<usize>>, // matrix [0 -> [docID1, docID2]]
    doc_term_idx: Vec<Vec<usize>> // matrix
}

pub fn build_from_json<'a>(json_txt: &'a str) -> Result<u32, IndexerError > {
    let val: serde_json::Value;

    match serde_json::from_str(json_txt) {
        Ok(json_val) => val = json_val,
        Err(_)  => return Err(IndexerError::new("Failed to parse json text."))
    };


    Ok(1)
}