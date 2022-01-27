use std::{time::Duration, thread::sleep};

use diesel::{
    PgConnection,
    Connection,
    RunQueryDsl,
    QueryDsl,
    ExpressionMethods,
    result::{Error::{NotFound, self}, DatabaseErrorKind}
};
use lazy_static::lazy_static;

use crate::{
    schema::responses,
    job_queue::JobQueue,
    response::Response,
    db_conf::{DBConf, run_migrations}
};

const SLEEP_DURATION_SEC: u64 = 60;

/// A JobQueue storing Responses in a PostgreSQL database.
pub struct DBResponseQueue {
    connection: PgConnection,
}

impl DBResponseQueue {

    /// Creates a new access for the response queue sorted in database.
    pub fn new(conf: &DBConf) -> DBResponseQueue {
        let db_url = &conf.url;
        let connection = PgConnection::establish(&db_url)
            .expect(&format!("Error connecting to {}", &db_url));
        run_migrations(&connection);
        DBResponseQueue { connection }
    }

}

impl JobQueue<Response> for DBResponseQueue {

    /// Adds a response to this queue.
    fn submit(&mut self, response: Response) {
        match diesel::insert_into(responses::table)
                    .values(&response)
                    .get_result::<Response>(&self.connection) {
            Ok(_) => {},
            Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) =>
                        {},
            Err(error) => {
                panic!("Error submitting response to database: {}", error);
            }
        }
    }

    /// Takes a response from this queue (blocking).
    fn take(&mut self) -> Response {
        lazy_static! {
            static ref SLEEP_DURATION: Duration = Duration::new(
                SLEEP_DURATION_SEC, 0);
        }
        loop {
            match responses::dsl::responses
                            .order_by(responses::date.asc())
                            .first::<Response>(&self.connection) {
                Ok(response) => {
                    diesel::delete(
                        responses::dsl::responses.filter(
                            responses::id.eq(&response.id)))
                            .execute(&self.connection)
                            .expect("Error while deleting request from queue");
                    break response;
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