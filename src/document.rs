use std::collections::HashMap;
use std::path::PathBuf;
use std::fs::File;
use std::io::{ Error, ErrorKind };

use serde_json;
use serde_json::Value;

#[derive(Clone)]
pub struct Document {
    pub id: usize, // documents id on the index
    pub label: String, // spdx id for license aka docment class to predict
    pub text: String, // original text
    pub tcm: HashMap<String, u32> // term Count map
}

impl Document {
    pub fn new(id: usize, label: String, text: String) -> Document {
        Document {
            id: id,
            label: label,
            text: text,
            tcm: HashMap::new()
        }
    }

    // splits document text into tokens and saves result its on tcm field
    pub fn tokenize(&mut self) {
        let txt = self.text.clone();

        for token in txt.split_whitespace() {
            let counter = self.tcm.entry(token.to_string()).or_insert(0);
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
    let val:Value = serde_json::from_reader(file).expect("Failed to parse the file");
    let lic = val.as_object().expect("Failed to unpack JSON hashmap");

    let temp_doc = Document::new(
        0,
        lic["licenseId"].as_str().unwrap_or("").to_string(),
        lic["licenseText"].as_str().unwrap_or("").to_string()
    );

    Ok(temp_doc)
}