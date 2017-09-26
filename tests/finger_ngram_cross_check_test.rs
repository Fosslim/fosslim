extern crate fosslim;

use fosslim::finger_ngram;
use fosslim::index;

// for executing this tests with output

// cargo test test_finger_ngram_cross_check -- --nocapture

#[test]
fn test_finger_ngram_cross_check(){
    let data_path = "data/licenses";

    print!("Building index...");
    let idx = index::build_from_path(data_path).expect("Failed to build test index");
    println!("Done");

    print!("Building the test model...");
    let mdl = finger_ngram::from_index(&idx);
    println!("Done");

    let mut true_pos = 0;
    let mut false_neg = 0;
    let mut n_docs = 0;
    let mut fails  = 0;

    println!("Index. N_Terms: {}, N_Docs: #{}", idx.n_terms, idx.n_docs);
    println!("Correct?|Expected|Result|Score");
    for doc in idx.get_documents().iter() {
        n_docs += 1;

        if let Some(score) = mdl.match_document(doc) {
            let res_label = score.label.unwrap_or("".to_string());

            if doc.label == res_label {
                println!("+| {} | {} | {}", doc.label, res_label, score.score);
                true_pos += 1;
            } else {
                println!("-| {} | {} | {}", doc.label, res_label, score.score);
                false_neg += 1;
            }


        } else {
            println!("{} => NONE", doc.label);
            fails += 1;
        }
    }

    let accuracy = (true_pos as f32) / (n_docs as f32);
    println!("#-- Summary\n\t Matched #{}\n\tCorrect: {}", n_docs, true_pos);
    println!("\tFalse negatives: {}\n\tFails: {}\n\tAccuracy: {}", false_neg, fails, accuracy);
}
