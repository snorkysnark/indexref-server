mod file_finder;
mod node_importer;

use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use entity::{file, types::FileType};
use eyre::Result;
use futures::{future, TryStreamExt};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait};
use tryvial::try_block;

use crate::{config::SourcesConfig, macros::transaction};

use self::file_finder::FoundFile;

#[derive(Debug, Eq, PartialEq, Hash)]
struct FileSummary {
    file_type: FileType,
    path: PathBuf,
    hash: String,
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
                    path: file_entry.path.0.into_std_path_buf(),
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

pub async fn sync_db_with_sources(db: &DatabaseConnection, sources: &SourcesConfig) -> Result<()> {
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

    // Remove deleted files from database
    transaction!(db => {
        for id in to_delete {
            file::Entity::delete_by_id(id).exec(db).await?;
        }
    });

    // TODO: make this concurrent with tokio::spawn
    for summary in to_add.into_iter() {
        transaction!(db => {
            let insered_file = file::ActiveModel {
                source_type: Set(summary.file_type),
                path: Set(summary.path.clone().try_into()?),
                hash: Set(summary.hash),
                ..Default::default()
            }
            .insert(db)
            .await?;

            node_importer::import_from_file(db, summary.file_type, &summary.path, insered_file.id).await?;
        });
    }

    Ok(())
}
