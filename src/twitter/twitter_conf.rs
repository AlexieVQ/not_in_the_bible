use yaml_rust::Yaml;

/// Configuration for the Twitter API.
pub struct TwitterConf {
    pub api_key: String,
    pub api_secret: String,
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
                .to_string()
        }
    }

}