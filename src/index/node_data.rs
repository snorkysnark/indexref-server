use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    Json,
};

use hyper::StatusCode;
use sea_orm::{DatabaseConnection, EntityTrait};
use thiserror::Error;

use crate::{entity::node, AppState};

use super::node_presentation::NodePresentation;

#[derive(Debug, Error)]
pub enum NodeDataError {
    #[error("Node not found: {id}")]
    NodeNotFound { id: i32 },
    #[error("{0}")]
    DbError(#[from] sea_orm::error::DbErr),
    #[error("{0}")]
    NodePresentationError(#[source] eyre::Report),
}

pub async fn get_node_full(
    db: &DatabaseConnection,
    id: i32,
) -> Result<NodePresentation, NodeDataError> {
    Ok(node::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(NodeDataError::NodeNotFound { id })?
        .into_presentation()
        .map_err(NodeDataError::NodePresentationError)?)
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
