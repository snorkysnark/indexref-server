use std::path::Path;

use chrono::NaiveDateTime;
use once_cell::sync::Lazy;
use regex::Regex;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::Deserialize;
use walkdir::WalkDir;

use crate::{
    entity::{node, types::NodeType},
    ext::{PathExt, ResultExt},
    path_convert::ToRelativePath,
};

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

        let blocks: Vec<OnetabBlock> =
            serde_json::from_str(&fs_err::read_to_string(entry.path())?)?;

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
                        r#type: Set(NodeType::OneTab),
                        title: Set(Some(title.string)),
                        url: Set(Some(url.string)),
                        file: Set(Some(relative_path.clone().into())),
                        created: Set(Some(created)),
                        ..Default::default()
                    }
                    .insert(db)
                    .await?;

                    inserted_nodes.push(inserted);
                }
            }
        }
    }

    Ok(inserted_nodes)
}
