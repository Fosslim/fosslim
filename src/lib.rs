#![feature(pattern)]

extern crate serde_json;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate rmp_serde;
extern crate seahash;

pub mod document;
pub mod index;
pub mod tokenizer;
pub mod score;

// models
pub mod naive_tf;
pub mod finger_ngram;
