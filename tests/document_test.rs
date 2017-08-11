extern crate fosslim;

use fosslim::document::Document;

#[test]
fn test_document_tokenizer(){
    let mut doc = Document::new(0, "MIT", "AAA BCD AAA DEF" );
    doc.tokenize();

    assert_eq!(2, doc.tcm[&"AAA"]);
    assert_eq!(1, doc.tcm[&"BCD"]);
    assert_eq!(1u32, doc.tcm[&"DEF"]);
    assert_eq!(None, doc.tcm.get("XYZ"));
}