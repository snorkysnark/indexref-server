use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIter, DeriveActiveEnum, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(None)")]
pub enum FileType {
    #[sea_orm(string_value = "Telegram")]
    Telegram,
    #[sea_orm(string_value = "SingleFileZ")]
    SingleFileZ,
    #[sea_orm(string_value = "Scrapbook")]
    Scrapbook,
    #[sea_orm(string_value = "OneTab")]
    OneTab,
}
