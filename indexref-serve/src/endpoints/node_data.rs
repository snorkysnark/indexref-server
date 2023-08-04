use axum::{
    extract::{Path, State},
    response::Response,
};

use hyper::StatusCode;
use sea_orm::{DatabaseConnection, EntityTrait};
use thiserror::Error;

use crate::{
    err::{ErrorStatusCode, ToJsonResultResponse},
    AppState,
};
use entity::{file, node, NodePresentaion};

#[derive(Debug, Error)]
enum NodeDataError {
    #[error("Node not found: {id}")]
    NodeNotFound { id: i32 },
    #[error("{0}")]
    DbError(#[from] sea_orm::error::DbErr),
}

async fn get_node_full(db: &DatabaseConnection, id: i32) -> Result<NodePresentaion, NodeDataError> {
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
