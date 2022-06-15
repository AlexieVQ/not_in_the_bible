use std::io::stdout;

use diesel::PgConnection;
use yaml_rust::Yaml;

use crate::{embedded_migrations, log_expect::LogExpect, config::from_env};

const DB_VAR: &str = "NITB_DB";

/// Configuration for the database.
pub struct DBConf {
    pub url: String,
}

impl DBConf {

    /// Creates a new database with given URL.
    pub fn new(url: String) -> DBConf {
        DBConf { url }
    }

    /// Creates a database from given YAML config.
    pub fn from_config(conf: &Yaml) -> DBConf {
        DBConf::new(from_env(DB_VAR).unwrap_or_else(|| conf
            .as_str()
            .log_expect("Missing or malformed \"db\" url")
            .to_string()))
    }

}

/// Runs database migrations.
pub fn run_migrations(connection: &PgConnection) {
    embedded_migrations::run_with_output(connection, &mut stdout())
        .log_expect("Error while running migrations");
}