#![feature(test)]
#![cfg(test)]

extern crate test;
extern crate fosslim;

use std::fs::File;
use test::Bencher;

use fosslim::document;
use fosslim::index;
use fosslim::finger_ngram;

// on console: cargo test test_bench_finger_ngram -- --bench

#[bench]
fn test_bench_finger_ngram_from_index(b: &mut Bencher){
    let data_path = "tests/fixtures/licenses";

    print!("Building index...");
    let idx = index::build_from_path(data_path).expect("Failed to build test index");
    println!("Done");

    // bench:   2,050,853 ns/iter (+/- 230,371) , not the fastest
    b.iter(||{ finger_ngram::from_index(&idx); });
}

// on console: cargo test test_bench -- --bench
#[bench]
fn test_bench_finger_ngram_make_fingergram(b: &mut Bencher) {
    let data_path = "tests/fixtures/licenses";

    print!("Building index...");
    let idx = index::build_from_path(data_path).expect("Failed to build test index");
    println!("Done");

    // build model
    print!("Building the test model...");
    let mdl = finger_ngram::from_index(&idx);
    println!("Done");

    let fp = File::open("tests/fixtures/licenses/MIT.json").expect("Failed to open test file");
    let doc = document::parse_from_file(fp).expect("Failed to build document");

    // bench:   1,239,363 ns/iter (+/- 164,185)
    b.iter(|| mdl.fingerprint( doc.text.clone() ));
}