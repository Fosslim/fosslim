use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

type BoxedError = Box<dyn std::error::Error>;

#[derive(Debug, Clone)]
pub struct Document {
    pub id: usize,                // documents id on the index
    pub label: String,            // spdx id for license aka docment class to predict
    pub text: String,             // original text
    pub tf: HashMap<String, u32>, // term Count/frequency map
}

impl Document {
    pub fn new(id: usize, label: String, text: String) -> Document {
        Document {
            id: id,
            label: label,
            text: text,
            tf: HashMap::new(),
        }
    }

    // goes over termvector and counts each of those frequencies
    pub fn add_tf(&mut self, tokens: &Vec<String>) {
        for token in tokens.iter() {
            let counter = self.tf.entry(token.to_string()).or_insert(0);
            *counter += 1u32
        }
    }
}

pub fn from_json_file(file_path: PathBuf) -> Result<Document, BoxedError> {
    match File::open(file_path) {
        Ok(file) => parse_from_file(file),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn parse_from_file(file: File) -> Result<Document, BoxedError> {
    let rdr = BufReader::new(file);
    let lic: serde_json::Value = serde_json::from_reader(rdr)?;
    let license_label = lic["licenseId"].as_str().unwrap_or("parseErr");
    let license_text = lic["licenseText"].as_str().unwrap_or("");
    let temp_doc = Document::new(0, license_label.to_owned(), license_text.to_owned());

    Ok(temp_doc)
}

// reads and parses all documents from Path and turns them into Document
// ps: it returns error when the folder includes anything else
pub fn read_folder(path: &Path) -> Result<Vec<Document>, BoxedError> {
    let mut docs: Vec<Document> = Vec::new();

    // iterate over files and build docs and add them into index
    for entry in path.read_dir().expect("read_dir failed") {
        if let Ok(metadata) = entry {
            // add new document into index only if parsing was successful
            match from_json_file(metadata.path()) {
                Ok(doc) => docs.push(doc),
                Err(e) => {
                    println!("Failed to parse: {:?}", metadata.path());
                    return Err(e);
                }
            }
        }
    }

    Ok(docs)
}
