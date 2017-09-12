#![feature(test)]
#![cfg(test)]

extern crate test;
extern crate fosslim;

use std::fs::File;
use test::Bencher;

use fosslim::document;
use fosslim::index;
use fosslim::jaccard;

#[bench]
fn test_bench_from_index(b: &mut Bencher){
    let data_path = "tests/fixtures/licenses";
    let fixtures_path = "tests/fixtures/test_licenses";

    print!("Building index...");
    let idx = index::build_from_path(data_path).expect("Failed to build test index");
    println!("Done");

    b.iter(||{
        jaccard::from_index(&idx);
    });
}


// on console: cargo test test_bench -- --bench
#[bench]
fn test_bench_make_term_vector(b: &mut Bencher) {
    let data_path = "tests/fixtures/licenses";
    let fixtures_path = "tests/fixtures/test_licenses";

    print!("Building index...");
    let idx = index::build_from_path(data_path).expect("Failed to build test index");
    println!("Done");

    // build model
    print!("Building the test model...");
    let mdl = jaccard::from_index(&idx);
    println!("Done");

    let fp = File::open("tests/fixtures/licenses/MIT.json").expect("Failed to open test file");
    let doc = document::parse_from_file(fp).expect("Failed to build document");
    let tokens = fosslim::tokenizer::tokenize_whitespace(doc.text);

    // 3_190_452 ns  ==> BAD!!, 373_761ns => OK
    b.iter(|| jaccard::make_term_vector(&mdl.terms, &tokens));
}



