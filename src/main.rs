mod config;
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
use opensearch::OpenSearch;
use paths::ProjectPaths;
use sea_orm::{Database, DatabaseConnection, EntityTrait};

use migration::{Migrator, MigratorTrait};
use tower_http::cors::{self, CorsLayer};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Rebuild the index
    Index,
    /// Upload to opensearch
    Opensearch,
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
    dotenvy::dotenv().ok();

    color_eyre::install()?;
    tracing_subscriber::fmt().init();

    let paths = ProjectPaths::init("indexref-server")?;
    let config = AppConfig::load(paths.config_path())?;

    let db = Database::connect(std::env::var("DATABASE_URL")?).await?;

    match cli.command {
        Commands::Index => {
            index::rebuild_index(&db, &config.sources).await?;
        }
        Commands::Opensearch => {
            index::upload_to_opensearch(
                OpenSearch::default(),
                entity::node::Entity::find().all(&db).await?,
            )
            .await?
        }
        Commands::Serve => {
            Migrator::up(&db, None)
                .await
                .suggestion("Try rebuilding the index from scratch")?;

            #[allow(unused_mut)]
            let mut app = Router::new()
                .route("/nodes", get(index::get_node_tree_handler))
                .route("/node/:id", get(index::get_node_full_handler))
                .route("/files/:node_type/*path", get(index::serve_file_handler))
                .with_state(AppState {
                    db,
                    sources: config.sources,
                })
                .layer(CorsLayer::new().allow_origin(cors::Any));

            #[cfg(feature = "static_server")]
            {
                use axum::routing::get_service;
                use tower_http::services::{ServeDir, ServeFile};

                app = app
                    .nest_service("/static", ServeDir::new("static"))
                    .route("/", get_service(ServeFile::new("static/index.html")));
            }

            let socket_addr = SocketAddrV4::new(LOCALHOST, std::env::var("PORT")?.parse()?);
            println!("Serving on http://{socket_addr}");
            axum::Server::bind(&SocketAddr::V4(socket_addr))
                .serve(app.into_make_service())
                .await?;
        }
    }

    Ok(())
}
