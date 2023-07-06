use axum::response::{IntoResponse, Response};
use axum::{extract::State, Json};
use hyper::StatusCode;
use sea_orm::{DatabaseConnection, FromQueryResult, Statement};
use serde::Serialize;

use crate::entity::types::NodeType;
use crate::AppState;

pub use self::builder::*;
pub use self::node_data::{get_node_full, get_node_full_handler};
use self::node_presentation::{NodePresentationWithChildren, NodeWithChildren};
pub use self::serve_file::*;

mod builder;
mod node_data;
mod node_presentation;
mod serve_file;

#[derive(Debug, Serialize)]
pub struct NodeTree {
    root: Vec<i32>,
    nodes: Vec<NodePresentationWithChildren>,
}

pub async fn get_nodes(db: &DatabaseConnection) -> eyre::Result<NodeTree> {
    let select = Statement::from_string(
        sea_orm::DatabaseBackend::Sqlite,
        "select parent.*, array_remove(array_agg(child.id), null) as children
            from node as parent
            left join node as child
            on child.parent_id = parent.id
            group by parent.id;"
            .to_owned(),
    );

    let mut root: Option<Vec<i32>> = None;
    let nodes: Vec<_> = NodeWithChildren::find_by_statement(select)
        .all(db)
        .await?
        .into_iter()
        .filter_map(|row| match row.node.r#type {
            NodeType::Root => {
                root.replace(row.children);
                None
            }
            _ => Some(row.into_presentation()),
        })
        .collect::<eyre::Result<_>>()?;

    Ok(NodeTree {
        root: root.unwrap_or_default(),
        nodes,
    })
}

pub async fn get_nodes_handler(state: State<AppState>) -> Response {
    match get_nodes(&state.db).await {
        Ok(value) => Json(value).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}
