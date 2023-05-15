mod config;
mod date_serializer;
mod entity;
mod ext;
mod index;
mod macros;
mod path_convert;
mod paths;

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use axum::{routing::get, Router};
use clap::{Parser, Subcommand};
use color_eyre::Help;
use config::{AppConfig, SourcesConfig};
use paths::ProjectPaths;
use sea_orm::{Database, DatabaseConnection};

use migration::{ConnectionTrait, Migrator, MigratorTrait};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Rebuild the index
    Index,
    /// Run local server
    Serve,
}

const LOCALHOST: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);

#[derive(Debug, Clone)]
pub struct AppState {
    db: DatabaseConnection,
    sources: SourcesConfig,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let cli = Cli::parse();

    color_eyre::install()?;
    tracing_subscriber::fmt().init();

    let paths = ProjectPaths::init("com", "snorkysnark", "Indexref-Server")?;
    let config = AppConfig::load(paths.config_path())?;

    libsqlite3_extensions::init();
    let db = Database::connect(paths.db_connection_string()?).await?;

    match cli.command {
        Commands::Index => {
            index::rebuild_index(&db, &config.sources).await?;
        }
        Commands::Serve => {
            Migrator::up(&db, None)
                .await
                .suggestion("Try rebuilding the index from scratch")?;

            db.execute_unprepared(
                r#"CREATE VIRTUAL TABLE IF NOT EXISTS node_closure USING transitive_closure (
                tablename="node",
                idcolumn="id",
                parentcolumn="parent_id");"#,
            )
            .await?;

            let app = Router::new()
                .route("/nodes", get(index::get_nodes_handler))
                .route("/node/:id", get(index::get_node_full_handler))
                .route("/files/:node_type/*path", get(index::serve_file_handler))
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
