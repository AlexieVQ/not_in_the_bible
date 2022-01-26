use std::{collections::{HashSet, HashMap}, io::{Read, BufReader, BufRead, Write}, panic, fs::File};

use unidecode::unidecode;
use yaml_rust::Yaml;

use crate::{dictionary::{Dictionary, DictionarySet}, tokenize::Tokenize};

/// A Dictionary that stores its set of words in memory.
pub struct InMemoryDictionary {
    words: HashSet<String>,
    name: String,
    lang: String,
}

/// A set of dictionaries stored in memory.
pub struct InMemoryDictionarySet {
    dics: HashMap<String, InMemoryDictionary>,
    default_lang: String,
}

impl InMemoryDictionary {
    /// Creates an empty dictionary.
    fn new(name: String, lang: String) -> InMemoryDictionary {
        InMemoryDictionary {
            words: HashSet::new(),
            name,
            lang,
        }
    }

    /// Creates a dictionary with words from given input.
    pub fn from_input(input: &mut impl Read,
                      name: String,
                      lang: String) -> InMemoryDictionary {
        let mut dic = Self::new(name, lang);
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

    fn name(&self) -> &str {
        &self.name
    }

    fn lang(&self) -> &str {
        &self.lang
    }

    fn print(&self, out: &mut impl Write) {
        let mut words: Vec<&String> = self.words.iter().collect();
        words.sort();
        for word in words {
            writeln!(out, "{}", word).expect("Error writing to output");
        }
    }
}

impl InMemoryDictionarySet {

    /// Creates a set of dictionaries from given YAML config.
    pub fn from_config(conf: &Yaml) -> InMemoryDictionarySet {
        let mut dics: HashMap<String, InMemoryDictionary> = HashMap::new();
        let mut default: Option<String> = None;
        for dic_conf in conf
            .as_vec()
            .expect("Missing or wrong \"sources\" array") {
            let dic = InMemoryDictionary::from_input(
                &mut File::open(dic_conf["path"]
                    .as_str()
                    .expect("Missing or wrong source path"))
                    .expect("Error loading source"),
                dic_conf["name"]
                    .as_str()
                    .expect("Missing source name")
                    .to_string(),
                dic_conf["lang"]
                    .as_str()
                    .expect("Missing source lang")
                    .to_string());
            let lang = String::from(dic.lang());
            dics.insert(dic.lang.to_string(), dic);
            if default.is_none() {
                default = Some(lang);
            }
        }
        match default {
            Some(s) => InMemoryDictionarySet { dics, default_lang: s},
            None => panic!("No default source"),
        }
    }

}

impl DictionarySet<InMemoryDictionary> for InMemoryDictionarySet {
    fn by_lang(&self, lang: &str) -> &InMemoryDictionary {
        match self.dics.get(lang) {
            Some(dic) => dic,
            None => self.by_lang(&self.default_lang),
        }
    }

    fn default(&self) -> &InMemoryDictionary {
        self.by_lang(&self.default_lang)
    }
}