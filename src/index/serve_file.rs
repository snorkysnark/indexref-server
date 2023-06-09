use axum::{
    body::StreamBody,
    extract::{Path, State},
    headers::ContentType,
    response::{IntoResponse, Response},
    TypedHeader,
};
use hyper::StatusCode;
use relative_path::RelativePathBuf;
use thiserror::Error;
use tokio_util::io::ReaderStream;

use crate::{config::BasePathError, entity::types::SourceFolderType, AppState};

#[derive(Debug, Error)]
pub enum ServeFileError {
    #[error("Unknown container name: {0}")]
    UnknownContainerName(String),
    #[error("{0}")]
    BasePath(#[from] BasePathError),
    #[error("{0}")]
    IoErr(#[from] std::io::Error),
    #[error("This is a directory, not a file")]
    PathIsDir,
}

async fn serve_file(
    state: State<AppState>,
    Path((type_name, rel_path)): Path<(String, RelativePathBuf)>,
) -> Result<impl IntoResponse, ServeFileError> {
    let base_path = state.sources.get_base_path(
        SourceFolderType::from_url_name(&type_name)
            .ok_or(ServeFileError::UnknownContainerName(type_name))?,
    )?;
    let full_path = rel_path.to_path(base_path);

    if full_path.is_dir() {
        return Err(ServeFileError::PathIsDir);
    }

    let mime = mime_guess::from_path(&full_path).first_or_text_plain();

    let file = fs_err::tokio::File::open(&full_path).await?;
    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);

    Ok((TypedHeader(ContentType::from(mime)), body))
}

pub async fn serve_file_handler(
    state: State<AppState>,
    path: Path<(String, RelativePathBuf)>,
) -> Response {
    match serve_file(state, path).await {
        Ok(value) => value.into_response(),
        Err(err) => (StatusCode::NOT_FOUND, err.to_string()).into_response(),
    }
}
