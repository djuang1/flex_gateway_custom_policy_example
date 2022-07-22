use std::fmt;

use serde::Deserialize;
use serde_json::{error::Category, Error};
use std::{error, fmt::Debug};

const JSON_SYNTAX_ERROR: &str = "Invalid JSON string";
const JSON_DATA_ERROR: &str = "Invalid data for this policy";
const JSON_UNEXPECTED_ERROR: &str = "Unexpected JSON error";

#[derive(Deserialize, Clone)]
pub struct Config {
    #[serde(alias = "header")]
    pub header_name: Option<String>,

    #[serde(alias = "value")]
    pub header_value: Option<String>,

    pub headers: Option<Vec<HeaderConfig>>,
}

#[derive(Deserialize, Clone)]
pub struct HeaderConfig {
    #[serde(alias = "header")]
    pub header_name: String,

    #[serde(alias = "value")]
    pub header_value: String,
}

#[derive(Debug)]
pub struct ConfigError {
    err: String,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&*self.err, f)
    }
}

impl From<serde_json::Error> for ConfigError {
    fn from(error: Error) -> Self {
        ConfigError {
            err: match error.classify() {
                Category::Syntax => String::from(JSON_SYNTAX_ERROR),
                Category::Data => String::from(JSON_DATA_ERROR),
                _ => String::from(JSON_UNEXPECTED_ERROR),
            },
        }
    }
}

impl error::Error for ConfigError {}

impl Config {
    fn new(header_name: String, header_value: String, headers: Vec<HeaderConfig>) -> Config {
        Config {
            header_name: Some(header_name),
            header_value: Some(header_value),
            headers: Some(headers),
        }
    }

    pub fn from_json(json: &str) -> Result<Config, ConfigError> {
        let config = serde_json::from_str::<Config>(json)?;

        if config.header_name != config.header_value
            && (config.header_name == None || config.header_value == None)
        {
            Err(ConfigError {
                err: JSON_DATA_ERROR.to_string(),
            })
        } else {
            Ok(config)
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new(
            String::from("x-test-value"),
            String::from("my-test-value"),
            vec![],
        )
    }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::Config;
    use crate::config::{HeaderConfig, JSON_DATA_ERROR, JSON_SYNTAX_ERROR};

    #[test]
    fn new_config() {
        let single_header_name = String::from("list-header-name");
        let single_header_value = String::from("list-header-value");
        let list_header_name = String::from("list-header-name");
        let list_header_value = String::from("list-header-value");

        let config = Config::new(
            single_header_name.clone(),
            single_header_value.clone(),
            vec![HeaderConfig {
                header_name: list_header_name.clone(),
                header_value: list_header_value.clone(),
            }],
        );

        assert_eq!(
            config.header_name.unwrap().as_str(),
            single_header_name.as_str()
        );
        assert_eq!(
            config.header_value.unwrap().as_str(),
            single_header_value.as_str()
        );

        let headers = config.headers.clone().unwrap();
        assert_eq!(headers.len(), 1);
        assert_eq!(headers[0].header_name.as_str(), list_header_name.as_str());
        assert_eq!(headers[0].header_value.as_str(), list_header_value.as_str());
    }

    #[test]
    fn new_config_from_json_single_header() {
        let header_name = String::from("header-name");
        let header_value = String::from("header-value");

        let json_str = format!(
            r#"{{
            "header": "{}",
            "value": "{}"
        }}"#,
            header_name, header_value
        );

        let config = Config::from_json(json_str.as_str());
        if !config.is_ok() {
            panic!("parsing failed: {:?}", config.err())
        }
        let config = config.ok().unwrap();

        assert_eq!(config.header_name.unwrap().as_str(), header_name.as_str());
        assert_eq!(config.header_value.unwrap().as_str(), header_value.as_str());
    }

    #[test]
    fn new_config_from_json_multiple_headers() {
        let header1_name = String::from("header1-name");
        let header1_value = String::from("header1-value");
        let header2_name = String::from("header2-name");
        let header2_value = String::from("header2-value");

        let json_str = format!(
            r#"{{
            "headers": [
                {{
                    "header": "{}",
                    "value": "{}"
                }},
                {{
                    "header": "{}",
                    "value": "{}"
                }}
            ]
        }}"#,
            header1_name, header1_value, header2_name, header2_value
        );

        let config = Config::from_json(json_str.as_str());
        if !config.is_ok() {
            panic!("parsing failed: {:?}", config.err())
        }
        let config = config.ok().unwrap();

        let headers = config.headers.clone().unwrap();
        assert_eq!(headers.len(), 2);
        assert_eq!(headers[0].header_name.as_str(), header1_name.as_str());
        assert_eq!(headers[0].header_value.as_str(), header1_value.as_str());
        assert_eq!(headers[1].header_name.as_str(), header2_name.as_str());
        assert_eq!(headers[1].header_value.as_str(), header2_value.as_str());
    }

    #[test]
    fn new_config_from_json_with_wrong_format_returns_error() {
        let json_str = String::from("not json");

        let config = Config::from_json(json_str.as_str());
        let config = config.err().unwrap();

        assert_eq!(config.to_string(), JSON_SYNTAX_ERROR)
    }

    #[test]
    fn new_config_from_json_without_header_name_returns_error() {
        let json_str = String::from(r#"{"value": "header-value"}"#);

        let config = Config::from_json(json_str.as_str());
        let config = config.err().unwrap();

        assert_eq!(config.to_string(), JSON_DATA_ERROR)
    }

    #[test]
    fn new_default_config() {
        let config: Config = Default::default();

        let headers = config.headers.clone().unwrap();
        assert_eq!(headers.len(), 0);
        assert_eq!(config.header_name.unwrap().as_str(), "x-test-value");
        assert_eq!(config.header_value.unwrap().as_str(), "my-test-value");
    }
}