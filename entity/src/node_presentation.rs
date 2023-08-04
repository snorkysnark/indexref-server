use sea_orm::entity::prelude::*;
use serde::Serialize;
use serde_json::json;

use crate::{
    file, node,
    types::{NodeType, PathBufSql},
};

// Node description presented to the and user and indexed by opensearch
// Used in both 'serve' and 'sync' crates, so keeping it here
#[derive(Debug, Serialize)]
pub struct NodePresentaion {
    id: i32,
    file_id: Option<i32>,
    file_path: Option<PathBufSql>,
    node_type: NodeType,
    title: Option<String>,
    subtype: Option<String>,
    url: Option<String>,
    icon: Option<String>,
    created: Option<DateTime>,
    modified: Option<DateTime>,
    original_id: Option<String>,
    parent_id: Option<i32>,
    #[serde(flatten)]
    data: Option<Json>,
}

impl From<(node::Model, Option<file::Model>)> for NodePresentaion {
    fn from((node, file): (node::Model, Option<file::Model>)) -> Self {
        NodePresentaion {
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
