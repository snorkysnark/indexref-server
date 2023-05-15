use std::path::PathBuf;

use chrono::NaiveDateTime;
use eyre::ContextCompat;
use sea_orm::{FromQueryResult, QueryResult};
use serde::Serialize;

use crate::{
    config::SourcesConfig,
    date_serializer::human_readable_opt,
    entity::{node, types::NodeType},
};

use super::types::StringVec;

#[derive(Clone, Debug, Serialize)]
pub struct NodePresentation {
    pub id: i32,
    pub r#type: NodeType,
    pub title: Option<String>,
    pub url: Option<String>,
    #[serde(serialize_with = "human_readable_opt")]
    pub created: Option<NaiveDateTime>,
    pub file: Option<PathBuf>,
    pub file_proxy: Option<String>,
    pub original_id: Option<String>,
}

impl node::Model {
    pub fn into_presentation(self, sources: &SourcesConfig) -> eyre::Result<NodePresentation> {
        let (file, file_proxy) = match self.file {
            Some(rel_path) => {
                let source_folder = self.source_folder.context("Node has no source folder")?;
                let base_path = sources.get_base_path(source_folder)?;

                let full_path = rel_path.0.to_path(base_path);
                let file_proxy = ["files", source_folder.url_name()]
                    .into_iter()
                    .chain(rel_path.0.components().map(|component| component.as_str()))
                    .map(|segment| format!("/{}", urlencoding::encode(segment)))
                    .collect();

                (Some(full_path), Some(file_proxy))
            }
            None => (None, None),
        };

        Ok(NodePresentation {
            id: self.id,
            r#type: self.r#type,
            title: self.title,
            url: self.url,
            created: self.created,
            file,
            file_proxy,
            original_id: self.original_id,
        })
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct NodeRelations {
    parent_id: Option<i32>,
    children: Vec<i32>,
}

#[derive(Clone, Debug, Serialize)]
pub struct NodePresentationWithRelations {
    #[serde(flatten)]
    node: NodePresentation,
    #[serde(flatten)]
    rel: NodeRelations,
}

#[derive(Debug, Serialize)]
pub struct NodeWithChildren {
    #[serde(flatten)]
    node: node::Model,
    children: StringVec<i32>,
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
    pub fn into_presentation(
        self,
        sources: &SourcesConfig,
    ) -> eyre::Result<NodePresentationWithRelations> {
        let rel = NodeRelations {
            parent_id: self.node.parent_id,
            children: self.children.into(),
        };

        Ok(NodePresentationWithRelations {
            node: self.node.into_presentation(sources)?,
            rel,
        })
    }
}
