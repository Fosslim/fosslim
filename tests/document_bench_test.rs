#![feature(test)]
#![cfg(test)]

extern crate test;
extern crate fosslim;

use std::fs::File;
use test::Bencher;
use fosslim::document;

#[bench]
fn test_bench_from_json_file(b: &mut Bencher){
    // ~ 2.08M ns
    b.iter(||{
        let fp = File::open("tests/fixtures/licenses/MIT.json").expect("Failed to open test file");
        document::parse_from_file(fp).expect("Failed to parse from test file");
    })
}