use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use eyre::Result;
use futures::{future, TryStreamExt};
use sea_orm::{DatabaseConnection, EntityTrait};
use tryvial::try_block;

use crate::{
    config::SourcesConfig,
    file_finder::{self, FoundFile},
};
use entity::{file, types::FileType};

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct FileSummary {
    pub file_type: FileType,
    pub path: PathBuf,
    pub hash: String,
}

async fn get_indexed_files(db: &DatabaseConnection) -> Result<HashMap<FileSummary, i32>> {
    let mut indexed_files = HashMap::new();

    file::Entity::find()
        .stream(db)
        .await?
        .try_for_each(|file_entry: file::Model| {
            indexed_files.insert(
                FileSummary {
                    file_type: file_entry.source_type,
                    path: file_entry.path.into(),
                    hash: file_entry.hash,
                },
                file_entry.id,
            );
            future::ready(Ok(()))
        })
        .await?;

    Ok(indexed_files)
}

fn find_actual_files(sources: &SourcesConfig) -> Result<HashSet<FileSummary>> {
    let result: Result<_> = file_finder::find_all(sources)
        .map(|FoundFile { file_type, path }| {
            try_block! {
                FileSummary {
                    hash: sha256::try_digest(&path)?,
                    file_type,
                    path,
                }
            }
        })
        .collect();

    Ok(result?)
}

#[derive(Debug)]
pub struct FileDiff {
    pub to_delete: Vec<i32>,
    pub to_add: Vec<FileSummary>,
}

pub async fn diff_files_with_db(
    db: &DatabaseConnection,
    sources: &SourcesConfig,
) -> Result<FileDiff> {
    let indexed_files = get_indexed_files(db).await?;
    let actual_files = find_actual_files(sources)?;

    let mut to_delete = Vec::new();
    let mut to_add = Vec::new();

    for (summary, id) in indexed_files.iter() {
        if !actual_files.contains(summary) {
            to_delete.push(*id);
        }
    }
    for summary in actual_files.into_iter() {
        if !indexed_files.contains_key(&summary) {
            to_add.push(summary);
        }
    }

    Ok(FileDiff { to_delete, to_add })
}
