use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::entity::types::SourceFolderType;

#[derive(Debug, Clone, Deserialize)]
pub struct SourcesConfig {
    telegram_chat: Option<PathBuf>,
    single_file_z: Option<PathBuf>,
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
            ($this:expr, $field:ident) => {
                $this
                    .$field
                    .as_deref()
                    .ok_or(BasePathError::MissingValue(stringify!($field)))
            };
        }

        Ok(match container_type {
            SourceFolderType::Telegram => config_value!(self, telegram_chat)?,
            SourceFolderType::SingleFileZ => config_value!(self, single_file_z)?,
            SourceFolderType::Scrapbook => config_value!(self, scrapbook)?,
            SourceFolderType::OneTab => config_value!(self, onetab)?,
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
    getter!(onetab);
}
