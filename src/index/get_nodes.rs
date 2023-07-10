use std::collections::HashMap;

use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use chrono::NaiveDateTime;
use hyper::StatusCode;
use sea_orm::{DatabaseConnection, FromQueryResult, Statement};
use serde::Serialize;

use crate::{
    entity::types::{NodeType, RelativePathSql},
    AppState,
};

#[derive(Debug, FromQueryResult)]
struct NodeFlat {
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
    children: Vec<i32>,
}

#[derive(Debug, Serialize)]
struct NodeTree {
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
    children: Vec<NodeTree>,
}

impl NodeFlat {
    fn into_tree(self, node_by_id: &mut HashMap<i32, NodeFlat>) -> NodeTree {
        NodeTree {
            id: self.id,
            r#type: self.r#type,
            subtype: self.subtype,
            title: self.title,
            url: self.url,
            icon: self.icon,
            created: self.created,
            modified: self.modified,
            file: self.file,
            original_id: self.original_id,
            children: self
                .children
                .iter()
                .map(|child_id| node_by_id.remove(child_id).unwrap().into_tree(node_by_id))
                .collect(),
        }
    }
}

async fn get_node_tree(db: &DatabaseConnection) -> eyre::Result<Vec<NodeTree>> {
    let select = Statement::from_string(
        sea_orm::DatabaseBackend::Postgres,
        "SELECT
            parent.id,
            parent.type,
            parent.subtype,
            parent.title,
            parent.url,
            parent.icon,
            parent.created,
            parent.modified,
            parent.file,
            parent.original_id,
            array_remove(array_agg(child.id), NULL) AS children
        FROM
            node AS parent
            LEFT JOIN node AS child ON child.parent_id = parent.id
        GROUP BY
            parent.id;"
            .to_owned(),
    );

    let mut root_node: Option<NodeFlat> = None;
    let mut nodes_by_id: HashMap<i32, NodeFlat> = HashMap::new();

    for node in NodeFlat::find_by_statement(select).all(db).await? {
        match node.r#type {
            NodeType::Root => {
                root_node = Some(node);
            }
            _ => {
                nodes_by_id.insert(node.id, node);
            }
        };
    }

    let tree_roots = root_node
        .unwrap()
        .children
        .iter()
        .map(|child_id| {
            nodes_by_id
                .remove(child_id)
                .unwrap()
                .into_tree(&mut nodes_by_id)
        })
        .collect();

    Ok(tree_roots)
}

pub async fn get_node_tree_handler(state: State<AppState>) -> Response {
    match get_node_tree(&state.db).await {
        Ok(value) => Json(value).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}
