use std::{io::{Read, stdin, stdout, self}, fs::File};

use not_in_the_bible::{
    in_memory_dictionary::InMemoryDictionary,
    dictionary::Dictionary
};
use rustop::opts;

fn main() {
    let (args, _) = opts! {
        synopsis "Split a file into a list of words.";
        param file: Option<String>, desc: "File to split (stdin by default)";
    }.parse_or_exit();

    let mut input: Box<dyn Read> = match args.file {
        Some(path) => Box::new(File::open(path)
            .expect("Error while opening file")),
        None => Box::new(stdin()),
    };

    let dictionary = InMemoryDictionary::from_input(&mut input,
        Box::new(io::empty()).as_mut(), "".to_string(), "".to_string());

    dictionary.print(&mut stdout());
}