use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(None)")]
pub enum NodeType {
    #[sea_orm(string_value = "Telegram")]
    Telegram,
}
