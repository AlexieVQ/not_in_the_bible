use std::io::Write;

/// Trait for a dictionary, that stores words.
pub trait Dictionary {

    /// Name of the dictionary.
    fn name(&self) -> &str;

    /// Language of the dictionary.
    fn lang(&self) -> &str;

    /// Tests if this dictionary contains given word.
    fn contains(&self, word: &str) -> bool;

    /// Tests if given word is ignored by this dictionary (eg. functions words).
    fn ignored(&self, _word: &str) -> bool {
        false
    }

    /// Returns words that are not contained in this dictionary and the count
    /// of words that are present.
    fn absent_words<'a>(&self, words: &[&'a str]) -> (Vec<&'a str>, usize) {
        let mut vec: Vec<&str> = Vec::new();
        let mut present: usize = 0;
        for word in words {
            if !self.ignored(word) {
                if self.contains(word) {
                    present += 1;
                } else {
                    vec.push(word);
                }
            }
        }
        (vec, present)
    }

    /// Prints all the words in alphabetic order to given output.
    fn print(&self, out: &mut impl Write);

}

/// Trait for a set of dictionaries.
pub trait DictionarySet<T: Dictionary> {

    /// Returns the dictionary corresponding to given lang, or the default
    /// Dictionary if there is none.
    fn by_lang(&self, lang: &str) -> &T;

    /// Returns default dictionary.
    fn default(&self) -> &T;

}