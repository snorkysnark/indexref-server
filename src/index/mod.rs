use sea_orm::{DatabaseConnection, EntityTrait};

use crate::{config::SourcesConfig, result::AppResult};
use entity::node;

mod telegram;

pub async fn rebuild_index(
    db: &DatabaseConnection,
    sources: &SourcesConfig,
) -> AppResult<Vec<node::Model>> {
    // Clear existing index
    node::Entity::delete_many().exec(db).await?;

    let mut inserted_nodes = vec![];

    if let Some(telegram_chat) = sources.telegram_chat() {
        inserted_nodes.append(&mut self::telegram::insert_from_file(db, telegram_chat).await?);
    }

    Ok(inserted_nodes)
}
