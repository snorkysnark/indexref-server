use std::{fs, path::Path};

use chrono::NaiveDateTime;
use eyre::{Context, ContextCompat, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use scraper::{Html, Selector};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

use entity::{node, types::NodeType};

static DATE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\((\d{2}\.\d{2}.\d{4} \d{2}_\d{2}_\d{2})\)\.html$").unwrap());
static SEL_TITLE: Lazy<Selector> = Lazy::new(|| Selector::parse("title").unwrap());
static SEL_CANONICAL_LINK: Lazy<Selector> =
    Lazy::new(|| Selector::parse(r#"link[rel="canonical"]"#).unwrap());

pub async fn import_from_file(
    db: &DatabaseConnection,
    file_path: &Path,
    file_id: i32,
) -> Result<Vec<node::Model>> {
    // The 'zip' portion of the file is NOT valid UTF-8,
    // so we have to use lossy conversion
    let bytes = fs::read(file_path)?;
    let html = String::from_utf8_lossy(&bytes);
    let document = Html::parse_document(&html);

    let filename = file_path
        .file_name()
        .context("Not a file path")?
        .to_str()
        .context("Non-Utf8 filename")?;

    let title = document
        .select(&SEL_TITLE)
        .next()
        .and_then(|el| el.text().next())
        .map(|str| str.to_owned());
    let url = document
        .select(&SEL_CANONICAL_LINK)
        .next()
        .and_then(|el| el.value().attr("href"))
        .map(|str| str.to_owned());
    let created = DATE_REGEX
        .captures_iter(filename)
        .next()
        .and_then(|captures| captures.get(1))
        .map(|date| {
            NaiveDateTime::parse_from_str(date.as_str(), "%d.%m.%Y %H_%M_%S")
                .with_context(|| date.as_str().to_owned())
        })
        .transpose()?;

    let inserted = node::ActiveModel {
        file_id: Set(Some(file_id)),
        node_type: Set(NodeType::SingleFileZ),
        title: Set(title),
        url: Set(url),
        created: Set(created),
        ..Default::default()
    }
    .insert(db)
    .await?;

    Ok(vec![inserted])
}
