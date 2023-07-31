mod scrapbook;
mod single_file_z;
mod telegram;

use std::path::Path;

use entity::{node, types::FileType};
use eyre::{Context, Result};
use sea_orm::DatabaseConnection;

pub async fn import_from_file(
    db: &DatabaseConnection,
    file_type: FileType,
    file_path: &Path,
    id: i32,
) -> Result<Vec<node::Model>> {
    match file_type {
        FileType::Telegram => telegram::import_from_file(db, file_path, id).await,
        FileType::SingleFileZ => single_file_z::import_from_file(db, file_path, id).await,
        FileType::Scrapbook => scrapbook::import_from_file(db, file_path, id).await,
        FileType::OneTab => todo!(),
    }
    .with_context(|| format!("Error importing file {}", file_path.display()))
}
