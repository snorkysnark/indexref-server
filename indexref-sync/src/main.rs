mod env_config;
mod ext;
mod file_sync;
mod macros;
mod sources_config;

use env_config::EnvConfig;
use migration::{Migrator, MigratorTrait};
use opensearch::OpenSearch;
use sea_orm::Database;
use sources_config::SourcesConfig;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    dotenvy::dotenv().ok();
    let config = EnvConfig::from_env()?;

    tracing_subscriber::fmt()
        // Filter what crates emit logs
        .with_env_filter(config.env_filter()?)
        .init();

    let sources: SourcesConfig = serde_json::from_str(&std::fs::read_to_string(config.sources())?)?;

    let db = Database::connect(config.db()).await?;
    Migrator::up(&db, None).await?;

    let oss = OpenSearch::default();
    file_sync::sync_db_with_sources(&db, &oss, &sources).await?;

    Ok(())
}
