extern crate fosslim;

use fosslim::index;
use fosslim::document::Document;

#[test]
fn test_index_add_term() {
    let mut idx = index::Index::new();

    assert_eq!(0, idx.n_terms);
    idx.add_term("abc");
    assert_eq!(1, idx.n_terms);
}

#[test]
fn test_index_add_doc(){
    let mut idx = index::Index::new();

    assert_eq!(0, idx.n_docs);

    let doc1 = Box::new(Document::new(0, "doc1", "text1"));
    idx.add(doc1);
    assert_eq!(1, idx.n_docs);

    let doc2 = Box::new(Document::new(1, "doc2", "text2"));
    idx.add(doc2);
    assert_eq!(2, idx.n_docs);
}

#[test]
fn test_index_doc_new_document() {
    let mut idx = index::Index::new();
    let mut doc = Box::new(Document::new(0, "test", "brown fox"));
    doc.tokenize(); //TODO: move into Index::add function

    idx.add(doc);
    assert_eq!(1, idx.n_docs);

    idx.index_doc(0);

    let docs = idx.get_docs_by_term("fox").expect("failed to fetch documents for test term");
    assert_eq!(1, docs.len());
    assert_eq!("test", docs[0].label)
}
