use axum::{
    extract::{Path, State},
    Json,
};
use entity::{node, telegram, types::NodeType};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Serialize;

use crate::{
    config::SourcesConfig,
    result::{AppError, AppResult},
    AppState,
};

#[derive(Debug, Serialize)]
pub struct NodeExpanded<M> {
    node: M,
    data: NodeData,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum NodeData {
    Telegram(telegram::Model),
    Empty,
}

pub async fn get_node_full(
    db: &DatabaseConnection,
    sources: &SourcesConfig,
    id: i32,
) -> AppResult<NodeExpanded<node::ModelAbsPath>> {
    let node_model = node::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(AppError::IdNotFound { table: "node", id })?;
    let node_data = match node_model.r#type {
        NodeType::Telegram => {
            NodeData::Telegram(telegram::Entity::find_by_id(id).one(db).await?.ok_or(
                AppError::IdNotFound {
                    table: "telegram",
                    id,
                },
            )?)
        }
        _ => NodeData::Empty,
    };

    let base_path = sources.get_base_path(node_model.r#type)?;
    Ok(NodeExpanded {
        node: node_model.into_abs_path(base_path),
        data: node_data,
    })
}

pub async fn get_node_full_handler(
    state: State<AppState>,
    Path(id): Path<i32>,
) -> AppResult<Json<NodeExpanded<node::ModelAbsPath>>> {
    Ok(Json(get_node_full(&state.db, &state.sources, id).await?))
}
