use std::{thread::sleep, time::Duration, convert::TryInto};

use regex::Regex;
use log::error;

use crate::{job_queue::JobQueue, request::Request};

use super::{connection::Connection, tweet::Tweet};

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
            let mut _st: Option<Tweet> = None;
            let mut quoted = false;
            let op: Option<&Tweet> = match mention.in_reply_to_status_id_str.as_ref() {
                Some(op_id) => {
                    if mention.in_reply_to_user_id_str.as_ref() !=
                        Some(&connection.user_id) {
                        match connection.by_id(&op_id) {
                            Ok(op) => if !regex.is_match(
                                op.text.as_ref().unwrap_or(
                                    op.full_text.as_ref().unwrap_or(
                                        &"".to_string()))) {
                                    _st = Some(op);
                                    Some(_st.as_ref().unwrap())
                                } else {
                                    None
                                },
                            Err(error) => {
                                error!("Error querying tweet {}: {}", op_id,
                                    error);
                                None
                            },
                        }
                    } else {
                        None
                    }
                },
                None => match mention.quoted_status {
                    Some(ref op) => {
                        quoted = true;
                        Some(op)
                    },
                    None => None,
                },
            };
            if let Some(op) = op {
                let request = Request::from_tweets(&mention, &op, quoted);
                request_queue.submit(request);
            }
        }
        sleep(sleep_duration);
    }
}