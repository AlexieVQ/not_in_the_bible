use std::{collections::HashSet, io::{Read, BufReader, BufRead}};

use unidecode::unidecode;

use crate::{dictionary::Dictionary, tokenize::Tokenize};

/// A Dictionary that stores its set of words in memory.
pub struct InMemoryDictionary {
    words: HashSet<String>,
}

impl InMemoryDictionary {
    /// Creates an empty dictionary.
    fn new() -> InMemoryDictionary {
        InMemoryDictionary {
            words: HashSet::new(),
        }
    }

    /// Creates a dictionary with words from given input.
    pub fn from_input(input: &mut impl Read) -> InMemoryDictionary {
        let mut dic = Self::new();
        for line in BufReader::new(input).lines() {
            match line {
                Ok(s) => {
                    for word in s.tokenize() {
                        dic.words.insert(unidecode(word).to_lowercase());
                    }
                },
                Err(e) => {
                    panic!("Error while loading input file: {}", e);
                },
            }
        }
        dic
    }

}

impl Dictionary for InMemoryDictionary {
    fn contains(&self, word: &str) -> bool {
        let word = unidecode(&word.to_lowercase());
        self.words.contains(&word)
    }
}