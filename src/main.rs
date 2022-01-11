use std::{io::{self, Read}, fs::File};

use dictionary::Dictionary;
use rustop::opts;

use crate::in_memory_dictionary::InMemoryDictionary;

pub mod in_memory_dictionary;

pub mod dictionary;
pub mod tokenize;

fn main() {
    let (args, _) = opts! {
        synopsis concat!("A program that searches for word that are absent in ",
                "a text file.");
        opt input: Option<String>, desc: "Input file name.";
        param words: Vec<String>, desc: "Words to search.";
    }.parse_or_exit();
    let mut file = match args.input {
        Some(path) => match File::open(path) {
            Ok(file) =>
                    Box::new(file) as Box<dyn Read>,
            Err(e) => {
                panic!("Error while opening input file: {}", e);
            },
        },
        None => Box::new(io::stdin()) as Box<dyn Read>,
    };
    let dic = InMemoryDictionary::from_input(&mut file);
    let words: Vec<&String> = args.words.iter().map(|w| w).collect();
    let absent = dic.absent_words(&words);
    for word in absent {
        println!("{}", word);
    }
}
