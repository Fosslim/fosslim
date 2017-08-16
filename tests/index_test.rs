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

    let doc1 = Document::new(0, "doc1".to_string(), "text1".to_string());
    idx.add(doc1);
    assert_eq!(1, idx.n_docs);

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

    idx.index_doc(0);
    let term = "fox".to_string();
    let docs = idx.get_docs_by_term(term).expect("failed to fetch documents for test term");
    assert_eq!(1, docs.len());
    assert_eq!("test".to_string(), docs[0].label)
}
