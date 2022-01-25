#[macro_use]
extern crate diesel;

use std::{io::{self, Read}, fs::File, thread, sync::Arc};

use db_request_queue::DBRequestQueue;
use db_response_queue::DBResponseQueue;
use history::DBHistory;
use rustop::opts;
use twitter::{twitter_conf::TwitterConf, listener, connection::Connection, responder};
use yaml_rust::YamlLoader;

use crate::in_memory_dictionary::InMemoryDictionary;

mod in_memory_dictionary;
mod dictionary;
mod tokenize;
mod request;
mod job_queue;
mod response;
mod searcher;
mod db_request_queue;
mod schema;
mod db_response_queue;
mod history;
mod twitter;

fn main() {
    let (args, _) = opts! {
        synopsis concat!("A program that searches for word that are absent in ",
                "a text file.");
        opt input: Option<String>, desc: "Input file name.";
        opt config: String, desc: "Config file name (.yaml).";
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
    let mut str = String::new();
    File::open(args.config)
        .expect("Error opening config file")
        .read_to_string(&mut str)
        .expect("Error reading config file");
    let yaml_config = YamlLoader::load_from_str(&str)
        .expect("Error parsing config file");
    let twitter_conf = TwitterConf::from_yaml(&yaml_config[0]["twitter"]);
    let connection = Arc::new(Connection::init(twitter_conf));
    let dic = InMemoryDictionary::from_input(&mut file);

    let t_dic = thread::spawn(move || {
        let mut request_queue = DBRequestQueue::new();
        let mut response_queue = DBResponseQueue::new();
        searcher::run(&mut request_queue, &mut response_queue, &dic);
    });

    let c1 = connection.clone();
    let t_req = thread::spawn(move || {
        let mut request_queue = DBRequestQueue::new();
        listener::listen(&c1, &mut request_queue);
    });

    let t_res = thread::spawn(move || {
        let mut response_queue = DBResponseQueue::new();
        let mut history = DBHistory::new();
        responder::respond(&connection, &mut response_queue, &mut history);
    });

    t_dic.join().unwrap();
    t_req.join().unwrap();
    t_res.join().unwrap();
}
