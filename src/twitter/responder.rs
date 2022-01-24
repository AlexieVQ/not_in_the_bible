use std::{time::Duration, convert::TryInto, thread::sleep};

use crate::{job_queue::JobQueue, response::Response, history::History};

use super::connection::Connection;

/// A routine that send responses to processed requests.
pub fn respond(connection: &Connection,
               response_queue: &mut impl JobQueue<Response>,
               history: &mut impl History) {
    let sleep_duration = Duration::new(
        (3600 / connection.conf.updates_per_hour).try_into().unwrap(), 0);
    loop {
        let response = response_queue.take();
        let users: Vec<&str> = if response.op_author != response.user {
            vec![&response.user, &response.op_author]
        } else {
            vec![&response.user]
        };
        match connection.reply(&response.id, &users, &response.body) {
            Ok(_) => {
                history.add(&response.op_id);
            },
            Err(error) => {
                eprintln!("Error while replying to {}: {}", &response.id, error);
            },
        };
        sleep(sleep_duration);
    }
}