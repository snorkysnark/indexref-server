use axum::{extract::State, Json};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::{config::SourcesConfig, result::AppResult, AppState};
use entity::node;

mod telegram;

pub async fn get_nodes(
    db: &DatabaseConnection,
    sources: &SourcesConfig,
) -> AppResult<Vec<node::ModelAbsPath>> {
    let nodes: Vec<_> = node::Entity::find()
        .all(&*db)
        .await?
        .into_iter()
        .map(|node| {
            // TODO: turn node.type into an enum, replace unwrap with proper error
            let base_path = match node.r#type.as_str() {
                "Telegram" => sources.telegram_chat().unwrap(),
                _ => unreachable!(),
            };
            node.into_abs_path(base_path)
        })
        .collect();

    Ok(nodes)
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

    Ok(inserted_nodes)
}
