use std::collections::HashMap;

#[derive(Clone)]
pub struct Document<'a> {
    pub id: usize, // documents id on the index
    pub label: &'a str, // spdx id for license aka docment class to predict
    pub text: &'a str, // original text
    pub tcm: HashMap<&'a str, u32>, // term Count map
}

impl<'a> Document<'a> {
    pub fn new(id: usize, label: &'a str, text: &'a str) -> Document<'a> {
        Document {
            id: id,
            label: label,
            text: text,
            tcm: HashMap::new(),
        }
    }

    // splits document text into tokens and saves result its on tcm field
    pub fn tokenize(&mut self) {
        let tokens = self.text.split_whitespace();

        for token in tokens {
            let counter = self.tcm.entry(&token).or_insert(0);
            *counter += 1u32
        }
    }
}
