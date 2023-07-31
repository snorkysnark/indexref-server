mod config;
mod diff_files;
mod ext;
mod file_finder;

use std::path::PathBuf;

use config::SourcesConfig;
use diff_files::FileDiff;
use eyre::Result;
use migration::{Migrator, MigratorTrait};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, Database, DatabaseConnection, EntityTrait, TransactionTrait,
};

use crate::diff_files::FileSummary;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL")?;
    let config_path: PathBuf = std::env::var("INDEXREF_CONFIG")
        .unwrap_or_else(|_| "config.json".to_owned())
        .into();
    let sources: SourcesConfig = serde_json::from_str(&std::fs::read_to_string(&config_path)?)?;

    let db = Database::connect(&db_url).await?;
    Migrator::up(&db, None).await?;

    let diff = diff_files::diff_files_with_db(&db, &sources).await?;
    println!("{diff:#?}");

    apply_diff(&db, diff).await?;

    Ok(())
}

async fn apply_diff(db: &DatabaseConnection, diff: FileDiff) -> Result<()> {
    use entity::file;

    async fn delete_files(db: &DatabaseConnection, ids: &[i32]) -> Result<()> {
        let txn = db.begin().await?;
        for id in ids {
            file::Entity::delete_by_id(*id).exec(db).await?;
        }
        txn.commit().await?;

        Ok(())
    }
    async fn add_files(db: &DatabaseConnection, file_summaries: Vec<FileSummary>) -> Result<()> {
        for summary in file_summaries.into_iter() {
            let txn = db.begin().await?;

            let insered_file = file::ActiveModel {
                source_type: Set(summary.file_type),
                path: Set(summary.path.try_into()?),
                hash: Set(summary.hash),
                ..Default::default()
            }
            .insert(db)
            .await?;

            txn.commit().await?;
        }

        Ok(())
    }

    delete_files(&db, &diff.to_delete).await?;

    Ok(())
}
