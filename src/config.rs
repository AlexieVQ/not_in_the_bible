use yaml_rust::Yaml;

const DEFAULT_SHOW_PERCENT: i64 = 100;

/// Bot configuration.
pub struct Configuration {
    /// Ratio of absent words per request from which to show a percent instead
    /// of a list of words.
    pub show_percent: i64,
}

impl Configuration {

    /// Creates a Configuration from a YAML config file.
    pub fn from_config(conf: &Yaml) -> Configuration {
        Configuration {
            show_percent: conf["show_percent"]
                .as_i64()
                .unwrap_or(DEFAULT_SHOW_PERCENT),
        }
    }

}