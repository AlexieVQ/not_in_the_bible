use chrono::{DateTime, FixedOffset};
use lazy_static::lazy_static;
use regex::Regex;

use crate::{tweet::Tweet, tokenize::Tokenize};

/// A request from a status.
pub struct Request {
    user: String,
    request_id: String,
    date: DateTime<FixedOffset>,
    request_text: String,
    op_id: String,
    op_author: String,
    op_text: String,
    text: String,
}

impl Request {

    /// Creates a request from given tweets.
    pub fn from_tweets(request: &Tweet, op: &Tweet) -> Request {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"@\w+").unwrap();
        }
        Request {
            user: request.user.screen_name.to_string(),
            request_id: request.id_str.to_string(),
            date: DateTime::parse_from_str(&request.created_at,
                "%a %b %d %T %z %Y").unwrap(),
            request_text: request.text.to_string(),
            op_id: op.id_str.to_string(),
            op_author: op.user.screen_name.to_string(),
            op_text: op.text.to_string(),
            text: RE.replace_all(&op.text, "").to_string()
        }
    }

    /// Name of the user who made the request
    pub fn user(&self) -> &String {
        &self.user
    }

    /// Id of the status of the user who made the request
    pub fn request_id(&self) -> &String {
        &self.request_id
    }

    /// Date of the request
    pub fn date(&self) -> &DateTime<FixedOffset> {
        &self.date
    }

    /// Text of the request
    pub fn request_text(&self) -> &String {
        &self.request_text
    }

    /// Id of the status to analyze
    pub fn op_id(&self) -> &String {
        &self.op_id
    }

    /// Name of the author of the status to analyze
    pub fn op_author(&self) -> &String {
        &self.op_author
    }

    /// Text of the status to analyze
    pub fn op_text(&self) -> &String {
        &self.op_text
    }

    /// Returns words to analyze.
    pub fn words(&self) -> Vec<&str> {
        self.text.tokenize()
    }

}