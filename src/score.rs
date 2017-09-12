use std::cmp::Ordering;
use std::f32;

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

