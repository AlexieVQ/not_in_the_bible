use yaml_rust::Yaml;

/// Default maximum number of tweets sent in one hour
const UDPATES_PER_HOUR: i64 = 12;

/// Default refresh interval (in seconds)
const REFRESH_INTERVAL: i64 = 60;

/// Configuration for the Twitter API.
pub struct TwitterConf {
    pub api_key: String,
    pub api_secret: String,
    pub updates_per_hour: i64,
    pub refresh_interval: i64,
}

impl TwitterConf {

    /// Creates a TwitterConf from given YAML file.
    pub fn from_yaml(yaml: &Yaml) -> TwitterConf {
        TwitterConf {
            api_key: yaml["api_key"]
                .as_str()
                .expect("Missing or wrong twitter api_key")
                .to_string(),
            api_secret: yaml["api_secret"]
                .as_str()
                .expect("Missing or wrong twitter api_secret")
                .to_string(),
            updates_per_hour: match yaml["updates_per_hour"] {
                Yaml::Integer(i) => i,
                _ => UDPATES_PER_HOUR,
            },
            refresh_interval: match yaml["refresh_interval"] {
                Yaml::Integer(i) => i,
                _ => REFRESH_INTERVAL,
            },
        }
    }

}