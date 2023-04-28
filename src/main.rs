mod config;
mod ext;
mod index;
mod path_convert;
mod paths;
mod result;

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use axum::{routing::get, Router};
use clap::{Parser, Subcommand};
use config::{AppConfig, SourcesConfig};
use paths::ProjectPaths;
use sea_orm::{Database, DatabaseConnection};

use migration::{Migrator, MigratorTrait};
use result::AppResult;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Index,
    Serve,
}

const LOCALHOST: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);

#[derive(Debug, Clone)]
pub struct AppState {
    db: DatabaseConnection,
    sources: SourcesConfig,
}

#[tokio::main]
async fn main() -> AppResult<()> {
    let cli = Cli::parse();

    tracing_subscriber::fmt().init();

    let paths = ProjectPaths::init("com", "snorkysnark", "Indexref-Server")?;
    let config = AppConfig::load(paths.config_path())?;

    let db = Database::connect(paths.db_connection_string()?).await?;
    Migrator::up(&db, None).await?;

    match cli.command {
        Commands::Index => {
            index::rebuild_index(&db, &config.sources).await?;
        }
        Commands::Serve => {
            let app = Router::new()
                .route("/nodes", get(index::get_nodes_handler))
                .with_state(AppState {
                    db,
                    sources: config.sources,
                });

            axum::Server::bind(&SocketAddr::V4(SocketAddrV4::new(
                LOCALHOST,
                config.server.port(),
            )))
            .serve(app.into_make_service())
            .await?;
        }
    }

    Ok(())
}
