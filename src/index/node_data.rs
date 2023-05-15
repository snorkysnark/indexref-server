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
    entity::{node, telegram, types::NodeType},
    AppState,
};

use super::node_presentation::NodePresentation;

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

#[derive(Debug, Error)]
pub enum NodeDataError {
    #[error("Node not found: {id}")]
    NodeNotFound { id: i32 },
    #[error("Node data not found: {id} {node_type}")]
    NodeDataNotFound { id: i32, node_type: NodeType },
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
    let node_data = match node_model.r#type {
        NodeType::Telegram => {
            NodeData::Telegram(telegram::Entity::find_by_id(id).one(db).await?.ok_or(
                NodeDataError::NodeDataNotFound {
                    id,
                    node_type: node_model.r#type,
                },
            )?)
        }
        _ => NodeData::Empty,
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
