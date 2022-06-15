use std::{env, fs::File, io::Read, str::FromStr};

use yaml_rust::Yaml;

const DEFAULT_SHOW_PERCENT: i64 = 100;
const SHOW_PERCENT_VAR: &str = "NITB_SHOW_PERCENT";

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
            show_percent: from_env(SHOW_PERCENT_VAR).unwrap_or_else(||
                conf["show_percent"].as_i64().unwrap_or(DEFAULT_SHOW_PERCENT)),
        }
    }

}

/// Reads a value from environment variable of given name. If the environment
/// variable is unset or empty, tries reading from a file whose path is written
/// in a variable which has the same name with `_FILE` appended (for Docker
/// secrets support, for example MY_VAR_FILE for a variable called MY_VAR). If
/// this file does not exist, returns `None`.
pub fn from_env<T: FromStr>(key: &str) -> Option<T> {
    env::var(key).ok().map(|value| if value.is_empty() {
            None
        } else {
            Some(value)
        }
    ).or_else(||
        env::var(key.to_string() + "_FILE").ok().map(|path| {
                let mut value = String::new();
                match match File::open(path) {
                        Ok(mut file) =>
                            file.read_to_string(&mut value),
                        Err(_) => return None,
                    } {
                    Ok(_) => {},
                    Err(_) => return None,
                };
                Some(value)
            }
        )
    ).flatten().map(|v| v.parse().ok()).flatten()
}
