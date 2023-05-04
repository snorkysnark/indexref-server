use std::path::PathBuf;

use axum::{http::StatusCode, response::IntoResponse};

use crate::{config::ConfigError, path_convert::PathConvertError};

#[justerror::Error]
pub enum AppError {
    ProjectDirsNotFound,
    NonUtf8Path(PathBuf),
    DbErr(#[from] sea_orm::DbErr),
    IoErr(#[from] std::io::Error),
    JsonErr(#[from] serde_json::Error),
    TomlDeserializationErr(#[from] toml::de::Error),
    DateParseErr(#[from] chrono::ParseError),
    ServerErr(#[from] hyper::Error),
    PathConvertErr(#[from] PathConvertError),
    ConfigErr(#[from] ConfigError),
    IdNotFound { table: &'static str, id: i32 },
}

pub type AppResult<T> = Result<T, AppError>;

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}
