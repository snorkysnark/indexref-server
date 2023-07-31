use serde::{de::IgnoredAny, Deserialize, Serialize};
use serde_json::{Map as JsonMap, Value as JsonValue};

#[derive(Debug, Deserialize)]
pub struct Chat {
    #[serde(flatten)]
    pub metadata: ChatMetadata,
    pub messages: Vec<Message>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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
    pub text_entities: Vec<TextEntity>,
    pub photo: Option<String>,
    pub file: Option<String>,

    // Skip "text" field since it's an array of mixed types and will be rejected by Opensearch
    // Moreover, the same information is avaliable in "text_entities"
    #[serde(skip_serializing)]
    pub text: IgnoredAny,

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

#[derive(Debug, Serialize)]
pub struct MessageExport {
    pub chat: ChatMetadata,
    pub full_text: String,
    #[serde(flatten)]
    pub message: Message,
}
