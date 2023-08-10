mod raw;

use std::{
    collections::HashMap,
    format, fs,
    ops::Deref,
    path::{Path, PathBuf},
};

use chrono::{DateTime, Local};
use eyre::{eyre, ContextCompat, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use scraper::{Html, Selector};
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};
use url::Url;

use entity::{node, types::NodeType};

use self::raw::{Rdf, RdfDescription};

fn extract_redirect_path(index_html_path: &Path) -> Result<PathBuf> {
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
    db: &impl ConnectionTrait,
    file_id: i32,
    scrapbook_dir: &Path,
    description: RdfDescription,
) -> Result<node::Model> {
    let index_html = {
        let index_html = scrapbook_dir
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
    // Note that WinRAR doesn't save file creation time by default
    let created = metadata
        .as_ref()
        .and_then(|meta| meta.created().ok())
        .map(|time| DateTime::<Local>::from(time).naive_local());
    let modified = metadata
        .as_ref()
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

    fn none_if_empty<T: Deref<Target = str>>(string: T) -> Option<T> {
        match &*string {
            "" => None,
            _ => Some(string),
        }
    }

    let inserted_node = node::ActiveModel {
        file_id: Set(Some(file_id)),
        node_type: Set(NodeType::Scrapbook),
        subtype: Set(none_if_empty(description.r#type.clone())),
        title: Set(none_if_empty(description.title.clone())),
        url: Set(none_if_empty(description.source.clone())),
        created: Set(created),
        modified: Set(modified),
        original_id: Set(Some(description.id.clone())),
        data: Set(Some(serde_json::to_value(description)?)),
        ..Default::default()
    }
    .insert(db)
    .await?;

    Ok(inserted_node)
}

pub async fn import_from_file(
    db: &impl ConnectionTrait,
    file_path: &Path,
    file_id: i32,
) -> Result<Vec<node::Model>> {
    let mut inserted_nodes = vec![];

    let scrapbook_dir = file_path.parent().context("No parent folder")?;
    let rdf: Rdf = yaserde::de::from_str(&fs::read_to_string(file_path)?)
        .map_err(|err| eyre!("XML parse error: {err}"))?;

    let mut about_to_id: HashMap<String, i32> = HashMap::new();

    for description in rdf.descriptions {
        let about = description.about.clone();
        let inserted_node = insert_one(db, file_id, scrapbook_dir, description).await?;

        about_to_id.insert(about, inserted_node.id);
        inserted_nodes.push(inserted_node);
    }

    for sequence in rdf.sequences {
        if sequence.about == "urn:scrapbook:root" {
            // For now, mapping the scrapbook root to tree root
            continue;
        }

        let parent_id = about_to_id
            .get(&sequence.about)
            .with_context(|| format!("Missing about:{}", sequence.about))?;
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

    Ok(inserted_nodes)
}
