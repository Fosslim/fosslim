//use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::collections::{HashMap, HashSet};

use serde::Deserialize;
use serde_json;
use serde_json::{Value, Error};

use document::Document;


type DocumentHeap<'a> = Vec<Box<Document<'a>>>;

pub struct IndexerError<'a> {
    pub message: &'a str,
}

impl<'a> IndexerError<'a> {
    fn new(msg: &'a str) -> IndexerError {
        IndexerError { message: msg }
    }
}

pub struct Index<'a> {
    pub n_terms: usize,
    pub n_docs: usize,
    terms: HashMap<&'a str, usize>,
    documents: Vec< Box<Document<'a>> >,
    term_doc_idx: Vec<Vec<usize>>, // matrix [0 -> [docID1, docID2]]
}

impl<'a> Index<'a> {
    pub fn new() -> Index<'a> {
        Index {
            n_terms: 0,
            n_docs: 0,
            terms: HashMap::new(), // TODO: replace with fst?
            documents: Vec::new(),
            term_doc_idx: vec![],
        }
    }


    pub fn add_term(&mut self, term: &'a str) -> Option<usize> {
        if self.terms.contains_key(term) {
            let term_id:usize = *self.terms.get(&term).unwrap();
            Some(term_id)

        } else {
            let term_id = self.n_terms;
            self.terms.insert(term, term_id);
            self.n_terms += 1;

            Some(term_id)
        }
    }

    pub fn add(&mut self, doc: Box<Document<'a>> ) -> Result<usize, IndexerError> {
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

        //TODO: mover tokenizer into own module
        let mut doc = self.documents[doc_id].clone();
        doc.tokenize();

        for term in doc.tcm.keys() {
            match self.add_term(term) {
                None => return Err(IndexerError::new("Failed to add term into index")),
                Some(term_id) => {
                    // add document into term_doc_idx
                    self.add_doc_into_term_idx(term_id, doc_id);
                }
            }
        }

        Ok(doc_id)
    }

    pub fn get_docs_by_term(&self, term: &'a str) -> Option<DocumentHeap> {
        if !self.terms.contains_key(term) {
            return None;
        }

        let term_id = self.terms[term];
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
