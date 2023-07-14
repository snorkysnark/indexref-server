use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ZoteroSource {
    api_key: Option<String>,
    #[serde(flatten)]
    source_type: ZoteroSourceType,
}

impl ZoteroSource {
    pub fn api_key(&self) -> Option<&str> {
        self.api_key.as_deref()
    }

    pub fn source_type(&self) -> &ZoteroSourceType {
        &self.source_type
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ZoteroSourceType {
    User(i32),
    Group(i32),
}
