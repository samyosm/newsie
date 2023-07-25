use std::{fs::File, io::BufReader};

use crate::article::{Category, Language};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub config: Vec<FeedConfig>,
}

impl Config {
    pub async fn read_config() -> anyhow::Result<Config> {
        let path = std::env::var("config").unwrap_or("config.json".to_string());

        let file = File::open(path)?;
        let rdr = BufReader::new(file);
        let config: Config = serde_json::from_reader(rdr)?;
        return Ok(config);
    }
}

#[derive(Serialize, Deserialize)]
pub struct FeedConfig {
    pub language: Language,
    pub category: Category,
    pub feeds: Vec<String>,
}
