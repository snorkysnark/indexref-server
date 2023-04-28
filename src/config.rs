use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::Deserialize;

use crate::result::AppResult;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub sources: SourcesConfig,
    pub server: ServerConfig,
}

impl AppConfig {
    pub fn load(path: &Path) -> AppResult<Self> {
        // TODO: custom FileNotFound error that prints the expected path
        Ok(toml::from_str(&fs::read_to_string(path)?)?)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SourcesConfig {
    telegram_chat: Option<PathBuf>,
}

impl SourcesConfig {
    pub fn telegram_chat(&self) -> Option<&Path> {
        self.telegram_chat.as_deref()
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
