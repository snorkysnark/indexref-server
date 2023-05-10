use std::path::Path;

use fs_err as fs;
use serde::Deserialize;

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
pub struct ServerConfig {
    port: u16,
}

impl ServerConfig {
    pub fn port(&self) -> u16 {
        self.port
    }
}

pub use self::sources::{BasePathError, SourcesConfig};
mod sources {
    use std::path::{Path, PathBuf};

    use serde::Deserialize;

    use crate::entity::types::ContainerType;

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

    impl SourcesConfig {
        pub fn get_base_path(&self, container_type: ContainerType) -> Result<&Path, BasePathError> {
            macro_rules! config_value {
                ($this:expr, $field:ident) => {
                    $this
                        .$field
                        .as_deref()
                        .ok_or(BasePathError::MissingValue(stringify!($field)))
                };
            }

            Ok(match container_type {
                ContainerType::Telegram => config_value!(self, telegram_chat)?,
                ContainerType::SingleFileZ => config_value!(self, single_file_z)?,
                ContainerType::Scrapbook => config_value!(self, scrapbook)?,
            })
        }
    }

    macro_rules! getter {
        ($name:ident) => {
            pub fn $name(&self) -> Option<&Path> {
                self.$name.as_deref()
            }
        };
    }

    impl SourcesConfig {
        getter!(telegram_chat);
        getter!(single_file_z);
        getter!(scrapbook);
    }
}
