/// Trait for a dictionary, that stores words.
pub trait Dictionary {

    /// Tests if this dictionary contains given word.
    fn contains(&self, word: &str) -> bool;

    /// Returns words that are not contained in this dictionary.
    fn absent_words<'a>(&self, words: &[&'a str]) -> Vec<&'a str> {
        let mut vec: Vec<&str> = Vec::new();
        for word in words {
            if !self.contains(word) {
                vec.push(word);
            }
        }
        vec
    }

}