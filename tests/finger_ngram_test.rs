extern crate fosslim;
//extern crate seahash;

use fosslim::finger_ngram;
use fosslim::index::Index;
use fosslim::document::Document;

fn create_test_index() -> Index {
    let mut idx = Index::new();
    let mit_txt = r#"
    Permission is hereby granted, free of charge, to any person obtaining a copy of this software \
    and associated documentation files (the "Software"), to deal in the Software without restriction,\
    including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense,\
    and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so,\
    subject to the following conditions:\
    "#;
    let doc1 = Document::new(0, "mit".to_string(), mit_txt.to_string());
    idx.add(doc1).unwrap();

    let bsd_txt = r#"
    Redistribution and use in source and binary forms, with or without modification, are permitted
    provided that the following conditions are met:
    1. Redistributions of source code must retain the above copyright notice, this list of conditions
    and the following disclaimer.
    2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions
    and the following disclaimer in the documentation and/or other materials provided with the distribution.
    "#;
    let doc2 = Document::new(0, "bsd2".to_string(), bsd_txt.to_string());
    idx.add(doc2).unwrap();

    idx.index().unwrap();
    assert_eq!(2, idx.n_docs);
    assert_eq!(82, idx.n_terms);

    idx
}

#[test]
fn test_finger_ngram_model_fingerprint(){
    let mdl = finger_ngram::FingerNgram::new(1, 3, 2);

    let test_txt = "the quick brown fox jump over lazy dog".to_string();
    let fingerprint = mdl.fingerprint(test_txt);

    println!("Hash1: {}", finger_ngram::hash_text("quick brown fox".to_string()));
    println!("Fingerprints: {:?}", fingerprint);

    assert_eq!(2, fingerprint.len());
    assert!(fingerprint.contains(&252194812427236350));
    assert!(fingerprint.contains(&7300354117637348550));
}

#[test]
fn test_finger_ngram_from_index(){
    let idx = create_test_index();
    let mdl = finger_ngram::from_index(&idx);

    assert_eq!(5, mdl.ngram_size);
    assert_eq!(4, mdl.p_modder);
    assert_eq!(2, mdl.n_docs);

    let tbl = mdl.get_fingerprints();
    assert_eq!(18, tbl["bsd2"].len());
    assert!(tbl["bsd2"].contains(&12778302951660348072));

    assert_eq!(14, tbl["mit"].len());
    assert!(tbl["mit"].contains(&10642986856394854804));
}

#[test]
fn test_finger_ngram_rank(){
    let idx = create_test_index();
    let mdl = finger_ngram::from_index(&idx);

    let mit_txt = r#"
    Permission is hereby granted, free of charge, to any person obtaining a copy of this software \
    and associated documentation files (the "Software"), to deal in the Software without restriction,\
    including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense,\
    and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so,\
    subject to the following conditions:\
    "#;
    let query = mdl.fingerprint(mit_txt.to_string());
    let scores = mdl.rank(query);

    assert_eq!(2, scores.len());

    assert_eq!(0, scores[0].doc_id);
    assert_eq!(1.0, scores[0].score);
    assert_eq!(Some("mit".to_string()), scores[0].label);

    assert_eq!(1, scores[1].doc_id);
    assert_eq!(0.0, scores[1].score);
    assert_eq!(Some("bsd2".to_string()), scores[1].label);
}

#[test]
fn test_finger_ngram_match_document(){
    let idx = create_test_index();
    let mdl = finger_ngram::from_index(&idx);

    let bsd_txt = r#"
    Redistribution and use in source and binary forms, with or without modification, are permitted
    provided that the following conditions are met:
    1. Redistributions of source code must retain the above copyright notice, this list of conditions
    and the following disclaimer.
    2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions
    and the following disclaimer in the documentation and/or other materials provided with the distribution.
    "#;

    let doc2 = Document::new(0, "bsd2".to_string(), bsd_txt.to_string());

    let res = mdl.match_document(&doc2);
    assert!(res.is_some());

    let m = res.unwrap();
    assert_eq!(1.0, m.score);
    assert_eq!(1, m.doc_id);
    assert_eq!(Some("bsd2".to_string()), m.label);
}