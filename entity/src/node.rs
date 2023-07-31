//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use super::types::NodeType;
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "node")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub file_id: Option<i32>,
    pub node_type: NodeType,
    pub title: Option<String>,
    pub subtype: Option<String>,
    pub url: Option<String>,
    pub icon: Option<String>,
    pub created: Option<DateTime>,
    pub modified: Option<DateTime>,
    pub original_id: Option<String>,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub data: Option<Json>,
    pub parent_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::file::Entity",
        from = "Column::FileId",
        to = "super::file::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    File,
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::ParentId",
        to = "Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    SelfRef,
}

impl Related<super::file::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::File.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}