mod result;

use std::{
    fs,
    net::{SocketAddr, SocketAddrV4},
};

use axum::{extract::State, routing::get, Json, Router};
use directories_next::ProjectDirs;
use sea_orm::{Database, DatabaseConnection, EntityTrait};

use entity::prelude::*;
use migration::{Migrator, MigratorTrait};
use result::{path::TryToStr, AppError, AppResult};

type NodeModel = <Node as EntityTrait>::Model;

async fn get_nodes(db: State<DatabaseConnection>) -> AppResult<Json<Vec<NodeModel>>> {
    Ok(Json(Node::find().all(&*db).await?))
}

#[tokio::main]
async fn main() -> AppResult<()> {
    tracing_subscriber::fmt().init();

    let dirs = ProjectDirs::from("com", "snorkysnark", "Indexref-Server")
        .ok_or(AppError::ProjectDirsNotFound)?;

    let data_dir = dirs.data_dir();
    fs::create_dir_all(data_dir)?;

    let db = Database::connect(format!(
        "sqlite://{}?mode=rwc",
        data_dir.join("index.db").try_to_str()?,
    ))
    .await?;
    Migrator::up(&db, None).await?;

    let app = Router::new().route("/nodes", get(get_nodes)).with_state(db);

    axum::Server::bind(&SocketAddr::V4(SocketAddrV4::new(
        "127.0.0.1".parse().unwrap(),
        3000,
    )))
    .serve(app.into_make_service())
    .await
    .unwrap();

    Ok(())
}
