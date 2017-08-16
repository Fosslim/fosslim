extern crate fosslim;

use std::fs::File;

use fosslim::document::{self, Document};

#[test]
fn test_document_tokenizer(){
    let mut doc = Document::new(0, "MIT".to_string(), "AAA BCD AAA DEF".to_string() );
    doc.tokenize();

    assert_eq!(2, doc.tcm["AAA"]);
    assert_eq!(1, doc.tcm["BCD"]);
    assert_eq!(1u32, doc.tcm["DEF"]);
    assert_eq!(None, doc.tcm.get("XYZ"));
}

#[test]
fn test_document_parse_from_file(){
    let fp = File::open("tests/fixtures/licenses/MIT.json").expect("Failed to open test file");
    let res = document::parse_from_file(fp);

    assert!(res.is_ok());
    if let Ok(doc) = res {
        assert_eq!("MIT", doc.label);
        assert_eq!(1077, doc.text.len());
    }
}