use lazy_static::lazy_static;
use regex::Regex;

pub fn tokenize_text(text: &str) -> Vec<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[A-Za-z]+").unwrap();
    }
    RE.find_iter(text)
        .filter_map(|token| token.as_str().to_lowercase().parse().ok())
        .collect()
}
