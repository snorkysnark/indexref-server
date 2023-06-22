use migration::{Migrator, MigratorTrait};
use opensearch::{
    indices::{IndicesCreateParts, IndicesDeleteParts},
    BulkOperation, BulkOperations, BulkParts, OpenSearch,
};
use sea_orm::DatabaseConnection;
use serde_json::Value as Json;

use crate::{
    config::SourcesConfig,
    entity::{node, types::NodeType},
};

mod onetab;
mod scrapbook;
mod single_file_z;
mod telegram;
mod zotero;

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
    if let Some(zotero) = sources.zotero() {
        for source in zotero {
            inserted_nodes.append(&mut self::zotero::insert_from_source(db, source).await?);
        }
    }

    Ok(inserted_nodes)
}

pub async fn upload_to_opensearch(client: OpenSearch, nodes: Vec<node::Model>) -> eyre::Result<()> {
    println!(
        "{}",
        client
            .indices()
            .delete(IndicesDeleteParts::Index(&["nodes"]))
            .send()
            .await?
            .json::<Json>()
            .await?
    );
    println!(
        "{}",
        client
            .indices()
            .create(IndicesCreateParts::Index("nodes"))
            .send()
            .await?
            .json::<Json>()
            .await?
    );

    let mut ops = BulkOperations::new();
    for node in nodes.into_iter() {
        if node.r#type != NodeType::Root {
            ops.push(
                BulkOperation::create(node.id.to_string(), node.into_presentation()?)
                    .index("nodes"),
            )?;
        }
    }

    println!(
        "{}",
        client
            .bulk(BulkParts::None)
            .body(vec![ops])
            .send()
            .await?
            .json::<Json>()
            .await?
    );

    Ok(())
}
