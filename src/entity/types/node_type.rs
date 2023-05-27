use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::Serialize;

use crate::macros::from_to_str;

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(None)")]
pub enum NodeType {
    #[sea_orm(string_value = "Root")]
    Root,
    #[sea_orm(string_value = "Telegram")]
    Telegram,
    #[sea_orm(string_value = "SingleFileZ")]
    SingleFileZ,
    #[sea_orm(string_value = "Scrapbook")]
    Scrapbook,
    #[sea_orm(string_value = "OneTab")]
    OneTab,
    #[sea_orm(string_value = "Zotero")]
    Zotero,
}

#[derive(Clone, Copy, Debug)]
pub enum AttachedTableType {
    Telegram,
    Scrapbook,
    Zotero,
}

impl NodeType {
    pub fn attached_table_type(&self) -> Option<AttachedTableType> {
        match self {
            Self::Telegram => Some(AttachedTableType::Telegram),
            Self::Scrapbook => Some(AttachedTableType::Scrapbook),
            Self::Zotero => Some(AttachedTableType::Zotero),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SourceFolderType {
    Telegram,
    SingleFileZ,
    Scrapbook,
    OneTab,
}

impl NodeType {
    pub fn source_folder_type(&self) -> Option<SourceFolderType> {
        match self {
            Self::Telegram => Some(SourceFolderType::Telegram),
            Self::SingleFileZ => Some(SourceFolderType::SingleFileZ),
            Self::Scrapbook => Some(SourceFolderType::Scrapbook),
            Self::OneTab => Some(SourceFolderType::OneTab),
            _ => None,
        }
    }
}

impl SourceFolderType {
    from_to_str! {
        pub url_name {
            SourceFolderType::Telegram => "telegram",
            SourceFolderType::SingleFileZ => "single_file_z",
            SourceFolderType::Scrapbook => "scrapbook",
            SourceFolderType::OneTab => "onetab",
        }
    }
}
