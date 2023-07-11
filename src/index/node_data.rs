use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    Json,
};

use chrono::NaiveDateTime;
use hyper::StatusCode;
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;

use crate::{
    entity::{
        node,
        types::{NodeType, RelativePathSql},
    },
    AppState,
};

#[derive(Debug, Error)]
enum NodeDataError {
    #[error("Node not found: {id}")]
    NodeNotFound { id: i32 },
    #[error("{0}")]
    DbError(#[from] sea_orm::error::DbErr),
}

#[derive(Debug, Serialize)]
struct NodeWithData {
    id: i32,
    r#type: NodeType,
    subtype: Option<String>,
    title: Option<String>,
    url: Option<String>,
    icon: Option<String>,
    created: Option<NaiveDateTime>,
    modified: Option<NaiveDateTime>,
    file: Option<RelativePathSql>,
    original_id: Option<String>,
    parent_id: Option<i32>,
    #[serde(flatten)]
    data: Option<serde_json::Value>,
}

impl From<node::Model> for NodeWithData {
    fn from(value: node::Model) -> Self {
        Self {
            id: value.id,
            r#type: value.r#type,
            subtype: value.subtype,
            title: value.title,
            url: value.url,
            icon: value.icon,
            created: value.created,
            modified: value.modified,
            file: value.file,
            original_id: value.original_id,
            parent_id: value.parent_id,
            data: value
                .data
                .map(|data| json!({ value.r#type.field_name(): data })),
        }
    }
}

async fn get_node_full(db: &DatabaseConnection, id: i32) -> Result<NodeWithData, NodeDataError> {
    Ok(node::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(NodeDataError::NodeNotFound { id })?
        .into())
}

pub async fn get_node_full_handler(state: State<AppState>, Path(id): Path<i32>) -> Response {
    match get_node_full(&state.db, id).await {
        Ok(value) => Json(value).into_response(),
        Err(err) => {
            let status = match err {
                NodeDataError::NodeNotFound { .. } => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };

            (status, err.to_string()).into_response()
        }
    }
}
