use std::{
    fs,
    path::{Path, PathBuf},
};

use paste::paste;
use serde::Deserialize;

use crate::entity::types::NodeType;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub sources: SourcesConfig,
    pub server: ServerConfig,
}

impl AppConfig {
    pub fn load(path: &Path) -> eyre::Result<Self> {
        // TODO: custom FileNotFound error that prints the expected path
        Ok(toml::from_str(&fs::read_to_string(path)?)?)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SourcesConfig {
    telegram_chat: Option<PathBuf>,
    single_file_z: Option<PathBuf>,
    scrapbook: Option<PathBuf>,
}

#[derive(Debug, thiserror::Error)]
pub enum BasePathError {
    #[error("Value '{0}' is missing in config, yet nodes of this type exist in the database")]
    MissingValue(&'static str),
}

macro_rules! config_getter {
    ($field:ident) => {
        #[allow(dead_code)]
        pub fn $field(&self) -> Option<&Path> {
            self.$field.as_deref()
        }

        paste! {
            #[allow(dead_code)]
            pub fn [<$field _ok>](&self) -> Result<&Path, BasePathError> {
                self.$field.as_deref().ok_or(BasePathError::MissingValue(stringify!($field)))
            }
        }
    };
}

impl SourcesConfig {
    config_getter!(telegram_chat);
    config_getter!(single_file_z);
    config_getter!(scrapbook);

    pub fn get_base_path(&self, node_type: NodeType) -> Result<&Path, BasePathError> {
        match node_type {
            NodeType::Telegram => self.telegram_chat_ok(),
            NodeType::SingleFileZ => self.single_file_z_ok(),
            NodeType::ScrapbookFile | NodeType::ScrapbookPage => self.scrapbook_ok(),
        }
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
