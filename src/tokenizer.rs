

pub fn tokenize_whitespace(text: String) -> Vec<String> {
    let tokens: Vec<String> = text.split_whitespace().map(|t| t.to_string() ).collect();

    tokens
}