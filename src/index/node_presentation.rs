use std::path::{Path, PathBuf};

use chrono::NaiveDateTime;
use eyre::ContextCompat;
use serde::Serialize;

use crate::{
    config::SourcesConfig,
    date_serializer::human_readable_opt,
    entity::{node, types::NodeType},
};

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

#[derive(Clone, Debug, Serialize)]
pub struct NodeRelations {
    parent_id: i32,
    children: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct NodePresentationWithRelations {
    #[serde(flatten)]
    data: NodePresentation,
    #[serde(flatten)]
    rel: NodeRelations,
}

impl node::Model {
    pub fn into_presentation(self, sources: &SourcesConfig) -> eyre::Result<NodePresentation> {
        let (file, file_proxy) = match self.file {
            Some(rel_path) => {
                let container = self
                    .r#type
                    .container_type()
                    .context("Cannot resolve file path")?;
                let base_path = sources.get_base_path(container)?;

                let full_path = rel_path.0.to_path(base_path);
                let file_proxy = ["files", container.url_name()]
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
