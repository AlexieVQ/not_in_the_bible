use std::{thread::sleep, time::Duration, convert::TryInto};

use crate::{job_queue::JobQueue, request::Request, history::History};

use super::connection::Connection;

/// Routine that listens to incoming Twitter mentions and adds request in the
/// request queue.
pub fn listen(connection: &Connection,
              request_queue: &mut impl JobQueue<Request>,
              history: &impl History) {
    let last_mention_id: Option<&str> = None;
    let sleep_duration = Duration::new(connection.conf
        .refresh_interval.try_into().unwrap(), 0);
    loop {
        let mentions = connection.mentions(last_mention_id);
        for mention in mentions {
            if let Some(op_id) = mention.in_reply_to_status_id_str.as_ref() {
                if !history.exists(&op_id) {
                    match connection.by_id(&op_id) {
                        Ok(op) => {
                            let request = Request::from_tweets(&mention, &op);
                            request_queue.submit(request);
                        },
                        Err(error) => {
                            eprintln!("Error querying tweet {}: {}", op_id,
                                error);
                        },
                    };
                }
            }
        }
        sleep(sleep_duration);
    }
}