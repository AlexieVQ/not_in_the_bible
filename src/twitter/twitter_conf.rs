use yaml_rust::Yaml;

use crate::{log_expect::LogExpect, config::from_env};

/// Default maximum number of tweets sent in one hour
const UPDATES_PER_HOUR: i64 = 12;

/// Default refresh interval (in seconds)
const REFRESH_INTERVAL: i64 = 60;

const API_KEY_VAR: &str = "NITB_TWITTER_API_KEY";
const API_SECRET_VAR: &str = "NITB_TWITTER_API_SECRET";
const TOKEN_VAR: &str = "NITB_TWITTER_TOKEN";
const TOKEN_SECRET_VAR: &str = "NITB_TWITTER_TOKEN_SECRET";
const UPDATES_PER_HOUR_VAR: &str = "NITB_TWITTER_UPDATES_PER_HOUR";
const REFRESH_INTERVAL_VAR: &str = "NITB_TWITTER_REFRESH_INTERVAL";

/// Configuration for the Twitter API.
pub struct TwitterConf {
    pub api_key: String,
    pub api_secret: String,
    pub token: String,
    pub token_secret: String,
    pub updates_per_hour: i64,
    pub refresh_interval: i64,
}

impl TwitterConf {

    /// Creates a TwitterConf from given YAML file.
    pub fn from_yaml(yaml: &Yaml) -> TwitterConf {
        TwitterConf {
            api_key: from_env(API_KEY_VAR).unwrap_or_else(|| yaml["api_key"]
                .as_str()
                .log_expect("Missing or wrong twitter api_key")
                .to_string()),
            api_secret: from_env(API_SECRET_VAR).unwrap_or_else(||
                yaml["api_secret"]
                    .as_str()
                    .log_expect("Missing or wrong twitter api_secret")
                    .to_string()),
            token: from_env(TOKEN_VAR).unwrap_or_else(|| yaml["token"]
                .as_str()
                .log_expect("Missing or wrong twitter token")
                .to_string()),
            token_secret: from_env(TOKEN_SECRET_VAR).unwrap_or_else(||
                yaml["token_secret"]
                    .as_str()
                    .log_expect("Missing or wrong twitter token_secret")
                    .to_string()),
            updates_per_hour: from_env(UPDATES_PER_HOUR_VAR).unwrap_or_else(||
                match yaml["updates_per_hour"] {
                    Yaml::Integer(i) => i,
                    _ => UPDATES_PER_HOUR,
                }),
            refresh_interval: from_env(REFRESH_INTERVAL_VAR).unwrap_or_else(||
                match yaml["refresh_interval"] {
                    Yaml::Integer(i) => i,
                    _ => REFRESH_INTERVAL,
                }),
        }
    }

}
