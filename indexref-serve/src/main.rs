mod endpoints;
mod err;

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use axum::{routing::get, Router};
use sea_orm::{Database, DatabaseConnection};

use migration::{Migrator, MigratorTrait};
use tower_http::cors::{self, CorsLayer};
use tracing::info;
use tracing_subscriber::EnvFilter;

const LOCALHOST: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);

#[derive(Debug, Clone)]
pub struct AppState {
    db: DatabaseConnection,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenvy::dotenv().ok();

    color_eyre::install()?;
    tracing_subscriber::fmt()
        // Filter what crates emit logs
        .with_env_filter(EnvFilter::try_new("indexref_serve,sea_orm")?)
        .init();

    let db = Database::connect(std::env::var("DATABASE_URL")?).await?;
    let port: u16 = std::env::var("INDEXREF_PORT")?.parse()?;

    Migrator::up(&db, None).await?;

    #[allow(unused_mut)]
    let mut app = Router::new()
        .route("/nodes", get(endpoints::get_node_tree_handler))
        .route("/node/:id", get(endpoints::get_node_full_handler))
        .with_state(AppState { db })
        .layer(CorsLayer::new().allow_origin(cors::Any));

    #[cfg(feature = "static_server")]
    {
        use axum::routing::get_service;
        use tower_http::services::{ServeDir, ServeFile};

        app = app
            .nest_service("/static", ServeDir::new("static"))
            .route("/", get_service(ServeFile::new("static/index.html")));
    }

    let socket_addr = SocketAddrV4::new(LOCALHOST, port);
    info!("Serving on http://{socket_addr}");
    axum::Server::bind(&SocketAddr::V4(socket_addr))
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
