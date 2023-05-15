mod raw;

use std::{fs, path::Path};

use chrono::NaiveDateTime;
use relative_path::RelativePathBuf;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use walkdir::WalkDir;

use self::raw::{Chat, ChatMetadata, Message, ParsedAndRaw};
use crate::{
    entity::{
        node, telegram,
        types::{AttachedTableType, NodeType, SourceFolderType},
    },
    ext::{PathExt, ResultExt},
    path_convert::ToRelativePath,
};

async fn insert_message(
    db: &DatabaseConnection,
    metadata: ChatMetadata,
    relative_path: RelativePathBuf,
    message: ParsedAndRaw<Message>,
) -> eyre::Result<node::Model> {
    let full_text: String = message
        .parsed
        .text_entities
        .iter()
        .map(|entity| entity.text.as_str())
        .collect();

    let url = message.parsed.text_entities.iter().find_map(|block| {
        if block.r#type == "link" {
            Some(block.text.clone())
        } else {
            block.href.clone()
        }
    });
    let created = NaiveDateTime::parse_from_str(&message.parsed.date, "%Y-%m-%dT%H:%M:%S")?;
    let message_id = message.parsed.id.to_string();

    let inserted_node = node::ActiveModel {
        r#type: Set(NodeType::Telegram),
        source_folder: Set(Some(SourceFolderType::Telegram)),
        attached_table: Set(Some(AttachedTableType::Telegram)),
        title: Set(Some(full_text)),
        url: Set(url),
        created: Set(Some(created)),
        file: Set(Some(relative_path.into())),
        original_id: Set(Some(message_id)),
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

pub async fn insert_from_folder(
    db: &DatabaseConnection,
    folder: &Path,
) -> eyre::Result<Vec<node::Model>> {
    let mut inserted_nodes = vec![];

    for entry in WalkDir::new(folder)
        .into_iter()
        .filter_map(|result| result.ok_log_errors())
        .filter(|e| matches!(e.path().extension_str(), Some("json")))
    {
        let relative_path = entry.path().to_relative_path(folder)?;

        let chat: Chat = serde_json::from_str(&fs::read_to_string(entry.path())?)?;
        for message in chat.messages {
            // Skip non-text messages (may change in the future)
            if message.parsed.text_entities.len() == 0 {
                continue;
            }

            inserted_nodes.push(
                insert_message(db, chat.metadata.clone(), relative_path.clone(), message).await?,
            );
        }
    }

    Ok(inserted_nodes)
}
