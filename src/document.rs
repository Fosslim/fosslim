use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{ Error, ErrorKind, BufReader };

use serde_json;
use serde_json::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: usize, // documents id on the index
    pub label: String, // spdx id for license aka docment class to predict
    pub text: String, // original text
    pub tf: HashMap<String, u32> // term Count/frequency map
}

impl Document {
    pub fn new(id: usize, label: String, text: String) -> Document {
        Document {
            id: id,
            label: label,
            text: text,
            tf: HashMap::new()
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

pub fn from_json_file(file_path: PathBuf) -> Result<Document, Error> {
    if let Ok(file) = File::open(file_path) {
        parse_from_file(file)

    } else {
        Err(Error::new(ErrorKind::NotFound, "No such file"))
    }
}

pub fn parse_from_file(file: File) -> Result<Document, Error> {
    let mut rdr = BufReader::new(file);
    let val:Value = serde_json::from_reader(rdr).expect("Failed to parse the file");
    let lic = val.as_object().expect("Failed to unpack JSON hashmap");

    let temp_doc = Document::new(
        0,
        lic["licenseId"].as_str().unwrap_or("").to_string(),
        lic["licenseText"].as_str().unwrap_or("").to_string()
    );

    Ok(temp_doc)
}

// reads and parses all documents from Path and turns them into Document
// ps: it returns error when the folder includes anything else
pub fn read_folder(path: &Path) -> Result<Vec<Document>, Error> {
    let mut docs: Vec<Document> = Vec::new();

    // iterate over files and build docs and add them into index
    for entry in path.read_dir().expect("read_dir failed") {

        if let Ok(metadata) = entry {
            // add new document into index only if parsing was successful
            match from_json_file(metadata.path()) {
                Ok(doc) => docs.push(doc),
                Err(e)  => {
                    println!("Failed to parse: {:?}", metadata.path());
                    return Err(e)
                }
            }
        }
    };

    Ok(docs)
}