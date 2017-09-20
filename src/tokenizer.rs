use std::str::pattern::Pattern;

const SEPARATORS: &'static str = ",.;:!?";

pub fn tokenize_whitespace(text: String) -> Vec<String> {
    let tokens: Vec<String> = text.split_whitespace().map(|t| t.to_string() ).collect();

    tokens
}

/// It splits original text into word ngrams, which are overlapping by  (n-1) elements
pub fn tokenize_overlapping_ngrams(text: String, n: usize) -> Vec<String> {
    let mut ngrams: Vec<String> = Vec::new();

    let mut curr_token = String::new();
    let mut curr_ngram: Vec<String> = Vec::with_capacity(n);

    for ch in text.chars() {
        if ch.is_alphanumeric() {
            // add it to current token
            curr_token.push(ch);

        } else if ch.is_whitespace() || ch.is_contained_in(SEPARATORS) {
            if !curr_token.is_empty(){

                curr_ngram.push(curr_token);
                curr_token = String::new();
            }

        };

        // make some space when curr_ngram is full
        // by joining them into string and putting onto ngrams list
        if n == curr_ngram.len() {
            let ngram = curr_ngram.join(" ");
            ngrams.push(ngram);

            curr_ngram = curr_ngram.split_off(1);
        }
    }

    //dump leftover ngrams tokens into the list of ngrams
    if !curr_token.is_empty() {
        curr_ngram.push(curr_token);
    }

    if !curr_ngram.is_empty(){
        ngrams.push(curr_ngram.join(" "))
    }

    ngrams
}