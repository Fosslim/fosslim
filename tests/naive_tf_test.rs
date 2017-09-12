extern crate fosslim;

use fosslim::index::Index;
use fosslim::document::Document;
use fosslim::naive_tf;
use fosslim::tokenizer;

fn create_test_index() -> Index {
    let mut idx = Index::new();
    let doc1 = Document::new(0, "doc1".to_string(), "brown fox".to_string());
    idx.add(doc1);

    let doc2 = Document::new(0, "doc2".to_string(), "lazy dog".to_string());
    idx.add(doc2);

    idx.index();
    assert_eq!(2, idx.n_docs);
    assert_eq!(4, idx.n_terms);

    idx
}

#[test]
fn test_naive_tf_init_new() {
    let mdl = naive_tf::NaiveTFModel::new(4, 2);

    assert_eq!(4, mdl.terms.capacity());
    assert_eq!(2, mdl.labels.capacity());
}

#[test]
fn test_naive_tf_from_example_index() {
    let idx = create_test_index();
    let mdl = naive_tf::from_index(&idx);

    assert_eq!(2, mdl.labels.len());
    assert_eq!(4, mdl.terms.len());
    assert_eq!(2, mdl.word_bag.len());
    assert_eq!(4, mdl.word_bag[0].len());

    assert_eq!("brown".to_string(), mdl.terms[0]);
    assert_eq!(4, mdl.word_bag[0].len());
    assert_eq!(1, mdl.word_bag[0][0]); //word brown should be in the first doc selected
    assert_eq!(0, mdl.word_bag[1][0]); // but not in document.2

    assert_eq!("fox".to_string(), mdl.terms[1]);
    assert_eq!(1, mdl.word_bag[0][1]); // word fox should be in the first doc
    assert_eq!(0, mdl.word_bag[1][1]); // but not in document.2

    assert_eq!("lazy".to_string(), mdl.terms[2]);
    assert_eq!(0, mdl.word_bag[0][2]); // word lazy should be in the first doc
    assert_eq!(1, mdl.word_bag[1][2]); // but not in document.2

    assert_eq!("dog".to_string(), mdl.terms[3]);
    assert_eq!(0, mdl.word_bag[0][3]); // word dog should be in the first doc
    assert_eq!(1, mdl.word_bag[1][3]); // but not in document.2
}


#[test]
fn test_naive_tf_make_term_vector_is_consistent() {
    let idx = create_test_index();
    let mdl = naive_tf::from_index(&idx);
    let docs = idx.get_documents();
    let doc_terms = tokenizer::tokenize_whitespace(docs[0].text.clone());
    let query_vec = naive_tf::make_term_vector(&mdl.terms, &doc_terms);

    assert_eq!(mdl.word_bag[0], query_vec)
}


#[test]
fn test_naive_tf_rank_doc1_first(){
    let idx = create_test_index();
    let mdl = naive_tf::from_index(&idx);

    assert_eq!(2, mdl.labels.len());
    assert_eq!(4, mdl.terms.len());
    assert_eq!(2, mdl.word_bag.len());
    assert_eq!(4, mdl.word_bag[0].len());

    // brown = 1, fox = 1, lazy = 0, dog = 0
    let scores = mdl.rank(vec![1, 1, 0, 0]);
    println!("score: {} > {}", scores[0].score, scores[1].score);
    println!("Terms: {:?}", mdl.terms);

    assert_eq!(2, scores.len());
    assert_eq!(1.0, scores[0].score);
    assert_eq!(0, scores[0].doc_id);
    assert_eq!(Some("doc1".to_string()), scores[0].label);

    assert!(0.1 > scores[1].score);
    assert_eq!(1, scores[1].doc_id);
    assert_eq!(Some("doc2".to_string()), scores[1].label);
}