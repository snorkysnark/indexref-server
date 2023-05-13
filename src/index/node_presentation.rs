use std::path::{PathBuf, Path};

use chrono::NaiveDateTime;
use serde::Serialize;

use crate::{date_serializer::human_readable_opt, entity::{node, types::NodeType}};

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
    pub fn into_presentation(self, base: &Path) -> NodePresentation {
        NodePresentation {
            id: self.id,
            r#type: self.r#type,
            title: self.title,
            url: self.url,
            created: self.created,
            file: self.file.as_ref().map(|rel_path| rel_path.0.to_path(base)),
            file_proxy: self.file.as_ref().map(|rel_path| {
                // Construct relative url
                ["files", self.r#type.container_type().url_name()]
                    .into_iter()
                    .chain(rel_path.0.components().map(|component| component.as_str()))
                    .map(|segment| format!("/{}", urlencoding::encode(segment)))
                    .collect()
            }),
            original_id: self.original_id,
        }
    }
}
