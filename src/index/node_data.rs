use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Serialize;
use thiserror::Error;

use crate::{
    config::SourcesConfig,
    entity::{node, scrapbook, telegram, types::AttachedTableType, zotero},
    AppState,
};

use super::node_presentation::NodePresentation;

#[derive(Debug, Serialize)]
pub struct NodeExpanded<M> {
    node: M,
    data: Option<NodeData>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum NodeData {
    Telegram(telegram::Model),
    Scrapbook(scrapbook::Model),
    Zotero(zotero::Model),
}

#[derive(Debug, Error)]
pub enum NodeDataError {
    #[error("Node not found: {id}")]
    NodeNotFound { id: i32 },
    #[error("Node data not found: {id} {attached_table:?}")]
    NodeDataNotFound {
        id: i32,
        attached_table: AttachedTableType,
    },
    #[error("{0}")]
    DbError(#[from] sea_orm::error::DbErr),
    #[error("{0}")]
    NodePresentationError(#[source] eyre::Report),
}

pub async fn get_node_full(
    db: &DatabaseConnection,
    sources: &SourcesConfig,
    id: i32,
) -> Result<NodeExpanded<NodePresentation>, NodeDataError> {
    let node_model = node::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(NodeDataError::NodeNotFound { id })?;

    let node_data = match node_model.r#type.attached_table_type() {
        Some(table_type) => {
            let node_data = match table_type {
                AttachedTableType::Telegram => telegram::Entity::find_by_id(id)
                    .one(db)
                    .await?
                    .map(NodeData::Telegram),
                AttachedTableType::Scrapbook => scrapbook::Entity::find_by_id(id)
                    .one(db)
                    .await?
                    .map(NodeData::Scrapbook),
                AttachedTableType::Zotero => zotero::Entity::find_by_id(id)
                    .one(db)
                    .await?
                    .map(NodeData::Zotero),
            };

            Some(node_data.ok_or(NodeDataError::NodeDataNotFound {
                id,
                attached_table: table_type,
            })?)
        }
        None => None,
    };

    Ok(NodeExpanded {
        node: node_model
            .into_presentation(sources)
            .map_err(NodeDataError::NodePresentationError)?,
        data: node_data,
    })
}

pub async fn get_node_full_handler(state: State<AppState>, Path(id): Path<i32>) -> Response {
    match get_node_full(&state.db, &state.sources, id).await {
        Ok(value) => Json(value).into_response(),
        Err(err) => {
            let status = match err {
                NodeDataError::NodeNotFound { .. } | NodeDataError::NodeDataNotFound { .. } => {
                    StatusCode::NOT_FOUND
                }
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };

            (status, err.to_string()).into_response()
        }
    }
}
