use lazy_static::lazy_static;
use regex::Regex;

/// Trait implemented by types that can be split into words.
pub trait Tokenize {

    /// Splits self into a list of words.
    fn tokenize(&self) -> Vec<&str>;

    /// Splits self into a list of words, keeping only those with a given
    /// minimum length.
    fn tokenize_min(&self, len: usize) -> Vec<&str> {
        self.tokenize()
            .into_iter()
            .filter(|word| word.len() >= len)
            .collect()
    }

}

impl Tokenize for str {
    fn tokenize(&self) -> Vec<&str> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\p{L}+").unwrap();
        }
        RE.find_iter(&self).map(|m| m.as_str()).collect()
    }
}