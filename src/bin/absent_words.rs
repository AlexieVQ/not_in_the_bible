use std::{io::{Read, stdin}, fs::File};

use not_in_the_bible::{in_memory_dictionary::InMemoryDictionary, tokenize::Tokenize, dictionary::Dictionary};
use rustop::opts;

fn main() {
    let (args, _) = opts! {
        synopsis "Search for absent words in a text.";
        opt input: Option<String>,
            desc: "Text in which to search (stdin by default)";
        param words: Vec<String>, desc: "Words to search";
    }.parse_or_exit();

    let mut text: Box<dyn Read> = match args.input {
        Some(path) => Box::new(File::open(&path)
            .expect(&format!("Error while opening {}", &path))),
        None => Box::new(stdin()),
    };

    let dic = InMemoryDictionary::from_input(&mut text,
        "".to_string(), "".to_string());

    for string in args.words {
        for word in string.tokenize() {
            if !dic.contains(word) {
                println!("{}", word);
            }
        }
    }
}