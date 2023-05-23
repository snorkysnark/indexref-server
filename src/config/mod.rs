mod sources;

use std::path::Path;

use fs_err as fs;
use regex::Regex;
use serde::Deserialize;

pub use self::sources::{BasePathError, SourcesConfig};

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub sources: SourcesConfig,
    pub server: ServerConfig,
    #[serde(default)]
    pub settings: ImportSettings,
}

impl AppConfig {
    pub fn load(path: &Path) -> eyre::Result<Self> {
        Ok(toml::from_str(&fs::read_to_string(path)?)?)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    port: u16,
}

impl ServerConfig {
    pub fn port(&self) -> u16 {
        self.port
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ImportSettings {
    pub single_file_z: SingleFileZImportSettings,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct SingleFileZImportSettings {
    #[serde(with = "serde_regex")]
    pub date_regex: Option<Regex>,
}
