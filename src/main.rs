mod config;
mod entity;
mod ext;
mod index;
mod path_convert;
mod paths;
mod macros;

use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    process::exit,
};

use axum::{routing::get, Router};
use clap::{Parser, Subcommand};
use config::{AppConfig, SourcesConfig};
use paths::ProjectPaths;
use sea_orm::{Database, DatabaseConnection};

use migration::{Migrator, MigratorTrait};

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

async fn run_migrations(db: &DatabaseConnection) {
    use dialoguer::{theme::ColorfulTheme, Confirm};

    if let Err(err) = Migrator::up(db, None).await {
        eprintln!("Migration failed: {err}");
        if matches!(
            Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Delete existing database?")
                .interact(),
            Ok(true)
        ) {
            let _ = Migrator::fresh(db).await;
        } else {
            exit(1);
        }
    }
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let cli = Cli::parse();

    color_eyre::install()?;
    tracing_subscriber::fmt().init();

    let paths = ProjectPaths::init("com", "snorkysnark", "Indexref-Server")?;
    let config = AppConfig::load(paths.config_path())?;

    let db = Database::connect(paths.db_connection_string()?).await?;
    run_migrations(&db).await;

    match cli.command {
        Commands::Index => {
            index::rebuild_index(&db, &config.sources).await?;
        }
        Commands::Serve => {
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
