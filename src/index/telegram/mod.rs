mod raw;

use std::{fs, path::Path};

use chrono::NaiveDateTime;
use relative_path::RelativePathBuf;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use walkdir::WalkDir;

use self::raw::{Chat, ChatMetadata, Message};
use crate::{
    entity::{
        node,
        types::{NodeData, NodeType, TelegramData},
    },
    ext::{PathExt, ResultExt},
    path_convert::ToRelativePath,
};

async fn insert_message(
    db: &DatabaseConnection,
    metadata: ChatMetadata,
    relative_path: RelativePathBuf,
    message: Message,
) -> eyre::Result<node::Model> {
    let title = if message.text_entities.len() > 0 {
        Some(
            message
                .text_entities
                .iter()
                .map(|entity| entity.text.as_str())
                .collect(),
        )
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
    let message_id = message.id.to_string();

    let inserted_node = node::ActiveModel {
        r#type: Set(NodeType::Telegram),
        subtype: Set(Some(message.r#type.clone())),
        title: Set(title),
        url: Set(url),
        created: Set(Some(created)),
        file: Set(Some(relative_path.into())),
        original_id: Set(Some(message_id)),
        data: Set(Some(NodeData::Telegram(TelegramData {
            chat_name: metadata.name,
            chat_type: metadata.r#type,
            chat_id: metadata.id,
            text_entities: message.text_entities.into_iter().map(Into::into).collect(),
            photo: message.photo,
            file: message.file,
            other: message.other,
        }))),
        ..Default::default()
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
            // Service messaged don't seem to have any useful data
            if message.r#type == "service" {
                continue;
            }

            inserted_nodes.push(
                insert_message(db, chat.metadata.clone(), relative_path.clone(), message).await?,
            );
        }
    }

    Ok(inserted_nodes)
}
