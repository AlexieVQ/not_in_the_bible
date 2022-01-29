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
    db_conf::DBConf,
    log_expect::LogExpect, config::Configuration,
};
use rustop::opts;
use yaml_rust::YamlLoader;

fn main() {
    let (args, _) = opts! {
        synopsis concat!("A program that searches for word that are absent in ",
                "a text file.");
        opt config: String, desc: "Config file name (.yaml).";
    }.parse_or_exit();
    env_logger::init();
    let mut str = String::new();
    File::open(args.config)
        .log_expect("Error opening config file")
        .read_to_string(&mut str)
        .log_expect("Error reading config file");
    let yaml_config = YamlLoader::load_from_str(&str)
        .log_expect("Error parsing config file");
    let bot_conf = Configuration::from_config(&yaml_config[0]);
    let db_conf = DBConf::from_config(&yaml_config[0]["db"]);
    let twitter_conf = TwitterConf::from_yaml(&yaml_config[0]["twitter"]);
    let connection = Arc::new(Connection::init(twitter_conf));
    let dics = InMemoryDictionarySet::from_config(&yaml_config[0]["sources"]);

    let mut request_queue = DBRequestQueue::new(&db_conf);
    let mut response_queue = DBResponseQueue::new(&db_conf);
    let t_dic = thread::spawn(move || {
        searcher::run(&mut request_queue, &mut response_queue, &dics,
            &bot_conf);
    });

    let c1 = Clone::clone(&connection);
    let mut request_queue = DBRequestQueue::new(&db_conf);
    let t_req = thread::spawn(move || {
        listener::listen(&c1, &mut request_queue);
    });

    let mut response_queue = DBResponseQueue::new(&db_conf);
    let mut history = DBHistory::new(&db_conf);
    let t_res = thread::spawn(move || {
        responder::respond(&connection, &mut response_queue, &mut history);
    });

    t_dic.join().unwrap();
    t_req.join().unwrap();
    t_res.join().unwrap();
}
