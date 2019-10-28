extern crate serde_json;

//extern crate rmp_serde;
extern crate seahash;
//extern crate serde;

pub mod document;
pub mod index;
pub mod score;
pub mod shingler;
pub mod tokenizer;

// models
pub mod finger_ngram;
pub mod minhash;
pub mod naive_tf;
