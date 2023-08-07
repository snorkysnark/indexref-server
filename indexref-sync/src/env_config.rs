use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use serde::Deserialize;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Deserialize)]
pub struct EnvConfig {
    db: String,
    #[serde(default = "default_sources")]
    sources: PathBuf,
    // Filters which crates can emit logs
    // See: https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html
    #[serde(default = "default_log_filter")]
    log: String,
}

impl EnvConfig {
    pub fn from_env() -> envy::Result<Self> {
        envy::prefixed("INDEXREF_").from_env()
    }

    pub fn db(&self) -> &str {
        self.db.as_ref()
    }

    pub fn sources(&self) -> &Path {
        self.sources.as_ref()
    }

    pub fn env_filter(&self) -> Result<EnvFilter, <EnvFilter as FromStr>::Err> {
        self.log.parse()
    }
}

fn default_sources() -> PathBuf {
    "config.json".into()
}

fn default_log_filter() -> String {
    "indexref_sync,sea_orm".to_owned()
}
