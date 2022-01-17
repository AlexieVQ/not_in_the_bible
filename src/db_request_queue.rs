use std::{env, time::Duration, thread::sleep};

use diesel::{
    PgConnection,
    Connection,
    RunQueryDsl,
    QueryDsl,
    ExpressionMethods,
    result::Error::NotFound
};
use dotenv::dotenv;
use lazy_static::lazy_static;

use crate::{request_queue::RequestQueue, request::Request, schema::requests};

const SLEEP_DURATION_SEC: u64 = 60;

/// A RequestQueue stored in a PostgreSQL database.
pub struct DBRequestQueue {
    connection: PgConnection,
}

impl DBRequestQueue {

    /// Creates a new access for the request queue sorted in database.
    pub fn new() -> DBRequestQueue {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        let connection = PgConnection::establish(&db_url)
            .expect(&format!("Error connecting to {}", &db_url));
        DBRequestQueue { connection }
    }

}

impl RequestQueue for DBRequestQueue {

    /// Adds a request to this queue.
    fn submit(&mut self, request: Request) {
        diesel::insert_into(requests::table)
            .values(&request)
            .get_result::<Request>(&self.connection)
            .expect("Error submitting request");
    }

    /// Takes a request from this queue (blocking).
    fn take(&mut self) -> Request {
        lazy_static! {
            static ref SLEEP_DURATION: Duration = Duration::new(
                SLEEP_DURATION_SEC, 0);
        }
        loop {
            match requests::dsl::requests
                            .order_by(requests::date.asc())
                            .first(&self.connection) {
                Ok(request) => break request,
                Err(NotFound) => {
                    sleep(*SLEEP_DURATION);
                },
                Err(error) => {
                    panic!("Error while querying requests: {}", error)
                }
            }
        }
    }

}