use std::collections::HashMap;

use axum::{extract::State, response::Response};
use chrono::NaiveDateTime;
use eyre::Result;
use futures::{future, TryStreamExt};
use sea_orm::{DatabaseConnection, FromQueryResult, Statement};
use serde::Serialize;

use crate::{err::ToJsonResultResponse, AppState};
use entity::types::{NodeType, PathBufSql};

#[derive(Debug, FromQueryResult)]
struct NodeFlat {
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
    children: Vec<i32>,
}

#[derive(Debug, Serialize)]
struct NodeTree {
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
    children: Vec<NodeTree>,
}

impl NodeFlat {
    fn into_tree(self, node_by_id: &mut HashMap<i32, NodeFlat>) -> NodeTree {
        NodeTree {
            id: self.id,
            file_id: self.file_id,
            file_path: self.file_path,
            node_type: self.node_type,
            title: self.title,
            subtype: self.subtype,
            url: self.url,
            icon: self.icon,
            created: self.created,
            modified: self.modified,
            original_id: self.original_id,
            parent_id: self.parent_id,
            children: self
                .children
                .iter()
                .map(|child_id| node_by_id.remove(child_id).unwrap().into_tree(node_by_id))
                .collect(),
        }
    }
}

async fn get_node_tree(db: &DatabaseConnection) -> Result<Vec<NodeTree>> {
    let select = Statement::from_string(
        sea_orm::DatabaseBackend::Postgres,
        include_str!("./node_tree.sql").to_owned(),
    );

    let mut root_node: Option<NodeFlat> = None;
    let mut nodes_by_id: HashMap<i32, NodeFlat> = HashMap::new();

    NodeFlat::find_by_statement(select)
        .stream(db)
        .await?
        .try_for_each(|node| {
            if node.node_type == NodeType::Root {
                root_node = Some(node);
            } else {
                nodes_by_id.insert(node.id, node);
            }
            future::ready(Ok(()))
        })
        .await?;

    let tree = root_node
        .expect("Root node should always exist")
        .children
        .iter()
        .map(|child_id| {
            nodes_by_id
                .remove(child_id)
                .unwrap()
                .into_tree(&mut nodes_by_id)
        })
        .collect();

    Ok(tree)
}

pub async fn get_node_tree_handler(state: State<AppState>) -> Response {
    get_node_tree(&state.db).await.to_json_result_response()
}
