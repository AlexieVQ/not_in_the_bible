/// Trait for a dictionary, that stores words.
pub trait Dictionary {

    /// Tests if this dictionary contains given word.
    fn contains(&self, word: &String) -> bool;

    /// Returns words that are not contained in this dictionary.
    fn absent_words<'a>(&self, words: &'a [String]) -> Vec<&'a String> {
        let mut vec: Vec<&String> = Vec::new();
        for word in words {
            if !self.contains(word) {
                vec.push(word);
            }
        }
        vec
    }

}