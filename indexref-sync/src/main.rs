mod config;
mod ext;
mod file_sync;
mod macros;

use std::path::PathBuf;

use config::SourcesConfig;
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt()
        // Filter what crates emit logs
        .with_env_filter(EnvFilter::try_new("indexref_sync,sea_orm")?)
        .init();

    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL")?;
    let config_path: PathBuf = std::env::var("INDEXREF_CONFIG")
        .unwrap_or_else(|_| "config.json".to_owned())
        .into();
    let sources: SourcesConfig = serde_json::from_str(&std::fs::read_to_string(&config_path)?)?;

    let db = Database::connect(&db_url).await?;
    Migrator::up(&db, None).await?;

    file_sync::sync_db_with_sources(&db, &sources).await?;

    Ok(())
}
