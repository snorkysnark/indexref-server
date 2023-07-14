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

impl NodeType {
    pub fn field_name(&self) -> &'static str {
        match self {
            NodeType::Root => "root",
            NodeType::Telegram => "telegram",
            NodeType::SingleFileZ => "single_file_z",
            NodeType::Scrapbook => "scrapbook",
            NodeType::OneTab => "onetab",
            NodeType::Zotero => "zotero",
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
