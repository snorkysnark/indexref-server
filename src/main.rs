mod ext;
mod index;
mod paths;
mod result;

use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    path::Path,
};

use axum::{extract::State, routing::get, Json, Router};
use clap::{Parser, Subcommand};
use paths::ProjectPaths;
use sea_orm::{Database, DatabaseConnection, EntityTrait};

use entity::node;
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

async fn get_nodes(db: State<DatabaseConnection>) -> AppResult<Json<Vec<node::Model>>> {
    Ok(Json(node::Entity::find().all(&*db).await?))
}

const LOCALHOST: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);

#[tokio::main]
async fn main() -> AppResult<()> {
    let cli = Cli::parse();

    tracing_subscriber::fmt().init();
    let paths = ProjectPaths::init("com", "snorkysnark", "Indexref-Server")?;
    let db = Database::connect(paths.db_connection_string()?).await?;
    Migrator::up(&db, None).await?;

    match cli.command {
        Commands::Index => {
            index::rebuild_index(
                &db,
                &Path::new("/home/lisk/Work/indexref/data/примеры данных/ChatExport"),
            )
            .await?;
        }
        Commands::Serve => {
            let app = Router::new().route("/nodes", get(get_nodes)).with_state(db);

            axum::Server::bind(&SocketAddr::V4(SocketAddrV4::new(LOCALHOST, 3000)))
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
    }

    Ok(())
}
