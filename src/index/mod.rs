use axum::{extract::State, Json};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::{config::SourcesConfig, result::AppResult, AppState};
use entity::{node, types::NodeType};

mod single_file_z;
mod telegram;

pub async fn get_nodes(
    db: &DatabaseConnection,
    sources: &SourcesConfig,
) -> AppResult<Vec<node::ModelAbsPath>> {
    let nodes: AppResult<Vec<_>> = node::Entity::find()
        .all(db)
        .await?
        .into_iter()
        .map(|node| {
            let base_path = match node.r#type {
                NodeType::Telegram => sources.telegram_chat_ok()?,
                NodeType::SingleFileZ => sources.single_file_z_ok()?,
            };
            Ok(node.into_abs_path(base_path))
        })
        .collect();

    Ok(nodes?)
}

pub async fn get_nodes_handler(state: State<AppState>) -> AppResult<Json<Vec<node::ModelAbsPath>>> {
    Ok(Json(get_nodes(&state.db, &state.sources).await?))
}

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
    if let Some(single_file_z) = sources.single_file_z() {
        inserted_nodes.append(&mut self::single_file_z::insert_from_file(db, single_file_z).await?);
    }

    Ok(inserted_nodes)
}
