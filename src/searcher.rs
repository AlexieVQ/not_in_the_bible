use std::collections::HashMap;

use fluent_templates::{static_loader, LanguageIdentifier, fs::langid, Loader, fluent_bundle::{FluentValue, types::FluentNumber}};

use crate::{
    dictionary::Dictionary,
    response::Response,
    job_queue::JobQueue,
    request::Request
};

const EN: LanguageIdentifier = langid!("en");
const FR: LanguageIdentifier = langid!("fr");

static_loader! {
    static LOCALES = {
        locales: "./locales",
        fallback_language: "en",
    };
}

/// A routine that wait for requests and send responses to them.
pub fn run(request_queue: &mut impl JobQueue<Request>,
       response_queue: &mut impl JobQueue<Response>,
       dictionary: &impl Dictionary) {
    let lang = &FR;
    let book = "the Bible";
    loop {
        let request = request_queue.take();
        let words = request.words();
        let absent_words = dictionary.absent_words(&words);
        let mut args = HashMap::new();
        args.insert("book", book.into());
        args.insert("wordsCount", FluentValue::Number(FluentNumber::from(
            absent_words.len())));
        args.insert("absentWordsCount", FluentValue::Number(
            FluentNumber::from(absent_words.len())));
        args.insert("words", match absent_words.len() {
            0 => "".to_string(),
            1 => format!("“{}”", absent_words[0]),
            n => {
                let last = absent_words.last().unwrap();
                let list = &absent_words[..n - 1];
                format!("“{}” {} “{}”", list.join("”, “"),
                    LOCALES.lookup(lang, "and"), last)
            }
        }.into());
        let message: String = if absent_words.is_empty() {
            LOCALES.lookup_with_args(lang, "in_book", &args)
        } else if absent_words.len() == words.len() {
            LOCALES.lookup_with_args(lang, "nothing_in_book", &args)
        } else {
            LOCALES.lookup_with_args(lang, "not_in_book", &args)
        };
        response_queue.submit(Response::new(&request, message));
    }
}