use std::{env, time::Duration, thread::sleep};

use diesel::{
    PgConnection,
    Connection,
    RunQueryDsl,
    QueryDsl,
    ExpressionMethods,
    result::Error::{NotFound, self}
};
use dotenv::dotenv;
use lazy_static::lazy_static;

use crate::{request::Request, schema::requests, job_queue::JobQueue};

const SLEEP_DURATION_SEC: u64 = 60;

/// A JobQueue storing Requests in a PostgreSQL database.
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

impl JobQueue<Request> for DBRequestQueue {

    /// Adds a request to this queue.
    fn submit(&mut self, request: Request) {
        match diesel::insert_into(requests::table)
                    .values(&request)
                    .get_result::<Request>(&self.connection) {
            Ok(_) => {},
            Err(Error::DatabaseError(_, error)) => {
                eprintln!("Database error while submitting request {}: {}",
                    request.id, error.message());
            },
            Err(error) => {
                panic!("Error submitting request to database: {}", error);
            }
        }
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
                            .first::<Request>(&self.connection) {
                Ok(request) => {
                    diesel::delete(
                        requests::dsl::requests.filter(
                            requests::id.eq(&request.id)))
                            .execute(&self.connection)
                            .expect("Error while deleting request from queue");
                    break request;
                },
                Err(NotFound) => {
                    sleep(*SLEEP_DURATION);
                },
                Err(error) => {
                    panic!("Error while querying requests: {}", error);
                }
            }
        }
    }

}