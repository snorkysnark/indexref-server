mod raw;

use std::{fs, path::Path};

use chrono::{TimeZone, Utc};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use walkdir::WalkDir;

use self::raw::{Chat, ChatMetadata, Message, ParsedAndRaw};
use crate::{
    ext::{PathExt, ResultExt},
    result::AppResult,
};
use entity::{node, telegram};

async fn insert_one(
    db: &DatabaseConnection,
    metadata: ChatMetadata,
    message: ParsedAndRaw<Message>,
) -> AppResult<node::Model> {
    let full_text: String = message
        .parsed
        .text_entities
        .iter()
        .map(|entity| entity.text.as_str())
        .collect();

    let created = Utc.datetime_from_str(&message.parsed.date, "%Y-%m-%dT%H:%M:%S")?;

    let inserted_node = node::ActiveModel {
        r#type: Set("Telegram".to_owned()),
        title: Set(Some(full_text)),
        created: Set(Some(created)),
        ..Default::default()
    }
    .insert(db)
    .await?;

    telegram::ActiveModel {
        node_id: Set(inserted_node.id),
        chat_name: Set(metadata.name),
        chat_type: Set(metadata.r#type),
        chat_id: Set(metadata.id),
        message: Set(message.raw),
    }
    .insert(db)
    .await?;

    Ok(inserted_node)
}

pub async fn insert_from_file(
    db: &DatabaseConnection,
    folder: &Path,
) -> AppResult<Vec<node::Model>> {
    let mut inserted_nodes = vec![];

    for entry in WalkDir::new(folder)
        .into_iter()
        .filter_map(|result| result.ok_log_errors())
        .filter(|e| matches!(e.path().extension_str(), Some("json")))
    {
        let chat: Chat = serde_json::from_str(&fs::read_to_string(entry.path())?)?;
        for message in chat.messages {
            // Skip non-text messages (may change in the future)
            if message.parsed.text_entities.len() == 0 {
                continue;
            }

            inserted_nodes.push(insert_one(db, chat.metadata.clone(), message).await?);
        }
    }

    Ok(inserted_nodes)
}
