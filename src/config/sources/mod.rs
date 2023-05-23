mod single_file_z;

use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::entity::types::SourceFolderType;
pub use self::single_file_z::SingleFileZConfig;

#[derive(Debug, Clone, Deserialize)]
pub struct SourcesConfig {
    telegram_chat: Option<PathBuf>,
    single_file_z: Option<SingleFileZConfig>,
    scrapbook: Option<PathBuf>,
    onetab: Option<PathBuf>,
}

#[derive(Debug, thiserror::Error)]
pub enum BasePathError {
    #[error("Value '{0}' is missing in config, yet nodes of this type exist in the database")]
    MissingValue(&'static str),
}

impl SourcesConfig {
    pub fn get_base_path(&self, container_type: SourceFolderType) -> Result<&Path, BasePathError> {
        macro_rules! config_value {
            ($this:expr, $getter:ident) => {
                $this
                    .$getter()
                    .ok_or(BasePathError::MissingValue(stringify!($getter)))
            };
        }

        Ok(match container_type {
            SourceFolderType::Telegram => config_value!(self, telegram_chat)?,
            SourceFolderType::SingleFileZ => config_value!(self, single_file_z)?.path(),
            SourceFolderType::Scrapbook => config_value!(self, scrapbook)?,
            SourceFolderType::OneTab => config_value!(self, onetab)?,
        })
    }
}

impl SourcesConfig {
    pub fn telegram_chat(&self) -> Option<&Path> {
        self.telegram_chat.as_deref()
    }
    pub fn single_file_z(&self) -> Option<&SingleFileZConfig> {
        self.single_file_z.as_ref()
    }
    pub fn scrapbook(&self) -> Option<&Path> {
        self.scrapbook.as_deref()
    }
    pub fn onetab(&self) -> Option<&Path> {
        self.onetab.as_deref()
    }
}
