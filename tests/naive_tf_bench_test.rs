/* find bencher which is not on nightly
#![feature(test)]
#![cfg(test)]

extern crate test;
extern crate fosslim;

use std::fs::File;
use test::Bencher;

use fosslim::document;
use fosslim::index;
use fosslim::naive_tf;

// on console: cargo test test_bench_naive_tf -- --bench
#[bench]
fn test_bench_naive_tf_from_index(b: &mut Bencher){
    let data_path = "tests/fixtures/licenses";

    print!("Building index...");
    // bench:     383,496 ns/iter (+/- 38,006)
    let idx = index::build_from_path(data_path).expect("Failed to build test index");
    println!("Done");

    b.iter(||{
        naive_tf::from_index(&idx);
    });
}


// on console: cargo test test_bench -- --bench
#[bench]
fn test_bench_naive_tf_make_term_vector(b: &mut Bencher) {
    let data_path = "tests/fixtures/licenses";

    print!("Building index...");
    let idx = index::build_from_path(data_path).expect("Failed to build test index");
    println!("Done");

    // build model
    print!("Building the test model...");
    let mdl = naive_tf::from_index(&idx);
    println!("Done");

    let fp = File::open("tests/fixtures/licenses/MIT.json").expect("Failed to open test file");
    let doc = document::parse_from_file(fp).expect("Failed to build document");
    let tokens = fosslim::tokenizer::tokenize_whitespace(doc.text);

    // 3_190_452 ns  ==> BAD!!, 373_761ns => OK
    b.iter(|| naive_tf::make_term_vector(&mdl.terms, &tokens));
}

*/
