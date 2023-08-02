use axum::{
    extract::{Path, State},
    response::Response,
};

use chrono::NaiveDateTime;
use hyper::StatusCode;
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;

use crate::{
    err::{ErrorStatusCode, ToJsonResultResponse},
    AppState,
};
use entity::{
    file, node,
    types::{NodeType, PathBufSql},
};

#[derive(Debug, Error)]
enum NodeDataError {
    #[error("Node not found: {id}")]
    NodeNotFound { id: i32 },
    #[error("{0}")]
    DbError(#[from] sea_orm::error::DbErr),
}

#[derive(Debug, Serialize)]
struct NodeFull {
    id: i32,
    file_id: Option<i32>,
    file_path: Option<PathBufSql>,
    node_type: NodeType,
    title: Option<String>,
    subtype: Option<String>,
    url: Option<String>,
    icon: Option<String>,
    created: Option<NaiveDateTime>,
    modified: Option<NaiveDateTime>,
    original_id: Option<String>,
    parent_id: Option<i32>,
    #[serde(flatten)]
    data: Option<serde_json::Value>,
}

impl From<(node::Model, Option<file::Model>)> for NodeFull {
    fn from((node, file): (node::Model, Option<file::Model>)) -> Self {
        NodeFull {
            id: node.id,
            file_id: node.file_id,
            file_path: file.map(|file| file.path),
            node_type: node.node_type,
            title: node.title,
            subtype: node.subtype,
            url: node.url,
            icon: node.icon,
            created: node.created,
            modified: node.modified,
            original_id: node.original_id,
            parent_id: node.parent_id,
            data: node
                .data
                .map(|data| json!({ node.node_type.field_name(): data })),
        }
    }
}

async fn get_node_full(db: &DatabaseConnection, id: i32) -> Result<NodeFull, NodeDataError> {
    Ok(node::Entity::find_by_id(id)
        .find_also_related(file::Entity)
        .one(db)
        .await?
        .ok_or(NodeDataError::NodeNotFound { id })?
        .into())
}

impl ErrorStatusCode for NodeDataError {
    fn status_code(&self) -> StatusCode {
        match self {
            NodeDataError::NodeNotFound { .. } => StatusCode::NOT_FOUND,
            NodeDataError::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub async fn get_node_full_handler(state: State<AppState>, Path(id): Path<i32>) -> Response {
    get_node_full(&state.db, id).await.to_json_result_response()
}
