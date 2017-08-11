use std::collections::HashMap;

pub struct Document {
    pub id: usize,     // documents id on the index
    pub label: &'static str,  // spdx id for license aka docment class to predict
    pub text: &'static str,   // original text
    pub tcm: HashMap< &'static str, u32 > // term Count map
}

impl Document {
    pub fn new(id: usize, label: &'static str, text: &'static str) -> Document {
        Document {
            id: id,
            label: label,
            text: text,
            tcm: HashMap::new()
        }
    }

    // splits document text into tokens and saves result its on tcm field
    pub fn tokenize(&mut self){
        let tokens = self.text.split_whitespace();

        for token in tokens {
            let counter = self.tcm.entry(token).or_insert(0);
            *counter += 1u32
        }
    }
}
