use axum::response::{IntoResponse, Response};
use axum::{extract::State, Json};
use hyper::StatusCode;
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::{config::SourcesConfig, entity::node, AppState};

pub use self::node_data::{get_node_full, get_node_full_handler};
pub use self::serve_file::*;

mod node_data;
mod scrapbook;
mod serve_file;
mod single_file_z;
mod telegram;

pub async fn get_nodes(
    db: &DatabaseConnection,
    sources: &SourcesConfig,
) -> eyre::Result<Vec<node::ModelAbsPath>> {
    let nodes: eyre::Result<Vec<_>> = node::Entity::find()
        .all(db)
        .await?
        .into_iter()
        .map(|node| {
            let base_path = sources.get_base_path(node.r#type)?;
            Ok(node.into_abs_path(base_path))
        })
        .collect();

    Ok(nodes?)
}

pub async fn get_nodes_handler(state: State<AppState>) -> Response {
    match get_nodes(&state.db, &state.sources).await {
        Ok(value) => Json(value).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

pub async fn rebuild_index(
    db: &DatabaseConnection,
    sources: &SourcesConfig,
) -> eyre::Result<Vec<node::Model>> {
    // Clear existing index
    node::Entity::delete_many().exec(db).await?;

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

    Ok(inserted_nodes)
}
