use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(None)")]
pub enum NodeType {
    #[sea_orm(string_value = "Telegram")]
    Telegram,
    #[sea_orm(string_value = "SingleFileZ")]
    SingleFileZ,
    #[sea_orm(string_value = "ScrapbookPage")]
    ScrapbookPage,
    #[sea_orm(string_value = "ScrapbookFile")]
    ScrapbookFile,
}

impl NodeType {
    pub fn container_type(self) -> ContainerType {
        match self {
            NodeType::Telegram => ContainerType::Telegram,
            NodeType::SingleFileZ => ContainerType::SingleFileZ,
            NodeType::ScrapbookPage | NodeType::ScrapbookFile => ContainerType::Scrapbook,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ContainerType {
    Telegram,
    SingleFileZ,
    Scrapbook,
}
