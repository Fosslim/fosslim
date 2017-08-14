//use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::collections::{HashMap, HashSet};

use serde::Deserialize;
use serde_json;
use serde_json::{Value, Error};

use document::Document;

pub struct IndexerError<'a> {
    pub message: &'a str
}

impl<'a> IndexerError<'a> {
    fn new(msg: &'a str) -> IndexerError {
        IndexerError { message: msg }
    }
}

pub struct Index<'a> {
    pub n_terms: u32,
    pub n_docs: u32,
    terms:  HashMap<&'a str, u32>,
    documents: Vec<Box<Document<'a>>>,
    term_doc_idx: Vec<Vec<u32>>, // matrix [0 -> [docID1, docID2]]
}

impl<'a> Index<'a> {
    pub fn new() -> Index<'a> {
        Index {
            n_terms: 0,
            n_docs: 0,
            terms: HashMap::new(),
            documents: vec![],
            term_doc_idx: vec![]
        }
    }

    fn add_term(&mut self, term: &'a str) -> Result<u32, IndexerError> {
        if self.terms.contains_key(term){
            self.terms.get(term)
        } else {
            let term_id = self.n_terms;
            self.terms.insert(term, term_id);
            self.n_terms += 1;

            Ok(term_id)
        }
    }

    fn add_doc_into_term_idx(&mut self, term_id: u32, doc_id: u32) -> Result<u32, IndexerError> {
        let mut idx = self.term_doc_idx;

        let doc_pos = match self.term_doc_idx.get(term_id) {
            None => {
                idx[term_id] = vec![doc_id];
                0
            },
            Some(doc_ids) => {
                idx[term_id].push(doc_id);
                doc_ids.len()
            }
        };

        Ok(doc_pos)
    }

    pub fn add(&mut self, doc: &'a Document) -> Result<u32, IndexerError> {
        let current_doc_id = self.n_docs;

        //TODO: normalize and tokenize document here
        for term in doc.tcm.keys() {

            match self.add_term(term) {
                Err(err)    => return Err(e),
                Ok(term_id) => {
                    // add document into term_doc_idx
                    let mut idx = self.term_doc_idx.get(term_id);
                    self.add_doc_into_term_idx(term_id, current_doc_id)
                }
            }

        }

        //add document into documents;
        self.documents.push(Box::new(doc));
        
        Ok(self.n_terms)
    }
}
