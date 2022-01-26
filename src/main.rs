use std::{fs::File, io::Read, sync::Arc, thread};

use not_in_the_bible::{
    twitter::{
        twitter_conf::TwitterConf,
        connection::Connection,
        listener,
        responder,
    },
    in_memory_dictionary::InMemoryDictionarySet,
    db_request_queue::DBRequestQueue,
    db_response_queue::DBResponseQueue,
    searcher,
    history::DBHistory,
};
use rustop::opts;
use yaml_rust::YamlLoader;

fn main() {
    let (args, _) = opts! {
        synopsis concat!("A program that searches for word that are absent in ",
                "a text file.");
        opt config: String, desc: "Config file name (.yaml).";
    }.parse_or_exit();
    let mut str = String::new();
    File::open(args.config)
        .expect("Error opening config file")
        .read_to_string(&mut str)
        .expect("Error reading config file");
    let yaml_config = YamlLoader::load_from_str(&str)
        .expect("Error parsing config file");
    let twitter_conf = TwitterConf::from_yaml(&yaml_config[0]["twitter"]);
    let connection = Arc::new(Connection::init(twitter_conf));
    let dics = InMemoryDictionarySet::from_config(&yaml_config[0]["sources"]);

    let t_dic = thread::spawn(move || {
        let mut request_queue = DBRequestQueue::new();
        let mut response_queue = DBResponseQueue::new();
        searcher::run(&mut request_queue, &mut response_queue, &dics);
    });

    let c1 = Clone::clone(&connection);
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
