use migration::{Migrator, MigratorTrait};
use sea_orm::{DatabaseConnection, FromJsonQueryResult};
use serde::{Deserialize, Serialize};

use crate::{config::SourcesConfig, entity::node};

mod onetab;
mod scrapbook;
mod single_file_z;
mod telegram;
// mod zotero;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
#[serde(rename_all = "lowercase")]
pub enum NodeData {
    Telegram(telegram::TelegramData),
    Scrapbook(scrapbook::ScrapbookData),
}

pub async fn rebuild_index(
    db: &DatabaseConnection,
    sources: &SourcesConfig,
) -> eyre::Result<Vec<node::Model>> {
    // Clear existing index
    Migrator::fresh(db).await?;

    let mut inserted_nodes = vec![];

    if let Some(telegram_chat) = sources.telegram_chat() {
        inserted_nodes.append(&mut self::telegram::insert_from_folder(db, telegram_chat).await?);
    }
    if let Some(single_file_z) = sources.single_file_z() {
        inserted_nodes
            .append(&mut self::single_file_z::insert_from_folder(db, single_file_z).await?);
    }
    if let Some(scrapbook) = sources.scrapbook() {
        inserted_nodes.append(&mut self::scrapbook::insert_from_folder(db, scrapbook).await?);
    }
    if let Some(onetab) = sources.onetab() {
        inserted_nodes.append(&mut self::onetab::insert_from_folder(db, onetab).await?);
    }
    // if let Some(zotero) = sources.zotero() {
    //     for source in zotero {
    //         inserted_nodes.append(&mut self::zotero::insert_from_source(db, source).await?);
    //     }
    // }

    Ok(inserted_nodes)
}
