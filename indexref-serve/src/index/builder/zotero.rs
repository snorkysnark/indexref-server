use std::collections::HashMap;

use chrono::DateTime;
use eyre::{bail, ContextCompat};
use hyper::HeaderMap;
use once_cell::sync::Lazy;
use reqwest::Client;
use scraper::{Html, Selector};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize, Serialize};
use serde_json::{Map as JsonMap, Value as JsonValue};

use crate::{
    config::{ZoteroSource, ZoteroSourceType},
    entity::{node, types::NodeType},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ZoteroItem {
    key: String,
    version: i32,
    library: ZoteroLibrary,
    links: JsonValue,
    meta: JsonValue,
    data: ZoteroItemData,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ZoteroItemData {
    item_type: String,
    date_added: String,
    date_modified: String,
    parent_item: Option<String>,
    title: Option<String>,
    note: Option<String>,
    annotation_text: Option<String>,
    annotation_comment: Option<String>,
    #[serde(flatten)]
    other: JsonMap<String, JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ZoteroLibrary {
    r#type: String,
    id: i32,
    name: String,
    links: JsonValue,
}

impl ZoteroSource {
    async fn fetch_items(&self) -> eyre::Result<Vec<ZoteroItem>> {
        let user_or_group = match self.source_type() {
            ZoteroSourceType::User(id) => format!("/users/{id}"),
            ZoteroSourceType::Group(id) => format!("/groups/{id}"),
        };

        let items = Client::new()
            .get(format!("https://api.zotero.org{user_or_group}/items"))
            .headers({
                let mut headers = HeaderMap::new();

                headers.insert("Zotero-API-Version", "3".parse().unwrap());
                if let Some(api_key) = self.api_key() {
                    headers.insert("Zotero-API-Key", api_key.parse()?);
                }

                headers
            })
            .send()
            .await?
            .json()
            .await?;

        Ok(items)
    }
}

fn extract_title(item: &ZoteroItem) -> eyre::Result<Option<String>> {
    static SELECT_H1: Lazy<Selector> = Lazy::new(|| Selector::parse("h1").unwrap());

    Ok(match item.data.item_type.as_str() {
        "note" => {
            let html = item.data.note.as_ref().context("note field missing")?;

            Html::parse_fragment(html)
                .select(&*SELECT_H1)
                .next()
                .map(|node| node.text().collect::<Vec<_>>().join(" "))
        }
        "annotation" => item
            .data
            .annotation_text
            .clone()
            .or(item.data.annotation_comment.clone()),
        _ => item.data.title.clone(),
    })
}

pub async fn insert_from_source(
    db: &DatabaseConnection,
    source: &ZoteroSource,
) -> eyre::Result<Vec<node::Model>> {
    let mut inserted_nodes = vec![];

    let mut relations: Vec<(String, String)> = vec![];
    let mut id_map: HashMap<String, i32> = HashMap::new();

    for item in source.fetch_items().await? {
        fn string_value(v: &JsonValue) -> eyre::Result<String> {
            v.as_str()
                .with_context(|| format!("Not a string: {v:?}"))
                .map(|str| str.to_owned())
        }

        let title = extract_title(&item)?;
        let item_type = item.data.item_type.to_owned();

        let date_added = DateTime::parse_from_rfc3339(&item.data.date_added)?.naive_utc();
        let date_modified = DateTime::parse_from_rfc3339(&item.data.date_modified)?.naive_utc();

        let item_key = item.key.clone();
        let parent_item = item.data.parent_item.clone();

        let zotero_select = if item_type == "annotation" {
            match parent_item.as_ref() {
                Some(parent_key) => format!(
                    "zotero://open-pdf/library/items/{parent_key}?annotation={}",
                    item.key
                ),
                None => bail!("annotation {} has not parent pdf", item.key),
            }
        } else {
            format!("zotero://select/library/items/{}", item.key)
        };

        let node_inserted = node::ActiveModel {
            r#type: Set(NodeType::Zotero),
            subtype: Set(Some(item_type)),
            created: Set(Some(date_added)),
            modified: Set(Some(date_modified)),
            title: Set(title),
            original_id: Set(Some(item.key.clone())),
            url: Set(Some(zotero_select)),
            data: Set(Some(serde_json::to_value(item)?)),
            ..Default::default()
        }
        .insert(db)
        .await?;

        if let Some(parent_key) = parent_item {
            relations.push((parent_key, item_key.clone()));
        }
        id_map.insert(item_key, node_inserted.id);
        inserted_nodes.push(node_inserted);
    }

    for (parent_key, child_key) in relations {
        let parent_id = *id_map.get(&parent_key).context(parent_key)?;
        let child_id = *id_map.get(&child_key).context(child_key)?;

        node::ActiveModel {
            id: Set(child_id),
            parent_id: Set(Some(parent_id)),
            ..Default::default()
        }
        .update(db)
        .await?;
    }

    Ok(inserted_nodes)
}
