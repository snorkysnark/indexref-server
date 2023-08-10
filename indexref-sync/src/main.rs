mod env_config;
mod ext;
mod file_sync;
mod macros;
mod sources_config;
mod watcher;

use env_config::EnvConfig;
use eyre::Result;
use migration::{Migrator, MigratorTrait};
use opensearch::OpenSearch;
use sea_orm::Database;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    dotenvy::dotenv().ok();
    let config = EnvConfig::from_env()?;

    tracing_subscriber::fmt()
        // Filter what crates emit logs
        // .with_env_filter(config.env_filter()?)
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let db = Database::connect(config.db()).await?;
    let oss = OpenSearch::default();

    Migrator::up(&db, None).await?;
    watcher::sync_and_watch(config.sources().to_owned(), db, oss).await?;

    Ok(())
}
