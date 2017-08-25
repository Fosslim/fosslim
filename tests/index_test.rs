extern crate fosslim;

use fosslim::index;
use fosslim::document::Document;

#[test]
fn test_index_add_term() {
    let mut idx = index::Index::new();

    assert_eq!(0, idx.n_terms);
    idx.add_term("abc".to_string());
    assert_eq!(1, idx.n_terms);
}

#[test]
fn test_index_add_doc(){
    let mut idx = index::Index::new();

    assert_eq!(0, idx.n_docs);
    assert_eq!(0, idx.n_terms);

    let doc1 = Document::new(0, "doc1".to_string(), "text1".to_string());
    idx.add(doc1);
    assert_eq!(1, idx.n_docs);
    assert_eq!(0, idx.n_terms);

    let doc2 = Document::new(1, "doc2".to_string(), "text2".to_string());
    idx.add(doc2);
    assert_eq!(2, idx.n_docs);

}

#[test]
fn test_index_doc_new_document() {
    let mut idx = index::Index::new();
    let mut doc = Document::new(0, "test".to_string(), "brown fox".to_string());

    idx.add(doc);
    assert_eq!(1, idx.n_docs);
    assert_eq!(0, idx.n_terms);

    idx.index_doc(0);
    assert_eq!(1, idx.n_docs);
    assert_eq!(2, idx.n_terms);

    let term = "fox".to_string();
    let docs = idx.get_docs_by_term(term).expect("failed to fetch documents for test term");
    assert_eq!(1, docs.len());
    assert_eq!("test".to_string(), docs[0].label)
}


#[test]
fn test_index_get_term_index(){
    let mut idx = index::Index::new();
    let mut doc = Document::new(0, "test".to_string(), "yellow cat".to_string());

    idx.add(doc);
    idx.index_doc(0);
    assert_eq!(1, idx.n_docs);
    assert_eq!(2, idx.n_terms);

    let term_docs = idx.get_term_index();
    assert_eq!(2, term_docs.len());
    assert_eq!(0, term_docs[0].0);
    assert_eq!(vec![0], term_docs[0].1)
}

#[test]
fn test_index_build_from_path() {
    let test_path = "tests/fixtures/licenses";
    let res = index::build_from_path(test_path);

    assert!(res.is_ok());

    if let Ok(idx) = res {
        assert_eq!(2, idx.n_docs);
        assert!(idx.n_terms > 0);

        let docs = idx.get_docs_by_term("MIT".to_string()).expect("Not docs with MIT");
        assert_eq!(1, docs.len());
    }
}

#[test]
fn test_index_save_and_load(){
    let target_path = "temp/index1.msgpack";

    // build test index
    let mut orig_idx = index::Index::new();
    let doc1 = Document::new(0, "doc1".to_string(), "text1".to_string());
    orig_idx.add(doc1).ok();
    assert_eq!(1, orig_idx.n_docs);

    // dump into the file
    let res = index::save(&orig_idx, target_path);
    assert!(res.is_ok());

    // read it from file
    let res = index::load(target_path);
    assert!(res.is_ok());

    // check correctness of index
    let idx = res.ok().unwrap();
    assert_eq!(1, idx.n_docs);
}