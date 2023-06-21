use chrono::NaiveDateTime;
use eyre::ContextCompat;
use sea_orm::{FromQueryResult, QueryResult};
use serde::Serialize;
use serde_json::json;

use crate::entity::{node, types::NodeType};

#[derive(Clone, Debug, Serialize)]
pub struct NodePresentation {
    id: i32,
    r#type: NodeType,
    subtype: Option<String>,
    title: Option<String>,
    url: Option<String>,
    icon: Option<String>,
    created: Option<NaiveDateTime>,
    modified: Option<NaiveDateTime>,
    file: Option<String>,
    original_id: Option<String>,
    parent_id: Option<i32>,
    #[serde(flatten)]
    data: Option<serde_json::Value>,
}

impl node::Model {
    pub fn into_presentation(self) -> eyre::Result<NodePresentation> {
        let file_proxy = match self.file {
            Some(rel_path) => {
                let source_folder = self
                    .r#type
                    .source_folder_type()
                    .context("Node has no source folder")?;

                let file_proxy = ["files", source_folder.url_name()]
                    .into_iter()
                    .chain(rel_path.0.components().map(|component| component.as_str()))
                    .map(|segment| format!("/{}", urlencoding::encode(segment)))
                    .collect();

                Some(file_proxy)
            }
            None => None,
        };

        Ok(NodePresentation {
            id: self.id,
            r#type: self.r#type,
            subtype: self.subtype,
            title: self.title,
            url: self.url,
            icon: self.icon,
            created: self.created,
            modified: self.modified,
            file: file_proxy,
            original_id: self.original_id,
            parent_id: self.parent_id,
            data: self
                .data
                .map(|data| json!({ self.r#type.field_name(): data })),
        })
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct NodePresentationWithChildren {
    #[serde(flatten)]
    node: NodePresentation,
    children: Vec<i32>,
}

#[derive(Debug, Serialize)]
pub struct NodeWithChildren {
    #[serde(flatten)]
    node: node::Model,
    children: Vec<i32>,
}

impl FromQueryResult for NodeWithChildren {
    fn from_query_result(res: &QueryResult, pre: &str) -> Result<Self, migration::DbErr> {
        Ok(NodeWithChildren {
            node: node::Model::from_query_result(res, pre)?,
            children: res.try_get(pre, "children")?,
        })
    }
}

impl NodeWithChildren {
    pub fn into_presentation(self) -> eyre::Result<NodePresentationWithChildren> {
        Ok(NodePresentationWithChildren {
            node: self.node.into_presentation()?,
            children: self.children,
        })
    }
}
