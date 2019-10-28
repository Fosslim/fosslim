use std::collections::VecDeque;

const EMPTY_TOKEN: &str = " ";

pub fn shingle(tokens: Vec<String>, n: usize) -> Vec<String> {
    let n_tokens = tokens.len();
    let mut shingles: Vec<String> = Vec::with_capacity(n_tokens);

    let unprocessed_tokens = tokens.clone();
    let mut shingle_queue: VecDeque<_> = unprocessed_tokens.into_iter().take(n).collect();

    shingles.push(join_shingles(&shingle_queue));
    // build shingles
    for cur_token in tokens.iter().skip(n) {
        shingle_queue.pop_front(); // make room
        shingle_queue.push_back(cur_token.clone());

        let cur_shingle = join_shingles(&shingle_queue);
        shingles.push(cur_shingle);
    }

    shingles
}

fn join_shingles<T>(shingles: T) -> String
where
    T: IntoIterator,
    T::Item: ToString,
{
    shingles
        .into_iter()
        .fold("".to_owned(), |acc, token| {
            acc + EMPTY_TOKEN + &token.to_string()
        })
        .trim()
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_test_tokens() -> Vec<String> {
        vec![
            "the".to_owned(),
            "quick".to_owned(),
            "brown".to_owned(),
            "fox".to_owned(),
            "jump".to_owned(),
            "over".to_owned(),
            "lazy".to_owned(),
            "dog".to_owned(),
        ]
    }

    #[test]
    fn test_shingle_with_single() {
        let tokens = build_test_tokens();
        let shingles = shingle(tokens, 1);

        assert_eq!(shingles[0], "the".to_owned());
        assert_eq!(shingles[1], "quick".to_owned());
        assert_eq!(shingles[2], "brown".to_owned());
    }

    #[test]
    fn test_shingle_with_duples() {
        let tokens = build_test_tokens();
        let shingles = shingle(tokens, 2);

        assert_eq!(shingles[0], "the quick".to_owned());
        assert_eq!(shingles[1], "quick brown".to_owned());
        assert_eq!(shingles[2], "brown fox".to_owned());
    }

    #[test]
    fn test_shingle_n_bigger_than_tokens() {
        let tokens = vec!["the".to_owned(), "quick".to_owned()];
        let shingles = shingle(tokens, 3);

        assert_eq!(shingles.len(), 1);
        assert_eq!(shingles[0], "the quick".to_owned());
    }
}
