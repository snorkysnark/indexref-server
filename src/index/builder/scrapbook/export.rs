use serde::{Deserialize, Serialize};

use crate::index::NodeData;

use super::raw::RdfDescription;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RdfDescriptionNullable {
    pub about: String,
    pub id: String,
    pub r#type: Option<String>,
    pub title: Option<String>,
    pub chars: Option<String>,
    pub comment: Option<String>,
    pub icon: Option<String>,
    pub source: Option<String>,
}

impl From<RdfDescription> for RdfDescriptionNullable {
    fn from(value: RdfDescription) -> Self {
        fn none_if_empty(string: String) -> Option<String> {
            match string.as_str() {
                "" => None,
                _ => Some(string),
            }
        }

        Self {
            about: value.about,
            id: value.id,
            r#type: none_if_empty(value.r#type),
            title: none_if_empty(value.title),
            chars: none_if_empty(value.chars),
            comment: none_if_empty(value.comment),
            icon: none_if_empty(value.icon),
            source: none_if_empty(value.source),
        }
    }
}

impl From<RdfDescriptionNullable> for NodeData {
    fn from(value: RdfDescriptionNullable) -> Self {
        NodeData::Scrapbook(value)
    }
}
