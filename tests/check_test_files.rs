extern crate fosslim;

use std::path::Path;

use fosslim::index;
use fosslim::jaccard;
use fosslim::document;
use fosslim::score::Score;

// for executing this tests with output
// cargo test test_check_files  -- --nocapture

#[test]
fn test_check_files_with_fixtures(){
    let data_path = "tests/fixtures/licenses";
    let fixtures_path = "tests/fixtures/test_licenses";

    print!("Building index...");
    let idx = index::build_from_path(data_path).expect("Failed to build test index");
    println!("Done");

    // build model
    print!("Building the test model...");
    let mdl = jaccard::from_index(&idx);
    println!("Done");
    let mut true_pos = 0;
    let mut false_neg = 0;
    let mut n_docs = 0;
    let mut fails  = 0;

    println!("Correct?|Expected|Result|Score");
    // iterate over all files in tests/test_licenses and check result between doc.label and match
    let examples_path = Path::new(fixtures_path);
    for doc in document::read_folder(examples_path).expect("Failed to read folder").iter() {
        n_docs += 1;

        if let Some(score) = mdl.match_document(doc) {
            let res_label = score.label.unwrap_or("".to_string());

            if doc.label == res_label {
                println!("+|{}|{}|{}", doc.label, res_label, score.score);
                true_pos += 1;
            } else {
                println!("-|{}|{}|{}", doc.label, res_label, score.score);
                false_neg += 1;
            }


        } else {
            println!("{} => NONE", doc.label);
            fails += 1;
        }
    }

    let accuracy:f32 = (true_pos as f32) / (n_docs as f32);
    println!("#-- Summary\n\t Matched #{}\n\tCorrect: {}", n_docs, true_pos);
    println!("\tFalse negatives: {}\n\tFails: {}\n\tAccuracy: {}", false_neg, fails, accuracy);

}