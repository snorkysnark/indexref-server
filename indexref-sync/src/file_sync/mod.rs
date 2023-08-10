mod file_finder;
mod node_importer;
mod opensearch;

use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use ::opensearch::OpenSearch;
use camino::FromPathBufError;
use entity::{file, types::FileType, NodePresentaion};
use eyre::Result;
use futures::{future, TryStreamExt};
use log::info;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, DatabaseConnection, DbErr, EntityTrait, TransactionTrait,
};
use serde_json::json;
use thiserror::Error;
use tryvial::try_block;

use crate::sources_config::SourcesConfig;

use self::{
    file_finder::FoundFile,
    opensearch::{BulkApiError, DeleteByQueryError},
};

#[derive(Debug, Eq, PartialEq, Hash)]
struct FileSummary {
    file_type: FileType,
    path: PathBuf,
    hash: String,
}

#[derive(Debug, Error)]
enum SyncError {
    #[error("db: {0}")]
    Db(#[from] DbErr),
    #[error("delete_by_query: {0}")]
    DeleteByQuery(#[from] DeleteByQueryError),
    #[error("bulk_api: {0}")]
    Bulk(#[from] BulkApiError),
    #[error("path convert: {0}")]
    NonUtf8Path(#[from] FromPathBufError),
    #[error("{0}")]
    Other(#[from] eyre::Report),
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

pub async fn sync_db_with_sources(
    db: &DatabaseConnection,
    oss: &OpenSearch,
    sources: &SourcesConfig,
) -> Result<()> {
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

    info!("Files to delete: {to_delete:?}");
    // Remove deleted files from database
    db.transaction::<_, _, SyncError>(|txn| {
        let oss = oss.clone();
        Box::pin(async move {
            for id in to_delete.iter() {
                file::Entity::delete_by_id(*id).exec(txn).await?;
            }
            for id in to_delete {
                opensearch::delete_by_query(&oss, "nodes", json!({ "match": { "file_id": id } }))
                    .await?
            }

            Ok(())
        })
    })
    .await?;

    info!("Files to add: {to_add:#?}");
    // Make this parallel with tokio::spawn?
    // Or not? Parallel version seems to be slower
    for summary in to_add.into_iter() {
        db.transaction::<_, _, SyncError>(|txn| {
            let oss = oss.clone();
            Box::pin(async move {
                let insered_file = file::ActiveModel {
                    source_type: Set(summary.file_type),
                    path: Set(summary.path.clone().try_into()?),
                    hash: Set(summary.hash),
                    ..Default::default()
                }
                .insert(txn)
                .await?;

                let to_index: Vec<NodePresentaion> = node_importer::import_from_file(
                    txn,
                    summary.file_type,
                    &summary.path,
                    insered_file.id,
                )
                .await?
                .into_iter()
                .map(|node| (node, Some(insered_file.clone())).into())
                .collect();

                self::opensearch::add_documents(&oss, to_index).await?;
                Ok(())
            })
        })
        .await?;
    }

    Ok(())
}
