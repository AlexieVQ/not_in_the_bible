use chrono::{DateTime, FixedOffset};
use lazy_static::lazy_static;

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
}

impl Request {

    /// Creates a request from given tweets.
    pub fn from_tweets(request: &Tweet, op: &Tweet) -> Request {
        Request {
            user: request.user.screen_name,
            request_id: request.id_str,
            date: parse_from_str(request.created_at),
            request_text: request.text,
            op_id: op.id_str,
            op_author: op.user.screen_name,
            op_text: op.text,
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
    pub fn words(&self) -> Vec<String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"@\w+").unwrap();
        }
        let text = RE.replace_all(self.op_text, "");
        text.tokenize()
    }

}