mod sources;

use std::path::Path;

use fs_err as fs;
use serde::Deserialize;

pub use self::sources::{BasePathError, SingleFileZConfig, SourcesConfig};

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub sources: SourcesConfig,
    pub server: ServerConfig,
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
