extern crate fosslim;

use fosslim::tokenizer;

#[test]
fn test_tokenizer_tokenize_whitespace(){
    let txt = "Quick brown fox jumps".to_string();

    let tkns = tokenizer::tokenize_whitespace(txt);
    assert_eq!(4, tkns.len());

}