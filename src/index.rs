//use std::error::Error;
use std::fs::File;
use std::path::Path;
//TODO: reseacrc: will it be bottleneck, or BufferedWriter
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::io::{Read, Write};

//use rmp_serde::{Deserializer, Serializer};
//use serde::{Deserialize, Serialize};

use document::{self, Document};
use tokenizer;

type TermIndex = Vec<Vec<usize>>;
type TermIdPair = (usize, String);

#[derive(Debug)]
pub struct IndexerError<'a> {
    pub message: &'a str,
}

impl<'a> IndexerError<'a> {
    fn new(msg: &'a str) -> IndexerError {
        IndexerError { message: msg }
    }
}

#[derive(Clone, Debug)]
pub struct Index {
    pub n_terms: usize,
    pub n_docs: usize, // number of indexed documents
    terms: BTreeMap<String, usize>,
    documents: Vec<Document>,
    term_doc_idx: Vec<Vec<usize>>, // matrix [0 -> [docID1, docID2]]
}

impl Index {
    pub fn new() -> Index {
        Index {
            n_terms: 0,
            n_docs: 0,
            terms: BTreeMap::new(), // TODO: replace with fst?
            documents: Vec::new(),
            term_doc_idx: vec![],
        }
    }

    pub fn add_term(&mut self, term: String) -> Option<usize> {
        if self.terms.contains_key(&term) {
            let term_id: usize = *self.terms.get(&term).unwrap();
            Some(term_id)
        } else {
            let term_id = self.n_terms;
            self.terms.insert(term, term_id);
            self.n_terms += 1;

            Some(term_id)
        }
    }

    pub fn add(&mut self, doc: Document) -> Result<usize, IndexerError> {
        let current_doc_id = self.n_docs;

        //add document into documents;
        self.documents.push(doc);
        self.n_docs += 1;

        Ok(current_doc_id)
    }

    pub fn index(&mut self) -> Result<usize, IndexerError> {
        let mut doc_id: usize = 0;

        for doc in self.documents.clone().iter() {
            let tokens = tokenizer::tokenize_whitespace(doc.text.clone());

            for term in tokens.iter() {
                // &mut *self - trick to pass compiler error when re-burrowing mutable val
                match Index::add_term(&mut *self, term.clone()) {
                    None => return Err(IndexerError::new("Failed to add term into index")),
                    Some(term_id) => {
                        // add document into term_doc_idx
                        self.add_doc_into_term_idx(term_id, doc_id);
                    }
                }
            }

            doc_id += 1; // here it increase doc_id
        }

        self.n_docs = doc_id; // keep number of documents that are indexed
        Ok(doc_id)
    }

    pub fn get_docs_by_term(&self, term: String) -> Option<Vec<Document>> {
        if !self.terms.contains_key(&term) {
            return None;
        }

        let term_id = self.terms[&term];
        let docs = self.term_doc_idx[term_id]
            .iter()
            .fold(vec![], |mut acc, &id| {
                acc.push(self.documents[id].clone());
                acc
            });

        Some(docs)
    }

    pub fn get_terms(&self) -> Vec<String> {
        let mut vocabulary = Vec::with_capacity(self.n_terms);

        for (term, &term_id) in self.terms.iter() {
            vocabulary.push((term_id, term.to_string()))
        }

        vocabulary.sort_by(|a, b| cmp_term_pair(a, b)); // sort in ascending order
                                                        // return only term strings as it is sorted
        vocabulary.iter().map(|x| x.1.clone()).collect()
    }

    fn add_doc_into_term_idx(&mut self, term_id: usize, doc_id: usize) -> Option<usize> {
        // term doesnt exist in the index
        if term_id >= self.term_doc_idx.len() {
            self.term_doc_idx.push(vec![])
        };

        let doc_pos = self.term_doc_idx[term_id].len();

        //add doc into term index
        self.term_doc_idx[term_id].push(doc_id);

        Some(doc_pos)
    }

    pub fn get_term_index(&self) -> Vec<(usize, Vec<usize>)> {
        let mut term_idx = Vec::with_capacity(self.n_terms);
        let mut term_id = 0 as usize;

        for doc_ids in self.term_doc_idx.iter() {
            term_idx.push((term_id, doc_ids.clone()));
            term_id += 1;
        }

        term_idx
    }

    pub fn get_document_label(&self, doc_id: usize) -> Option<String> {
        match self.documents.get(doc_id) {
            Some(doc) => Some(doc.clone().label),
            _ => None,
        }
    }

    pub fn get_documents(&self) -> Vec<Document> {
        self.documents.clone()
    }
}

// builds Index from json files found in the path
pub fn build_from_path<'a>(target_path: &'a str) -> Result<Index, IndexerError> {
    let path = Path::new(target_path);
    if !path.exists() {
        return Err(IndexerError::new(
            "target path doesnt exists or is not accessible",
        ));
    }

    let mut idx = Index::new();

    //TODO: refactor it into document::read_all_from_folder(&path)
    // iterate over files and build docs and add them into index
    for entry in path.read_dir().expect("read_dir failed") {
        if let Ok(metadata) = entry {
            // add new document into index only if parsing was successful
            if let Ok(doc) = document::from_json_file(metadata.path()) {
                idx.add(doc).expect("Failed to add document");
            }
        }
    }

    idx.index().is_ok(); // re-index the content

    Ok(idx)
}

// dump index into file
pub fn save<'a>(idx: &Index, target_path: &'a str) -> Result<bool, IndexerError<'a>> {
    let mut fp = match File::create(target_path) {
        Ok(fp) => fp,
        Err(_) => return Err(IndexerError::new("Failed to open targetfile")),
    };

    let mut buf: Vec<u8> = Vec::new();
    /* remove broken dependency
    idx.serialize(&mut Serializer::new(&mut buf))
        .expect("Failed to serialize index");

    fp.write_all(&buf).expect("Failed to write into file");
    fp.sync_all().expect("Failed to save file on the disk");
    */

    Ok(true)
}

pub fn load<'a>(source_path: &'a str) -> Result<Index, IndexerError> {
    let mut fp = match File::open(source_path) {
        Ok(fp) => fp,
        Err(_) => return Err(IndexerError::new("Failed to open sourcefile")),
    };

    let mut buf = Vec::new();
    fp.read_to_end(&mut buf)
        .expect("Failed to read a content of the sourcefile");

    /* remove it
    let mut de = Deserializer::new(&buf[..]);

    match Deserialize::deserialize(&mut de) {
        Ok(idx) => Ok(idx),
        Err(_) => Err(IndexerError::new("Failed to deserialize file buffer")),
    }
    */

    Err(IndexerError::new("Broken dependency"))
}

fn cmp_term_pair(a: &TermIdPair, b: &TermIdPair) -> Ordering {
    if a.0 >= b.0 {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}
