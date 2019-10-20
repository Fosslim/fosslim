#![cfg(test)]

extern crate fosslim;

use fosslim::document;
use std::fs::File;
/*
use test::Bencher;

#[bench]
fn test_bench_from_json_file(b: &mut Bencher) {
    // ~ 2.08M ns without BufReader, ~1M with BufReader;
    b.iter(|| {
        let fp = File::open("tests/fixtures/licenses/MIT.json").expect("Failed to open test file");
        document::parse_from_file(fp).expect("Failed to parse from test file");
    })
}
*/
