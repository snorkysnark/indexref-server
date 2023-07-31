mod config;
mod ext;
mod file_sync;
mod macros;

use std::path::PathBuf;

use config::SourcesConfig;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

async fn connect_db(url: String) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(url);
    opt.sqlx_logging_level(log::LevelFilter::Trace); // Too verbose

    Database::connect(opt).await
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL")?;
    let config_path: PathBuf = std::env::var("INDEXREF_CONFIG")
        .unwrap_or_else(|_| "config.json".to_owned())
        .into();
    let sources: SourcesConfig = serde_json::from_str(&std::fs::read_to_string(&config_path)?)?;

    let db = connect_db(db_url).await?;
    Migrator::up(&db, None).await?;

    file_sync::sync_db_with_sources(&db, &sources).await?;

    Ok(())
}
