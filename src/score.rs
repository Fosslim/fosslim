use std::collections::HashSet;
use std::iter::FromIterator;
use std::hash::Hash;
use std::cmp::Ordering;
use std::f32;

type TermVector = Vec<u8>;

#[derive(Clone)]
pub struct Score {
    pub doc_id: usize,
    pub score : f32,
    pub label : Option<String>
}

impl Score {
    pub fn new(doc_id: usize, score: f32) -> Score {
        Score {
            doc_id: doc_id,
            score: score,
            label: None
        }
    }
}

impl Eq for Score {}

impl Ord for Score {
    fn cmp(&self, other: &Score) -> Ordering {
        let diff = self.score - other.score ;

        if  diff.is_sign_positive() && diff.abs() > f32::EPSILON {
            Ordering::Greater
        } else if diff.is_sign_negative() && diff.abs() > f32::EPSILON {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Score) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Score {
    fn eq(&self, other: &Score) -> bool {
        self.score == other.score
    }
}

#[test]
fn test_score_comparison(){
    let s1 = Score::new(0, 0.5);
    let s2 = Score::new(1, 0.7);
    let s3 = Score::new(2, 0.7);

    assert!(s1 < s2);
    assert!(s2 > s1);
    assert!(s2 == s3);
}



pub fn jaccard(t1: TermVector, t2: TermVector) -> f32 {
    if t1.len() != t2.len() { return 0.0; }
    if t1.len() == 0 { return 0.0; }

    // count how many terms are common;
    let n_common = (0..t1.len())
        .fold(0.0, |acc, i|{
            if t1[i] == t2[i] { acc + 1.0} else { acc }
        });

    let total_size = (t1.len() + t2.len()) as u16;
    let res = n_common  / ( f32::from(total_size) - n_common);

    res
}

pub fn jaccard_set<T: Eq + Hash >(s1: &HashSet<T>, s2: &HashSet<T>) -> f32 {
    match s1.union(s2).count() {
        0 => 0.0,
        t => ( s1.intersection(s2).count() as f32) / ( t as f32)
    }
}

#[test]
fn test_jaccard_set(){
    let s1 = HashSet::from_iter(vec![1,2,3,4]);
    let s2 = HashSet::from_iter(vec![3,4,5,6]);
    let s3 = HashSet::from_iter(vec![5,6,7,8]);

    assert_eq!(1.0, jaccard_set(&s1, &s1));
    assert!(0.3 < jaccard_set(&s1, &s2));
    assert_eq!(0.0, jaccard_set(&s1, &s3));
}