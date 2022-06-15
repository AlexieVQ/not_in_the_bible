use std::{
    collections::{HashSet, HashMap},
    io::{Read, BufReader, BufRead, Write, self},
    fs::File, path::{Path, PathBuf}, env,
};

use unidecode::unidecode;
use yaml_rust::Yaml;

use crate::{
    dictionary::{Dictionary, DictionarySet},
    tokenize::Tokenize,
    log_expect::LogExpect,
};

/// A Dictionary that stores its set of words in memory.
pub struct InMemoryDictionary {
    words: HashSet<String>,
    excluded: HashSet<String>,
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
            excluded: HashSet::new(),
            name,
            lang,
        }
    }

    /// Creates a dictionary with words from given input.
    pub fn from_input(input: &mut dyn Read,
                      excluded: &mut dyn Read,
                      name: String,
                      lang: String) -> InMemoryDictionary {
        let mut dic = Self::new(name, lang);
        for line in BufReader::new(input).lines() {
            for word in line
                .expect("Error while loading input file")
                .tokenize() {
                dic.words.insert(unidecode(word).to_lowercase());
            }
        }
        for line in BufReader::new(excluded).lines() {
            for word in line
                .expect("Error while loading exclusion file")
                .tokenize() {
                dic.excluded.insert(unidecode(word).to_lowercase());
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

    fn ignored(&self, word: &str) -> bool {
        let word = unidecode(&word.to_lowercase());
        self.excluded.contains(&word)
    }

}

impl InMemoryDictionarySet {

    /// Creates a set of dictionaries from given YAML config.
    ///
    /// `config_path` is the path of the YAML config. Dictionaries' paths are
    /// given relatively to this path.
    pub fn from_config<P: AsRef<Path>>(conf: &Yaml,
                                       config_path: P) -> InMemoryDictionarySet
    {
        let config_dir = {
            let mut buf;
            if config_path.as_ref().is_absolute() {
                buf = PathBuf::new();
            } else {
                buf = env::current_dir()
                    .log_expect("Unable to read working directory");
            }
            if let Some(path) = config_path.as_ref().parent() {
                buf.push(path);
            }
            buf
        };
        let mut dics: HashMap<String, InMemoryDictionary> = HashMap::new();
        let mut default: Option<String> = None;
        for dic_conf in conf
            .as_vec()
            .log_expect("Missing or wrong \"sources\" array") {
            let path = {
                let dic_path = Path::new(dic_conf["path"]
                    .as_str()
                    .log_expect("Missing or wrong source path"));
                if dic_path.is_relative() {
                    let mut path = config_dir.clone();
                    path.push(&dic_path);
                    path
                } else {
                    dic_path.to_path_buf()
                }
            };
            let exclusion_path = dic_conf["excluded"]
                .as_str().map(|p| {
                    let path = Path::new(p);
                    if path.is_relative() {
                        let mut p2 = config_dir.clone();
                        p2.push(path);
                        p2
                    } else {
                        path.to_path_buf()
                    }
                });
            let dic = InMemoryDictionary::from_input(
                &mut File::open(&path)
                    .log_expect(&format!("Error loading source file {}",
                        &path.to_str().unwrap_or("(unknown)"))),
                match exclusion_path {
                    Some(path) => Box::new(File::open(path)
                        .expect("Error loading exclusion list"))
                        as Box<dyn Read>,
                    None => Box::new(io::empty()),
                }.as_mut(),
                dic_conf["name"]
                    .as_str()
                    .log_expect("Missing source name")
                    .to_string(),
                dic_conf["lang"]
                    .as_str()
                    .log_expect("Missing source lang")
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
