use std::collections::HashSet;
//use std::convert::From;
use std::iter::FromIterator;

use index::Index;
use document::Document;
use score::{self, Score};
use tokenizer;

type WordBag = Vec<Vec<u8>>;
type TermVector = Vec<u8>;

#[derive(Clone, Debug)]
pub struct NaiveTFModel {
    pub terms: Vec<String>, // keeps word vector
    pub labels: Vec<String>, // keeps document label index
    pub word_bag: WordBag
}

fn init_empty_bag(n_docs: usize, n_terms: usize) -> WordBag {
    let mut bag = Vec::with_capacity(n_docs);

    // initialize all subvectors
    for _doc_id in 0..n_docs {
        bag.push( vec![0; n_terms] );
    }

    bag
}

#[test]
fn test_naive_tf_empty_bag(){
    let bag = init_empty_bag(2, 1);

    assert_eq!(2, bag.len());
    assert_eq!(1, bag[0].len());
}

impl NaiveTFModel {
    pub fn new(n_terms: usize, n_docs:usize) -> NaiveTFModel {
        NaiveTFModel {
            terms: Vec::with_capacity(n_terms),
            labels: Vec::with_capacity(n_docs),
            word_bag: init_empty_bag(n_docs, n_terms)
        }
    }

    // ranks documents in Word vector, compared to query vector
    pub fn rank(&self, query_vec: TermVector) -> Vec<Score> {
        let mut scores : Vec<Score> = Vec::with_capacity(self.labels.len());
        let mut doc_id = 0 as usize;

        for doc_term_vec in self.word_bag.iter() {

            let sim = score::jaccard(doc_term_vec.clone(), query_vec.clone());
            let score = Score {
                score: sim,
                doc_id: doc_id,
                label:  Some(self.labels[doc_id].clone())
            };

            scores.push(score);
            doc_id += 1;
        }

        scores.sort_by(|a,b| b.cmp(a) );
        scores
    }

    pub fn match_document(&self, target_doc: &Document) -> Option<Score> {
        // tokenize doc
        let doc_tokens = tokenizer::tokenize_whitespace(target_doc.text.clone());
        // build term vector from doc
        let query_vec = make_term_vector(&self.terms, &doc_tokens);
        // calc scores for each doc

        let scores = NaiveTFModel::rank(self, query_vec);
        if scores.len() > 0 {
            Some(scores[0].clone())
        } else {
            None
        }

    }
}

// build a new model from the Index
pub fn from_index(idx: &Index) -> NaiveTFModel {
    let term_vector = idx.get_terms();
    let mut labels = vec!["".to_string(); idx.n_docs];
    let mut bag = init_empty_bag(idx.n_docs, idx.n_terms);

    // init document labels
    for doc_id in 0..idx.n_docs {
        match idx.get_document_label(doc_id as usize) {
            Some(lbl)   => labels[doc_id] = lbl,
            None        => labels[doc_id] = "unspecified".to_string()
        }
    }

    // init word bag from term document index
    for (term_id, doc_ids) in idx.get_term_index().into_iter() {

        // mark that term exists in document
        for &doc_id in doc_ids.iter() {
            bag[doc_id][term_id] = 1;
        }

    }

    NaiveTFModel {
        terms: term_vector,
        labels: labels,
        word_bag: bag
    }
}


// builds term vector (~ row of Wordbag) for
pub fn make_term_vector(terms: &Vec<String>, doc_tokens: &Vec<String>) -> TermVector {
    let mut term_vec = Vec::with_capacity(terms.len());

    let term_set:HashSet<&String> = HashSet::from_iter(doc_tokens.iter());
    for term in terms.iter() {
        if term_set.contains(term) {
            term_vec.push(1);
        } else {
            term_vec.push(0);
        }
    }

    term_vec
}


#[test]
fn test_naive_tf_make_term_vector(){
    let terms = vec!["brown".to_string(), "fox".to_string(), "jumps".to_string()];
    let tkns = vec!["red".to_string(), "fox".to_string().to_string()];

    let tv = make_term_vector(&terms, &tkns);
    assert_eq!(3, tv.len());
    assert_eq!(vec![0, 1, 0], tv);
}