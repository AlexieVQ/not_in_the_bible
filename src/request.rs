use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable};
use lazy_static::lazy_static;
use regex::Regex;

use crate::log_expect::LogExpect;
use crate::{twitter::tweet::Tweet, tokenize::Tokenize};
use crate::schema::requests;

/// A request from a status.
#[derive(Queryable, Insertable)]
pub struct Request {
    pub id: String,
    pub user: String,
    pub date: NaiveDateTime,
    pub op_id: String,
    pub op_author: String,
    pub text: String,
    pub lang: Option<String>,
}

impl Request {

    /// Creates a request from given tweets.
    pub fn from_tweets(request: &Tweet, op: &Tweet) -> Request {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(@\w+|https://t\.co/\w+)")
                .unwrap();
        }
        Request {
            user: request.user.screen_name.to_string(),
            id: request.id_str.to_string(),
            date: NaiveDateTime::parse_from_str(&request.created_at,
                "%a %b %d %T %z %Y").unwrap(),
            op_id: op.id_str.to_string(),
            op_author: op.user.screen_name.to_string(),
            text: RE.replace_all(match &op.full_text {
                Some(text) => &text,
                None => &op
                    .text
                    .as_ref()
                    .log_expect("No text nor full_text field"),
            }, "").to_string(),
            lang: match &op.lang {
                Some(lang) => Some(lang.to_string()),
                None => None
            },
        }
    }

    /// Returns words to analyze.
    pub fn words(&self) -> Vec<&str> {
        self.text.tokenize()
    }

}