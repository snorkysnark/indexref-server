mod ext;

use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};

use entity::{node, types::RelativePathSql};
use eyre::ContextCompat;
use futures::{StreamExt, TryStreamExt};
use migration::{Migrator, MigratorTrait};
use sea_orm::{ColumnTrait, Database, EntityTrait, FromQueryResult, QueryFilter, QuerySelect};
use walkdir::WalkDir;

use ext::{ResultExt, ToRelativePath};

// telegram
// single_file_z
// scrapbook
// onetab
// zotero

#[derive(Debug, Eq, PartialEq, Hash, FromQueryResult)]
struct FileSummary {
    file: RelativePathSql,
    hash: String,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL")?;
    let basepath: PathBuf = std::env::var("INDEXREF_PATH")?.into();
    let telegram_path = basepath.join("telegram");

    let db = Database::connect(&db_url).await?;
    Migrator::up(&db, None).await?;

    let indexed_files: HashSet<FileSummary> = node::Entity::find()
        .distinct()
        .filter(
            node::Column::File
                .is_not_null()
                .and(node::Column::Hash.is_not_null()),
        )
        .into_model::<FileSummary>()
        .stream(&db)
        .await?
        .try_collect()
        .await?;

    let actual_files: HashSet<FileSummary> = WalkDir::new(basepath.join("telegram"))
        .into_iter()
        .filter_map(|result| result.ok_log_errors())
        .filter(|entry| matches!(entry.file_name().to_str(), Some("result.json")))
        .map(|entry| {
            (|| -> eyre::Result<_> {
                Ok(FileSummary {
                    file: entry.path().to_relative_path(&telegram_path)?.into(),
                    hash: sha256::try_digest(entry.path())?,
                })
            })()
        })
        .collect::<Result<_, _>>()?;

    let to_delete = indexed_files.difference(&actual_files);
    let to_add = actual_files.difference(&indexed_files);

    Ok(())
}
