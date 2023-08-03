use std::str::FromStr;

use serde::Deserialize;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Deserialize)]
pub struct Config {
    db: String,
    #[serde(default = "default_port")]
    port: u16,
    // Filters which crates can emit logs
    // See: https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html
    #[serde(default = "default_log_filter")]
    log: String,
}

impl Config {
    pub fn from_env() -> envy::Result<Self> {
        envy::prefixed("INDEXREF_").from_env()
    }

    pub fn db(&self) -> &str {
        self.db.as_ref()
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn env_filter(&self) -> Result<EnvFilter, <EnvFilter as FromStr>::Err> {
        self.log.parse()
    }
}

fn default_log_filter() -> String {
    "indexref_serve,sea_orm".to_owned()
}

fn default_port() -> u16 {
    80
}
