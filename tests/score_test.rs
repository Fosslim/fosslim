extern crate fosslim;

use fosslim::score;

#[test]
fn test_score_jaccard_all_different() {
    let res = score::jaccard( vec![0,0], vec![1, 1] );
    assert_eq!(0.0, res);
}

#[test]
fn test_score_jaccard_all_same() {
    let res = score::jaccard( vec![1,1], vec![1, 1] );
    assert_eq!(1.0, res);
}


#[test]
fn test_score_jaccard_only_half_matching() {
    let res = score::jaccard( vec![0,0], vec![1, 0] );
    assert!(0.33 < res);
    assert!(0.35 > res);

    let res = score::jaccard( vec![1, 1, 1, 1], vec![1, 0, 0, 1]);
    assert!(0.33 < res);
    assert!(0.35 > res);
}
