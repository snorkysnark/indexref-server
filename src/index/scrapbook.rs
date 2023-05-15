use std::path::{Path, PathBuf};

use chrono::{DateTime, Local};
use eyre::{eyre, ContextCompat};
use fs_err as fs;
use once_cell::sync::Lazy;
use regex::Regex;
use scraper::{Html, Selector};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use url::Url;
use walkdir::WalkDir;
use yaserde_derive::YaDeserialize;

use crate::{
    entity::{
        node,
        types::{NodeType, SourceFolderType},
    },
    ext::ResultExt,
    path_convert::ToRelativePath,
};

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

fn extract_redirect_path(index_html_path: &Path) -> eyre::Result<PathBuf> {
    static SELECT_META: Lazy<Selector> = Lazy::new(|| Selector::parse("meta[content]").unwrap());
    static REGEX_CONTENT: Lazy<Regex> = Lazy::new(|| Regex::new(r"0;URL=(.+)").unwrap());

    let document = Html::parse_document(&fs::read_to_string(index_html_path)?);
    let content = document
        .select(&*SELECT_META)
        .next()
        .context("No meta tag in html")?
        .value()
        .attr("content")
        .context("No content attribute")?;

    let relative_url = REGEX_CONTENT
        .captures_iter(content)
        .next()
        .with_context(|| format!("Unexpected content value: {content}"))?
        .get(1)
        .unwrap()
        .as_str();

    let index_html_url = Url::from_file_path(&index_html_path)
        .map_err(|_| eyre!("Cannot convert path to URL: {}", index_html_path.display()))?;

    let file_url = index_html_url.join(&relative_url)?;
    let file_path = file_url
        .to_file_path()
        .map_err(|_| eyre!("Cannot convert back to file path: {file_url}"))?;

    Ok(file_path)
}

pub async fn insert_from_folder(
    db: &DatabaseConnection,
    folder: &Path,
) -> eyre::Result<Vec<node::Model>> {
    let mut inserted_nodes = vec![];

    for entry in WalkDir::new(folder)
        .into_iter()
        .filter_map(|result| result.ok_log_errors())
        .filter(|e| matches!(e.file_name().to_str(), Some("scrapbook.rdf")))
    {
        let scrapbook_root = entry.path().parent().unwrap();

        let rdf: Rdf = yaserde::de::from_str(&fs::read_to_string(entry.path())?)
            .map_err(|err| eyre!("XML parse error: {err}"))?;

        for description in rdf.descriptions {
            let index_html_path = scrapbook_root
                .join("data")
                .join(&description.id)
                .join("index.html");

            let metadata = fs::metadata(&index_html_path).ok();
            let created = metadata
                // For some reason, meta.created() returns the time the file was appeared in THIS
                // filesystem (and not where it originated)
                // meta.modified() returns the correct date
                .and_then(|meta| meta.modified().ok())
                .map(|time| DateTime::<Local>::from(time).naive_local());

            // For file nodes, the actual file path is found in index.html's redirect
            let file_path = match description.r#type.as_str() {
                "file" => extract_redirect_path(&index_html_path)?,
                _ => index_html_path,
            };

            let rel_path = file_path.to_relative_path(folder)?;

            fn none_if_empty(string: String) -> Option<String> {
                match string.as_str() {
                    "" => None,
                    _ => Some(string),
                }
            }

            let inserted = node::ActiveModel {
                r#type: Set(NodeType::Scrapbook),
                source_folder: Set(Some(SourceFolderType::Scrapbook)),
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
