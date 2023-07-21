mod ext;

use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use entity::file;
use futures::{future, TryStreamExt};
use migration::{Migrator, MigratorTrait};
use relative_path::RelativePathBuf;
use sea_orm::{Database, EntityTrait};
use walkdir::WalkDir;

use ext::{ResultExt, ToRelativePath};

// telegram
// single_file_z
// scrapbook
// onetab
// zotero

#[derive(Debug, Eq, PartialEq, Hash)]
struct FileSummary {
    path: RelativePathBuf,
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

    let indexed_files = {
        let mut indexed_files: HashMap<FileSummary, file::Model> = HashMap::new();

        file::Entity::find()
            .stream(&db)
            .await?
            .try_for_each(|file_entry| {
                indexed_files.insert(
                    FileSummary {
                        path: file_entry.path.clone().into(),
                        hash: file_entry.hash.clone(),
                    },
                    file_entry,
                );
                future::ready(Ok(()))
            })
            .await?;

        indexed_files
    };

    let actual_files: HashSet<FileSummary> = WalkDir::new(basepath.join("telegram"))
        .into_iter()
        .filter_map(|result| result.ok_log_errors())
        .filter(|entry| matches!(entry.file_name().to_str(), Some("result.json")))
        .map(|entry| {
            (|| -> eyre::Result<_> {
                Ok(FileSummary {
                    path: entry.path().to_relative_path(&telegram_path)?,
                    hash: sha256::try_digest(entry.path())?,
                })
            })()
        })
        .collect::<Result<_, _>>()?;

    for (indexed_summary, indexed_file) in indexed_files.iter() {
        if !actual_files.contains(indexed_summary) {
            println!("Delete {indexed_file:?}");
        }
    }

    for actual_summary in actual_files.into_iter() {
        if !indexed_files.contains_key(&actual_summary) {
            println!("Add {actual_summary:?}");
        }
    }

    Ok(())
}
