use std::{thread::sleep, time::Duration, convert::TryInto};

use regex::Regex;
use log::error;

use crate::{job_queue::JobQueue, request::Request};

use super::connection::Connection;

/// Routine that listens to incoming Twitter mentions and adds request in the
/// request queue.
pub fn listen(connection: &Connection,
              request_queue: &mut impl JobQueue<Request>) {
    let regex = Regex::new(&format!("@{}", &connection.user_name)).unwrap();
    let mut last_mention_id: Option<String> = None;
    let sleep_duration = Duration::new(connection.conf
        .refresh_interval.try_into().unwrap(), 0);
    loop {
        let mentions = connection.mentions(match last_mention_id.as_ref() {
            Some(str) => Some(&str),
            None => None,
        });
        if let Some(mention) = mentions.get(0) {
            last_mention_id = Some(mention.id_str.to_string());
        }
        for mention in mentions {
            if let Some(op_id) = mention.in_reply_to_status_id_str.as_ref() {
                if mention.in_reply_to_user_id_str.as_ref() !=
                    Some(&connection.user_id) {
                    match connection.by_id(&op_id) {
                        Ok(op) => {
                            if !regex.is_match(
                                op.text.as_ref().unwrap_or(
                                    op.full_text.as_ref().unwrap_or(
                                        &"".to_string()))) {
                                let request = Request::from_tweets(&mention, &op);
                                request_queue.submit(request);
                            }
                        },
                        Err(error) => {
                            error!("Error querying tweet {}: {}", op_id,
                                error);
                        },
                    };
                }
            }
        }
        sleep(sleep_duration);
    }
}