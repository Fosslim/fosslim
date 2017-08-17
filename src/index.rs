//use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::collections::{HashMap, HashSet};

use document::{self, Document};

pub struct IndexerError<'a> {
    pub message: &'a str,
}

impl<'a> IndexerError<'a> {
    fn new(msg: &'a str) -> IndexerError {
        IndexerError { message: msg }
    }
}

pub struct Index {
    pub n_terms: usize,
    pub n_docs: usize,
    terms: HashMap<String, usize>,
    documents: Vec<Document>,
    term_doc_idx: Vec<Vec<usize>>, // matrix [0 -> [docID1, docID2]]
}

impl Index {
    pub fn new() -> Index {
        Index {
            n_terms: 0,
            n_docs: 0,
            terms: HashMap::new(), // TODO: replace with fst?
            documents: Vec::new(),
            term_doc_idx: vec![],
        }
    }


    pub fn add_term(&mut self, term: String) -> Option<usize> {
        if self.terms.contains_key(&term) {
            let term_id:usize = *self.terms.get(&term).unwrap();
            Some(term_id)

        } else {
            let term_id = self.n_terms;
            self.terms.insert(term, term_id);
            self.n_terms += 1;

            Some(term_id)
        }
    }

    pub fn add(&mut self, doc: Document ) -> Result<usize, IndexerError> {
        let current_doc_id = self.n_docs;

        //add document into documents;
        self.documents.push( doc );
        self.n_docs += 1;

        Ok(current_doc_id)
    }

    pub fn index_doc(&mut self, doc_id: usize) -> Result<usize, IndexerError> {
        if doc_id >= self.documents.len() {
            return Err(IndexerError::new("found no such document"));
        }

        // TODO: mover tokenizer into own module
        let mut doc = self.documents[doc_id].clone();
        doc.tokenize();

        for term in doc.tcm.keys() {
            match self.add_term(term.clone()) {
                None => return Err(IndexerError::new("Failed to add term into index")),
                Some(term_id) => {
                    // add document into term_doc_idx
                    self.add_doc_into_term_idx(term_id, doc_id);
                }
            }
        }

        Ok(doc_id)
    }

    pub fn get_docs_by_term(&self, term: String) -> Option<Vec<Document>> {
        if !self.terms.contains_key(&term) {
            return None;
        }

        let term_id = self.terms[&term];
        let docs = self.term_doc_idx[term_id]
            .iter()
            .fold(vec![], |mut acc, &id|{
                acc.push(self.documents[id].clone());
                acc
            });

        Some(docs)
    }

    fn add_doc_into_term_idx(&mut self, term_id: usize, doc_id: usize) -> Option<usize> {

        // term doesnt exist in the index
        if term_id >= self.term_doc_idx.len() {
            self.term_doc_idx.push(vec![])
            //self.term_doc_idx[term_id] = vec![]
        };

        let doc_pos = self.term_doc_idx[term_id].len();

        //add doc into term index
        self.term_doc_idx[term_id].push(doc_id);

        Some(doc_pos)
    }
}


// builds Index from json files found in the path
pub fn build_from_path<'a>(target_path: &'a str) -> Result<Index, IndexerError> {
    let path = Path::new(target_path);
    if !path.exists() {
        return Err(IndexerError::new("target path doesnt exists or is not accessible"));
    }

    let mut idx = Index::new();

    // iterate over files and build docs and add them into index
    for entry in path.read_dir().expect("read_dir failed") {
        if let Ok(metadata) = entry {
            // add new document into index only if it was parsed successfullys
            if let Ok(doc) = document::from_json_file(metadata.path()) {
               idx.add(doc);
            }

        }
    }

    Ok(idx)
}

