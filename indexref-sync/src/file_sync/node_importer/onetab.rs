use std::{fs, path::Path};

use chrono::NaiveDateTime;
use eyre::Result;
use once_cell::sync::Lazy;
use regex::Regex;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::Deserialize;

use entity::{node, types::NodeType};

static DATE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\[\[OneTab\]\] (\d{4}-\d{2}-\d{2}_\d{2}-\d{2}-\d{2})").unwrap());

#[derive(Debug, Deserialize)]
struct OnetabBlock {
    title: String,
    children: Vec<OnetabDate>,
}

#[derive(Debug, Deserialize)]
struct OnetabDate {
    string: String,
    children: Vec<OnetabLink>,
}

#[derive(Debug, Deserialize)]
struct OnetabLink {
    children: (OnetabString, OnetabString),
}

#[derive(Debug, Deserialize)]
struct OnetabString {
    string: String,
}

pub async fn import_from_file(
    db: &DatabaseConnection,
    file_path: &Path,
    file_id: i32,
) -> Result<Vec<node::Model>> {
    let mut inserted_nodes = vec![];

    let blocks: Vec<OnetabBlock> = serde_json::from_str(&fs::read_to_string(file_path)?)?;

    for block in blocks {
        for date_block in block.children {
            let date_str = DATE_REGEX
                .captures_iter(&date_block.string)
                .next()
                .ok_or_else(|| eyre::eyre!("Unexpected date string: {}", date_block.string))?
                .get(1)
                .unwrap()
                .as_str();

            let created = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d_%H-%M-%S")?;

            for link in date_block.children {
                let (url, title) = link.children;

                let inserted = node::ActiveModel {
                    file_id: Set(Some(file_id)),
                    node_type: Set(NodeType::OneTab),
                    title: Set(Some(title.string)),
                    url: Set(Some(url.string)),
                    created: Set(Some(created)),
                    ..Default::default()
                }
                .insert(db)
                .await?;

                inserted_nodes.push(inserted);
            }
        }
    }

    Ok(inserted_nodes)
}
