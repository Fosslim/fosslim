extern crate fosslim;

use fosslim::index::Index;
use fosslim::document::Document;
use fosslim::jaccard;

fn create_test_index() -> Index {
    let mut idx = Index::new();
    let doc1 = Document::new(0, "doc1".to_string(), "brown fox".to_string());
    idx.add(doc1);
    idx.index_doc(0);
    assert_eq!(1, idx.n_docs);
    assert_eq!(2, idx.n_terms);

    let doc2 = Document::new(0, "doc2".to_string(), "lazy dog".to_string());
    idx.add(doc2);
    idx.index_doc(1);
    idx
}

#[test]
fn test_jaccard_init_new() {
    let mdl = jaccard::JaccardModel::new(4, 2);

    assert_eq!(4, mdl.terms.capacity());
    assert_eq!(2, mdl.labels.capacity());
}

#[test]
fn test_jaccard_from_example_index() {
    let idx = create_test_index();
    let mdl = jaccard::from_index(&idx);

    assert_eq!(2, mdl.labels.len());
    assert_eq!(4, mdl.terms.len());
    assert_eq!(2, mdl.word_bag.len());
    assert_eq!(4, mdl.word_bag[0].len());

    assert_eq!("brown".to_string(), mdl.terms[0]);
    assert_eq!(4, mdl.word_bag[0].len());
    assert_eq!(1, mdl.word_bag[0][0]); //word brown should be in the first doc selected
    assert_eq!(0, mdl.word_bag[1][0]); // but not in document.2

    assert_eq!("dog".to_string(), mdl.terms[1]);
    assert_eq!(1, mdl.word_bag[0][1]); // word fox should be in the first doc
    assert_eq!(0, mdl.word_bag[1][1]); // but not in document.2

    assert_eq!("fox".to_string(), mdl.terms[2]);
    assert_eq!(0, mdl.word_bag[0][2]); // word lazy should be in the first doc
    assert_eq!(1, mdl.word_bag[1][2]); // but not in document.2

    assert_eq!("lazy".to_string(), mdl.terms[3]);
    assert_eq!(0, mdl.word_bag[0][3]); // word dog should be in the first doc
    assert_eq!(1, mdl.word_bag[1][3]); // but not in document.2
}

