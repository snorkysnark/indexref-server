pub mod path;

use std::path::PathBuf;

use axum::{http::StatusCode, response::IntoResponse};

#[justerror::Error]
pub enum AppError {
    ProjectDirsNotFound,
    NonUtf8Path(PathBuf),
    DbErr(#[from] sea_orm::DbErr),
    IoErr(#[from] std::io::Error),
}

pub type AppResult<T> = Result<T, AppError>;

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}
