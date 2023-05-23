use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::Serialize;

use crate::macros::from_to_str;

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(None)")]
pub enum NodeType {
    #[sea_orm(string_value = "Root")]
    Root,
    #[sea_orm(string_value = "Folder")]
    Folder,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(None)")]
pub enum AttachedTableType {
    #[sea_orm(string_value = "Telegram")]
    Telegram,
    #[sea_orm(string_value = "Scrapbook")]
    Scrapbook,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(None)")]
pub enum SourceFolderType {
    #[sea_orm(string_value = "Telegram")]
    Telegram,
    #[sea_orm(string_value = "SingleFileZ")]
    SingleFileZ,
    #[sea_orm(string_value = "Scrapbook")]
    Scrapbook,
    #[sea_orm(string_value = "OneTab")]
    OneTab,
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
