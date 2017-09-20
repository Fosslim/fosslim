extern crate fosslim;

use fosslim::tokenizer;

#[test]
fn test_tokenizer_tokenize_whitespace(){
    let txt = "Quick brown fox jumps".to_string();

    let tkns = tokenizer::tokenize_whitespace(txt);
    assert_eq!(4, tkns.len());

}

#[test]
fn test_tokenize_overlapping_ngraps_with_empty_string(){
    let res = tokenizer::tokenize_overlapping_ngrams("".to_string(), 1);
    assert!(res.is_empty());

    let res = tokenizer::tokenize_overlapping_ngrams("    ".to_string(), 1);
    assert!(res.is_empty());
}

#[test]
fn test_tokenize_overlapping_ngrams_with_3tokens(){
    let test_txt = "The quick brown fox jumps over the lazy dog";
    let res = tokenizer::tokenize_overlapping_ngrams(test_txt.to_string(), 3);

    assert_eq!(7, res.len());
    assert_eq!("The quick brown".to_string(), res[0]);
    assert_eq!("quick brown fox".to_string(), res[1]);
    assert_eq!("brown fox jumps".to_string(), res[2]);
    assert_eq!("fox jumps over".to_string(), res[3]);
    assert_eq!("jumps over the".to_string(), res[4]);
    assert_eq!("over the lazy".to_string(), res[5]);
    assert_eq!("the lazy dog".to_string(), res[6]);
}

#[test]
fn test_tokenize_overlapping_ngrams_with_various_separators(){
    let test_text = "1,2,3 done;he said: next.";
    let res = tokenizer::tokenize_overlapping_ngrams(test_text.to_string(), 2);

    assert_eq!(7, res.len());
    assert_eq!("1 2".to_string(), res[0]);
    assert_eq!("2 3".to_string(), res[1]);
    assert_eq!("3 done".to_string(), res[2]);
    assert_eq!("done he".to_string(), res[3]);
    assert_eq!("he said".to_string(), res[4]);
    assert_eq!("said next".to_string(), res[5]);
    assert_eq!("next".to_string(), res[6]);
}