use std::{io::{self, Read}, fs::File};

use dictionary::Dictionary;
use rustop::opts;

use crate::in_memory_dictionary::InMemoryDictionary;

mod in_memory_dictionary;
mod dictionary;
mod tokenize;
mod request;
mod tweet;
mod request_queue;
mod response;

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
    let absent = dic.absent_words(&args.words);
    for word in absent {
        println!("{}", word);
    }
}
