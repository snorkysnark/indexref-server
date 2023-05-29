use std::collections::HashMap;

use chrono::DateTime;
use eyre::ContextCompat;
use hyper::HeaderMap;
use once_cell::sync::Lazy;
use reqwest::Client;
use scraper::{Html, Selector};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::Deserialize;
use serde_json::Value as JsonValue;

use crate::{
    config::{ZoteroSource, ZoteroSourceType},
    entity::{node, types::NodeType, zotero},
    ext::JsonValueExt,
};

#[derive(Debug, Deserialize)]
struct ZoteroItem {
    key: String,
    version: i32,
    library: ZoteroLibrary,
    links: JsonValue,
    meta: JsonValue,
    data: JsonValue,
}

#[derive(Debug, Deserialize)]
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

    Ok(match item.data.get_ok("itemType")?.as_str_ok()? {
        "note" => {
            let html = item.data.get_ok("note")?.as_str_ok()?;

            Html::parse_fragment(html)
                .select(&*SELECT_H1)
                .next()
                .map(|node| node.text().collect::<Vec<_>>().join(" "))
        }
        "annotation" => item
            .data
            .get("annotationText")
            .or(item.data.get("annotationComment"))
            .and_then(|v| v.as_str())
            .map(ToOwned::to_owned),
        _ => item
            .data
            .get("title")
            .and_then(|v| v.as_str())
            .map(ToOwned::to_owned),
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
        let item_type = item
            .data
            .get("itemType")
            .and_then(|v| v.as_str())
            .map(ToOwned::to_owned);

        let date_modified = item
            .data
            .get("dateModified")
            .and_then(|v| v.as_str())
            .map(|v| DateTime::parse_from_rfc3339(v))
            .transpose()?
            .map(|date| date.naive_utc());
        let parent_key = item.data.get("parentItem").map(string_value).transpose()?;

        let node_inserted = node::ActiveModel {
            r#type: Set(NodeType::Zotero),
            subtype: Set(item_type),
            created: Set(date_modified),
            title: Set(title),
            original_id: Set(Some(item.key.clone())),
            ..Default::default()
        }
        .insert(db)
        .await?;

        zotero::ActiveModel {
            node_id: Set(node_inserted.id),
            version: Set(item.version),
            library_type: Set(item.library.r#type),
            library_id: Set(item.library.id),
            library_name: Set(item.library.name),
            library_links: Set(item.library.links),
            links: Set(item.links),
            meta: Set(item.meta),
            data: Set(item.data),
        }
        .insert(db)
        .await?;

        if let Some(parent_id) = parent_key {
            relations.push((parent_id, item.key.clone()));
        }
        id_map.insert(item.key, node_inserted.id);
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
