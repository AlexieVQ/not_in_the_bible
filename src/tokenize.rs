use lazy_static::lazy_static;
use regex::Regex;

/// Trait implemented by types that can be split into words.
pub trait Tokenize {

    /// Splits self into a list of words.
    fn tokenize(&self) -> Vec<String>;

}

impl Tokenize for String {
    fn tokenize(&self) -> Vec<String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"[[:alpha:]]+").unwrap();
        }
        RE.find_iter(&self).map(|m| m.as_str().to_string()).collect()
    }
}