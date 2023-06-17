use serde::{Deserialize, Serialize};
use serde_json::{Map as JsonMap, Value as JsonValue};

#[derive(Debug, Deserialize)]
pub struct Chat {
    #[serde(flatten)]
    pub metadata: ChatMetadata,
    pub messages: Vec<Message>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChatMetadata {
    pub name: Option<String>,
    pub r#type: String,
    pub id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: i64,
    pub r#type: String,
    pub date: String,
    pub edited: Option<String>,
    pub text: JsonValue,
    pub text_entities: Vec<TextEntity>,
    pub photo: Option<String>,
    pub file: Option<String>,
    #[serde(flatten)]
    pub other: JsonMap<String, JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextEntity {
    pub r#type: String,
    pub text: String,
    pub href: Option<String>,
    #[serde(flatten)]
    pub other: JsonMap<String, JsonValue>,
}

impl From<TextEntity> for crate::entity::types::TextEntity {
    fn from(value: TextEntity) -> Self {
        Self {
            r#type: value.r#type,
            text: value.text,
            href: value.href,
            other: value.other,
        }
    }
}
