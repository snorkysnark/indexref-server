mod config;
mod endpoints;
mod err;

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use axum::{routing::get, Router};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use tracing::info;

use config::Config;

#[derive(Debug, Clone)]
pub struct AppState {
    db: DatabaseConnection,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    dotenvy::dotenv().ok();
    let config = Config::from_env()?;

    tracing_subscriber::fmt()
        .with_env_filter(config.env_filter()?)
        .init();

    let db = Database::connect(config.db()).await?;

    Migrator::up(&db, None).await?;

    let app = Router::new()
        .route("/nodes", get(endpoints::get_node_tree_handler))
        .route("/node/:id", get(endpoints::get_node_full_handler))
        .with_state(AppState { db });

    let socket_addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), config.port());
    info!("Serving on http://{socket_addr}");
    axum::Server::bind(&SocketAddr::V4(socket_addr))
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
