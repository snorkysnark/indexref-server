mod icon;

use std::{
    collections::HashMap,
    format,
    path::{Path, PathBuf},
};

use chrono::{DateTime, Local};
use eyre::{eyre, ContextCompat};
use fs_err as fs;
use once_cell::sync::Lazy;
use regex::Regex;
use relative_path::RelativePathBuf;
use scraper::{Html, Selector};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use url::Url;
use walkdir::WalkDir;
use yaserde_derive::YaDeserialize;

use crate::{
    entity::{node, scrapbook, types::NodeType},
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
    #[yaserde(rename = "Seq")]
    sequences: Vec<RdfSeq>,
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

#[derive(Debug, YaDeserialize)]
struct RdfSeq {
    #[yaserde(attribute, prefix = "RDF")]
    about: String,
    #[yaserde(rename = "li")]
    items: Vec<RdfLi>,
}

#[derive(Debug, YaDeserialize)]
struct RdfLi {
    #[yaserde(attribute, prefix = "RDF")]
    resource: String,
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

async fn insert_one(
    db: &DatabaseConnection,
    location: &ScrapbookLocation<'_, '_>,
    description: RdfDescription,
) -> eyre::Result<(node::Model, scrapbook::Model)> {
    let index_html = {
        let index_html = location
            .scrapbook_dir
            .join("data")
            .join(&description.id)
            .join("index.html");

        if index_html.exists() {
            Some(index_html)
        } else {
            None
        }
    };

    let metadata = index_html
        .as_ref()
        .and_then(|path| fs::metadata(&path).ok());
    let created = metadata
        // For some reason, meta.created() returns the time the file was appeared in THIS
        // filesystem (and not where it originated)
        // meta.modified() returns the correct date
        .and_then(|meta| meta.modified().ok())
        .map(|time| DateTime::<Local>::from(time).naive_local());

    // For file nodes, the actual file path is found in index.html's redirect
    let file_path = match description.r#type.as_str() {
        "file" => index_html
            .as_ref()
            .map(|index_html| extract_redirect_path(index_html))
            .transpose()?,
        _ => index_html,
    };

    let rel_path = file_path
        .map(|path| path.to_relative_path(location.root))
        .transpose()?;

    fn none_if_empty(string: String) -> Option<String> {
        match string.as_str() {
            "" => None,
            _ => Some(string),
        }
    }

    let old_icon = none_if_empty(description.icon);
    let remapped_icon = old_icon
        .as_ref()
        .and_then(|icon| icon::remap_icon(icon, &location.scrapbook_dir_rel));

    let inserted_node = node::ActiveModel {
        r#type: Set(NodeType::Scrapbook),
        subtype: Set(none_if_empty(description.r#type.clone())),
        title: Set(none_if_empty(description.title.clone())),
        url: Set(none_if_empty(description.source.clone())),
        icon: Set(remapped_icon),
        file: Set(rel_path.map(Into::into)),
        created: Set(created),
        original_id: Set(none_if_empty(description.id.clone())),
        ..Default::default()
    }
    .insert(db)
    .await?;

    let inserted_data = scrapbook::ActiveModel {
        node_id: Set(inserted_node.id),
        about: Set(none_if_empty(description.about)),
        old_id: Set(none_if_empty(description.id)),
        r#type: Set(none_if_empty(description.r#type)),
        title: Set(none_if_empty(description.title)),
        chars: Set(none_if_empty(description.chars)),
        comment: Set(none_if_empty(description.comment)),
        icon: Set(old_icon),
        source: Set(none_if_empty(description.source)),
    }
    .insert(db)
    .await?;

    Ok((inserted_node, inserted_data))
}

#[derive(Debug)]
struct ScrapbookLocation<'a, 'b> {
    pub root: &'a Path,
    pub scrapbook_dir: &'b Path,
    pub scrapbook_dir_rel: RelativePathBuf,
}

impl<'a, 'b> ScrapbookLocation<'a, 'b> {
    fn new(root: &'a Path, scrapbook_dir: &'b Path) -> eyre::Result<Self> {
        let scrapbook_dir_rel = scrapbook_dir.to_relative_path(root)?;

        Ok(Self {
            root,
            scrapbook_dir,
            scrapbook_dir_rel,
        })
    }
}

pub async fn insert_from_folder(
    db: &DatabaseConnection,
    root: &Path,
) -> eyre::Result<Vec<node::Model>> {
    let mut inserted_nodes = vec![];

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(|result| result.ok_log_errors())
        .filter(|e| matches!(e.file_name().to_str(), Some("scrapbook.rdf")))
    {
        let location = ScrapbookLocation::new(root, entry.path().parent().unwrap())?;

        let rdf: Rdf = yaserde::de::from_str(&fs::read_to_string(entry.path())?)
            .map_err(|err| eyre!("XML parse error: {err}"))?;

        let mut about_to_id: HashMap<String, i32> = HashMap::new();

        for description in rdf.descriptions {
            let (inserted_node, node_data) = insert_one(db, &location, description).await?;

            if let Some(about) = node_data.about {
                about_to_id.insert(about, inserted_node.id);
            }
            inserted_nodes.push(inserted_node);
        }

        for sequence in rdf.sequences {
            if sequence.about == "urn:scrapbook:root" {
                // For now, mapping the scrapbook root to tree root
                continue;
            }

            let parent_id = about_to_id.get(&sequence.about).context(sequence.about)?;
            let child_ids: Result<Vec<_>, _> = sequence
                .items
                .into_iter()
                .map(|li| about_to_id.get(&li.resource).context(li.resource))
                .collect();

            for child_id in child_ids? {
                node::ActiveModel {
                    id: Set(*child_id),
                    parent_id: Set(Some(*parent_id)),
                    ..Default::default()
                }
                .update(db)
                .await?;
            }
        }
    }

    Ok(inserted_nodes)
}
