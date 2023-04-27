use std::path::Path;

use sea_orm::{DatabaseConnection, EntityTrait};

use crate::result::AppResult;
use entity::node;

mod telegram;

pub async fn rebuild_index(
    db: &DatabaseConnection,
    telegram_folder: &Path,
) -> AppResult<Vec<node::Model>> {
    // Clear existing index
    node::Entity::delete_many().exec(db).await?;

    Ok(self::telegram::insert_from_file(db, telegram_folder).await?)
}
