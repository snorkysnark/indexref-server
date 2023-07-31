mod def;

use std::{fs, path::Path};

use chrono::NaiveDateTime;
use eyre::Result;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

use self::def::{Chat, ChatMetadata, Message, MessageExport};
use entity::{node, types::NodeType};

async fn insert_message(
    db: &DatabaseConnection,
    file_id: i32,
    metadata: ChatMetadata,
    message: Message,
) -> Result<node::Model> {
    let full_text: String = message
        .text_entities
        .iter()
        .map(|entity| entity.text.as_str())
        .collect();

    let title = if full_text.len() > 0 {
        Some(full_text.clone())
    } else {
        message.file.clone().or(message.photo.clone())
    };

    let url = message.text_entities.iter().find_map(|block| {
        if block.r#type == "link" {
            Some(block.text.clone())
        } else {
            block.href.clone()
        }
    });
    let created = NaiveDateTime::parse_from_str(&message.date, "%Y-%m-%dT%H:%M:%S")?;
    let edited = message
        .edited
        .as_deref()
        .map(|edited| NaiveDateTime::parse_from_str(edited, "%Y-%m-%dT%H:%M:%S"))
        .transpose()?
        .or(Some(created.clone()));

    let message_id = message.id.to_string();

    let inserted_node = node::ActiveModel {
        file_id: Set(Some(file_id)),
        node_type: Set(NodeType::Telegram),
        title: Set(title),
        subtype: Set(Some(message.r#type.clone())),
        url: Set(url),
        created: Set(Some(created)),
        modified: Set(edited),
        original_id: Set(Some(message_id)),
        data: Set(Some(serde_json::to_value(MessageExport {
            chat: metadata,
            full_text,
            message,
        })?)),
        ..Default::default()
    }
    .insert(db)
    .await?;

    Ok(inserted_node)
}

pub async fn import_from_file(
    db: &DatabaseConnection,
    file_path: &Path,
    file_id: i32,
) -> Result<Vec<node::Model>> {
    let mut inserted_nodes = vec![];

    let chat: Chat = serde_json::from_str(&fs::read_to_string(file_path)?)?;
    for message in chat.messages {
        // Service messaged don't seem to have any useful data
        if message.r#type == "service" {
            continue;
        }

        inserted_nodes.push(insert_message(db, file_id, chat.metadata.clone(), message).await?);
    }

    Ok(inserted_nodes)
}
