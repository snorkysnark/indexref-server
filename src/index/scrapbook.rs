use std::{fs, path::Path};

use chrono::{DateTime, Local};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use walkdir::WalkDir;
use yaserde_derive::YaDeserialize;

use crate::{
    ext::ResultExt,
    path_convert::ToRelativePath,
    result::{AppError, AppResult},
};
use entity::{node, types::NodeType};

#[derive(Debug, YaDeserialize)]
#[yaserde(
    prefix = "RDF",
    root = "RDF",
    namespace = "RDF: http://www.w3.org/1999/02/22-rdf-syntax-ns#",
    namespace = "NS1: http://amb.vis.ne.jp/mozilla/scrapbook-rdf#"
)]
struct Rdf {
    #[yaserde(rename = "Description")]
    descriptions: Vec<RdfDescription>,
}

#[derive(Debug, YaDeserialize)]
struct RdfDescription {
    #[yaserde(attribute, prefix = "RDF")]
    about: String,
    #[yaserde(attribute, prefix = "NS1")]
    id: String,
    #[yaserde(attribute, rename = "type", prefix = "NS1")]
    r#type: String,
    #[yaserde(attribute, prefix = "NS1")]
    title: String,
    #[yaserde(attribute, prefix = "NS1")]
    chars: String,
    #[yaserde(attribute, prefix = "NS1")]
    comment: String,
    #[yaserde(attribute, prefix = "NS1")]
    icon: String,
    #[yaserde(attribute, prefix = "NS1")]
    source: String,
}

pub async fn insert_from_folder(
    db: &DatabaseConnection,
    folder: &Path,
) -> AppResult<Vec<node::Model>> {
    let mut inserted_nodes = vec![];

    for entry in WalkDir::new(folder)
        .into_iter()
        .filter_map(|result| result.ok_log_errors())
        .filter(|e| matches!(e.file_name().to_str(), Some("scrapbook.rdf")))
    {
        let scrapbook_root = entry.path().parent().unwrap();

        let rdf: Rdf = yaserde::de::from_str(&fs::read_to_string(entry.path())?)
            .map_err(|err| AppError::XmlError(err))?;

        for description in rdf.descriptions {
            let node_type = match description.r#type.as_str() {
                "" => NodeType::ScrapbookPage,
                "file" => NodeType::ScrapbookFile,
                _ => continue,
            };

            let file_path = scrapbook_root
                .join("data")
                .join(&description.id)
                .join("index.html");

            let metadata = fs::metadata(&file_path).ok();
            let created = metadata
                // For some reason, meta.created() returns the time the file was appeared in THIS
                // filesystem (and not where it originated)
                // meta.modified() returns the correct date
                .and_then(|meta| meta.modified().ok())
                .map(|time| DateTime::<Local>::from(time).naive_local());

            let rel_path = file_path.to_relative_path(folder)?;

            fn none_if_empty(string: String) -> Option<String> {
                match string.as_str() {
                    "" => None,
                    _ => Some(string),
                }
            }

            let inserted = node::ActiveModel {
                r#type: Set(node_type),
                title: Set(none_if_empty(description.title)),
                url: Set(none_if_empty(description.source)),
                file: Set(Some(rel_path.into())),
                created: Set(created),
                original_id: Set(none_if_empty(description.id)),
                ..Default::default()
            }
            .insert(db)
            .await?;

            inserted_nodes.push(inserted);
        }
    }

    Ok(inserted_nodes)
}
