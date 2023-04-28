use std::{
    fs,
    path::{Path, PathBuf},
};

use paste::paste;
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

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Missing value in config: {0}")]
    MissingValue(&'static str),
}

macro_rules! config_getter {
    ($name:ident, $field:ident) => {
        #[allow(dead_code)]
        pub fn $name(&self) -> Option<&Path> {
            self.$field.as_deref()
        }

        paste! {
            #[allow(dead_code)]
            pub fn [<$name _ok>](&self) -> Result<&Path, ConfigError> {
                self.$field.as_deref().ok_or(ConfigError::MissingValue(stringify!($field)))
            }
        }
    };
}

impl SourcesConfig {
    config_getter!(telegram_chat, telegram_chat);
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
