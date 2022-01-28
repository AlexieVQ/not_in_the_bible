use std::panic;

use diesel::{
    PgConnection,
    Connection,
    QueryDsl,
    ExpressionMethods,
    RunQueryDsl, OptionalExtension, result::{Error, DatabaseErrorKind}
};

use crate::{
    schema::history,
    db_conf::{DBConf, run_migrations},
    log_expect::LogExpect,
};

/// History of statuses analyzed.
pub trait History {

    /// Tests if a status of given id already exists in the history.
    fn exists(&self, id: &str) -> bool;

    /// Adds given id to history.
    fn add(&mut self, id: &str);

}

/// An History stored in database.
pub struct DBHistory {
    connection: PgConnection,
}

/// An history element
#[derive(Queryable, Insertable)]
#[table_name = "history"]
struct HistoryElement {
    pub id: String,
}

impl DBHistory {

    /// Creates a new access to the history stored in database.
    pub fn new(conf: &DBConf) -> DBHistory {
        let db_url = &conf.url;
        let connection = PgConnection::establish(&db_url)
            .log_expect(&format!("Error connecting to {}", &db_url));
        run_migrations(&connection);
        DBHistory { connection }
    }

}

impl History for DBHistory {

    fn exists(&self, id: &str) -> bool {
        history::dsl::history
            .filter(history::id.eq(id))
            .get_result::<HistoryElement>(&self.connection)
            .optional()
            .log_expect("Error while reading history from database")
            .is_some()
    }

    fn add(&mut self, id: &str) {
        let element = HistoryElement { id: id.to_string() };
        match diesel::insert_into(history::table).values(&element).get_result::<HistoryElement>(&self.connection) {
            Ok(_) => {},
            Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) =>
                {},
            Err(error) => {
                panic!("Error while adding element to history: {}", error);
            }
        };
    }

}