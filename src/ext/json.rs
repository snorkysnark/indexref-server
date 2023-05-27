use thiserror::Error;

#[derive(Debug, Error)]
pub enum JsonExtError {
    #[error("Missing json field: {0}")]
    MissingField(String),
    #[error("Not a string: {0}")]
    NotAString(serde_json::Value),
}

pub trait JsonValueExt {
    fn get_ok(&self, key: &str) -> Result<&serde_json::Value, JsonExtError>;
    fn as_str_ok(&self) -> Result<&str, JsonExtError>;
}

impl JsonValueExt for serde_json::Value {
    fn as_str_ok(&self) -> Result<&str, JsonExtError> {
        self.as_str()
            .ok_or_else(|| JsonExtError::NotAString(self.clone()))
    }

    fn get_ok(&self, key: &str) -> Result<&serde_json::Value, JsonExtError> {
        self.get(key)
            .ok_or_else(|| JsonExtError::MissingField(key.to_owned()))
    }
}
