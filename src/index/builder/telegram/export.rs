use serde::{Deserialize, Serialize};
use serde_json::{Map as JsonMap, Value as JsonValue};

use crate::index::NodeData;

use super::raw::TextEntity;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TelegramData {
    pub chat_name: Option<String>,
    pub chat_type: String,
    pub chat_id: i64,
    pub full_text: String,
    pub text_entities: Vec<TextEntity>,
    pub photo: Option<String>,
    pub file: Option<String>,
    #[serde(flatten)]
    pub other: JsonMap<String, JsonValue>,
}

impl From<TelegramData> for NodeData {
    fn from(value: TelegramData) -> Self {
        NodeData::Telegram(value)
    }
}
