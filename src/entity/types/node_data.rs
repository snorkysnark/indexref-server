use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};
use serde_json::{Map as JsonMap, Value as JsonValue};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
#[serde(rename_all = "lowercase")]
pub enum NodeData {
    Telegram(TelegramData),
    Scrapbook(ScrapbookData),
}

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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextEntity {
    pub r#type: String,
    pub text: String,
    pub href: Option<String>,
    #[serde(flatten)]
    pub other: JsonMap<String, JsonValue>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScrapbookData {
    pub about: String,
    pub id: String,
    pub r#type: Option<String>,
    pub title: Option<String>,
    pub chars: Option<String>,
    pub comment: Option<String>,
    pub icon: Option<String>,
    pub source: Option<String>,
}
