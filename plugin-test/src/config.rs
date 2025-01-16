use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(with = "humantime_serde")]
    pub poll_interval: Duration,
    pub out_file: std::string::String
}

impl Default for Config {
    fn default() -> Self {
        Self { poll_interval: Duration::from_secs(1), out_file: String::from("alumet-tutorial-output.txt")}
    }
}