use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

use seahash;

use index::Index;
use tokenizer;
use score::{self, Score};
use document::Document;

type Fingerprint = HashSet<u64>;

#[derive(Clone, Debug)]
pub struct FingerNgram {
    pub n_docs: usize,
    pub ngram_size: usize, // how many words should NGram include
    pub p_modder: u64, // is hash divisible with that constant?
    labels: Vec<String>,
    hash_bag: Vec<Fingerprint>
}

impl FingerNgram {
    pub fn new(n_docs: usize, ngram_size: usize, p_modder: u64) -> FingerNgram {
        FingerNgram {
            n_docs: n_docs,
            ngram_size: ngram_size,
            p_modder: p_modder,
            labels: Vec::with_capacity(n_docs),
            hash_bag: Vec::with_capacity(n_docs)
        }
    }

    pub fn add_fingerprint(&mut self, label: String, fingerprint: Fingerprint) -> Option<usize> {
        if self.labels.len() == self.hash_bag.len() {
            self.labels.push(label);
            self.hash_bag.push(fingerprint);

            Some(self.labels.len())
        } else {
            None
        }
    }

    pub fn fingerprint(&self, txt: String) -> Fingerprint {
        let ngrams = tokenizer::tokenize_overlapping_ngrams(txt, self.ngram_size);

        ngrams.into_iter()
            .map(|t| hash_text(t) )  //calc hash value for each ngram
            .filter(|&h| h.wrapping_rem( self.p_modder ) == 0) // select only hashes which divide by p_modder
            .collect::<Fingerprint>()                          // collect results into final result
    }

    pub fn get_fingerprints(&self) -> HashMap<String, Fingerprint> {
        let mut table = HashMap::with_capacity(self.labels.len());

        let mut pos = 0;
        for label in self.labels.iter() {
            table.insert(label.clone().to_string(), self.hash_bag[pos].clone() );
            pos += 1;
        }

        table
    }

    pub fn rank(&self, query: Fingerprint) -> Vec<Score> {
        let mut scores:Vec<Score> = Vec::with_capacity(self.n_docs);

        for pos in 0..self.n_docs {
            let sim = score::jaccard_set(&self.hash_bag[pos], &query);
            let doc_label = match self.labels.get(pos) {
                Some(lbl) => lbl.clone(),
                None       => "".to_string()
            };

            let score = Score {
                doc_id: pos,
                score: sim,
                label: Some(doc_label)
            };

            scores.push(score);
        }

        scores.sort_by(|a,b| b.cmp(a) );
        scores
    }

    pub fn match_document(&self, target_doc: &Document) -> Option<Score> {
        let query = self.fingerprint(target_doc.text.clone());

        let scores = self.rank(query);
        if scores.len() > 0 {
            Some(scores[0].clone())
        } else {
            None
        }
    }
}

pub fn hash_text(txt: String) -> u64 {
    let mut hasher = seahash::SeaHasher::new(); // TODO: research effects of rolling hash vs current ad-hoc
    txt.hash(&mut hasher);
    hasher.finish()
}

// builds a new model from the Index
pub fn from_index(idx: &Index) -> FingerNgram {
    let mut mdl = FingerNgram::new(idx.n_docs, 5, 4);


    for doc in idx.get_documents().iter() {
        let fingerprint = mdl.fingerprint(doc.text.clone());
        mdl.add_fingerprint(doc.label.clone(), fingerprint);
    }

    mdl
}

