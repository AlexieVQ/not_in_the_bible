use yaml_rust::Yaml;

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
        DBConf::new(conf
            .as_str()
            .expect("Missing or malformed \"db\" url")
            .to_string())
    }

}