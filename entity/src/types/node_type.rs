use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::Serialize;

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
            NodeType::OneTab => "one_tab",
            NodeType::Zotero => "zotero",
        }
    }
}
