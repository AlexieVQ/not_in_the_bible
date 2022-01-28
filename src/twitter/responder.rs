use std::{time::Duration, convert::TryInto, thread::sleep};

use log::error;

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
        let result = if response.quoted {
            connection.quote(&response.op_id, &response.op_author,
                &response.body)
        } else {
            connection.reply(&response.id, &users, &response.body)
        };
        match result {
            Ok(tweet) => {
                history.add(&response.op_id);
                if response.quoted {
                    if let Err(error) = connection.reply(
                        &response.id, &[&response.user],
                        &format!("https://twitter.com/{}/status/{}",
                        &tweet.user.screen_name, &tweet.id_str)) {
                        error!("Error while replying to {}: {}", &response.id,
                            match error {
                            oauth_client::Error::HttpStatus(status) =>
                                status.to_string(),
                            oauth_client::Error::Io(error) => error.to_string(),
                            oauth_client::Error::HttpRequest(error) =>
                                error.to_string(),
                            other => other.to_string(),
                        });
                    }
                }
            },
            Err(error) => {
                error!("Error while replying to {}: {}", &response.id,
                    match error {
                    oauth_client::Error::HttpStatus(status) =>
                        status.to_string(),
                    oauth_client::Error::Io(error) => error.to_string(),
                    oauth_client::Error::HttpRequest(error) =>
                        error.to_string(),
                    other => other.to_string(),
                });
            },
        };
        sleep(sleep_duration);
    }
}