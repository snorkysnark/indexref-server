mod telegram_importer;

use std::path::Path;

use entity::{node, types::FileType};
use eyre::Result;
use sea_orm::DatabaseConnection;

pub async fn import_from_file(
    db: &DatabaseConnection,
    file_type: FileType,
    file_path: &Path,
    id: i32,
) -> Result<Vec<node::Model>> {
    match file_type {
        FileType::Telegram => telegram_importer::import_from_file(db, file_path, id).await,
        FileType::SingleFileZ => todo!(),
        FileType::Scrapbook => todo!(),
        FileType::OneTab => todo!(),
    }
}
