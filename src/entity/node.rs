//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use super::types::{AttachedTableType, NodeType, RelativePathSql, SourceFolderType};
use chrono::naive::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "node")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub r#type: NodeType,
    pub source_folder: Option<SourceFolderType>,
    pub attached_table: Option<AttachedTableType>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub created: Option<NaiveDateTime>,
    pub file: Option<RelativePathSql>,
    pub original_id: Option<String>,
    pub parent_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::ParentId",
        to = "Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    SelfRef,
    #[sea_orm(has_many = "super::scrapbook::Entity")]
    Scrapbook,
    #[sea_orm(has_many = "super::telegram::Entity")]
    Telegram,
}

impl Related<super::telegram::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Telegram.def()
    }
}

impl Related<super::scrapbook::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Scrapbook.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
