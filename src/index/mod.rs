use axum::response::{IntoResponse, Response};
use axum::{extract::State, Json};
use hyper::StatusCode;
use migration::{ConnectionTrait, Migrator, MigratorTrait};
use sea_orm::{DatabaseConnection, FromQueryResult, QueryResult, Statement};
use serde::Serialize;

use crate::{config::SourcesConfig, entity::node, AppState};

// pub use self::node_data::{get_node_full, get_node_full_handler};
use self::node_presentation::NodePresentationWithRelations;
pub use self::serve_file::*;

// mod node_data;
mod node_presentation;
mod scrapbook;
mod serve_file;
mod single_file_z;
mod telegram;

#[derive(Debug, Serialize)]
pub struct NodeWithChildren {
    #[serde(flatten)]
    node: node::Model,
    children: Option<String>,
}

impl FromQueryResult for NodeWithChildren {
    fn from_query_result(res: &QueryResult, pre: &str) -> Result<Self, migration::DbErr> {
        Ok(NodeWithChildren {
            node: node::Model::from_query_result(res, pre)?,
            children: res.try_get(pre, "children")?,
        })
    }
}

pub async fn get_nodes(
    db: &DatabaseConnection,
    sources: &SourcesConfig,
) -> eyre::Result<Vec<NodeWithChildren>> {
    let select = Statement::from_string(
        sea_orm::DatabaseBackend::Sqlite,
        "select node.*, group_concat(nc.id) as children
            from node
            left join node_closure as nc
            on node.id = nc.root and nc.depth = 1
            group by node.id;"
            .to_owned(),
    );

    let nodes = NodeWithChildren::find_by_statement(select).all(db).await?;
    Ok(nodes)
}

pub async fn get_nodes_handler(state: State<AppState>) -> Response {
    match get_nodes(&state.db, &state.sources).await {
        Ok(value) => Json(value).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

pub async fn rebuild_index(
    db: &DatabaseConnection,
    sources: &SourcesConfig,
) -> eyre::Result<Vec<node::Model>> {
    // Clear existing index
    Migrator::fresh(db).await?;

    let mut inserted_nodes = vec![];

    if let Some(telegram_chat) = sources.telegram_chat() {
        inserted_nodes.append(&mut self::telegram::insert_from_folder(db, telegram_chat).await?);
    }
    if let Some(single_file_z) = sources.single_file_z() {
        inserted_nodes
            .append(&mut self::single_file_z::insert_from_folder(db, single_file_z).await?);
    }
    if let Some(scrapbook) = sources.scrapbook() {
        inserted_nodes.append(&mut self::scrapbook::insert_from_folder(db, scrapbook).await?);
    }

    Ok(inserted_nodes)
}
