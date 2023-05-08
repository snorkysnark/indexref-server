use axum::{
    body::StreamBody,
    extract::{Path, State},
    headers::ContentType,
    response::IntoResponse,
    TypedHeader,
};
use relative_path::RelativePathBuf;
use tokio_util::io::ReaderStream;

use crate::{
    entity::types::NodeType,
    result::{AppError, AppResult},
    AppState,
};

pub async fn serve_file_handler(
    state: State<AppState>,
    Path((type_name, rel_path)): Path<(String, RelativePathBuf)>,
) -> AppResult<impl IntoResponse> {
    let node_type = match type_name.as_str() {
        "telegram" => NodeType::Telegram,
        "single_file_z" => NodeType::SingleFileZ,
        "scrapbook" => NodeType::ScrapbookPage,
        _ => return Err(AppError::UnknownBaseFolder(type_name)),
    };

    let base_path = state.sources.get_base_path(node_type)?;
    let full_path = rel_path.to_path(base_path);

    let mime = mime_guess::from_path(&full_path).first_or_text_plain();

    let file = tokio::fs::File::open(&full_path).await?;
    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);

    Ok((TypedHeader(ContentType::from(mime)), body))
}
