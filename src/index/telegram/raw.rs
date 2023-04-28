use serde::{
    de::{DeserializeOwned, Error as _},
    Deserialize,
};

#[derive(Debug)]
pub struct ParsedAndRaw<T> {
    // The full json value (we don't know the exact structure of it)
    pub raw: serde_json::Value,
    // Relevant data
    pub parsed: T,
}

impl<'de, T> Deserialize<'de> for ParsedAndRaw<T>
where
    T: DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = serde_json::Value::deserialize(deserializer)?;

        Ok(Self {
            parsed: serde_json::from_value(raw.clone()).map_err(D::Error::custom)?,
            raw,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct Chat {
    #[serde(flatten)]
    pub metadata: ChatMetadata,
    pub messages: Vec<ParsedAndRaw<Message>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChatMetadata {
    pub name: Option<String>,
    pub r#type: String,
    pub id: i64,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub id: i64,
    pub date: String,
    pub date_unixtime: String,
    pub text_entities: Vec<TextEntity>,
}

#[derive(Debug, Deserialize)]
pub struct TextEntity {
    pub r#type: String,
    pub text: String,
    #[serde(default)]
    pub href: Option<String>,
}
