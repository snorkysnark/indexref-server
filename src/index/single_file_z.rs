use chrono::NaiveDateTime;
use eyre::Context;
use fs_err as fs;
use once_cell::sync::Lazy;
use regex::Regex;
use scraper::{Html, Selector};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use walkdir::WalkDir;

use crate::{
    config::SingleFileZConfig,
    entity::{node, types::NodeType},
    ext::{PathExt, ResultExt},
    path_convert::ToRelativePath,
};

static DATE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\((\d{2}\.\d{2}.\d{4} \d{2}_\d{2}_\d{2})\)\.html$").unwrap());

pub async fn insert_from_folder(
    db: &DatabaseConnection,
    config: &SingleFileZConfig,
) -> eyre::Result<Vec<node::Model>> {
    let sel_title = Selector::parse("title").unwrap();
    let sel_canonical_link = Selector::parse(r#"link[rel="canonical"]"#).unwrap();

    let mut inserted_nodes = vec![];

    for entry in WalkDir::new(config.path())
        .into_iter()
        .filter_map(|result| result.ok_log_errors())
        .filter(|e| matches!(e.path().extension_str(), Some("html")))
    {
        let relative_path = entry.path().to_relative_path(config.path())?;

        // The 'zip' portion of the file is NOT valid UTF-8,
        // so we have to use lossy conversion
        let bytes = fs::read(entry.path())?;
        let html = String::from_utf8_lossy(&bytes);
        let document = Html::parse_document(&html);

        let title = document
            .select(&sel_title)
            .next()
            .and_then(|el| el.text().next())
            .map(|str| str.to_owned());
        let url = document
            .select(&sel_canonical_link)
            .next()
            .and_then(|el| el.value().attr("href"))
            .map(|str| str.to_owned());
        let created = config
            .date_regex()
            .unwrap_or(&*DATE_REGEX)
            .captures_iter(&entry.file_name().to_string_lossy())
            .next()
            .and_then(|captures| captures.get(1))
            .map(|date| {
                NaiveDateTime::parse_from_str(date.as_str(), "%d.%m.%Y %H_%M_%S")
                    .with_context(|| date.as_str().to_owned())
            })
            .transpose()?;

        let inserted = node::ActiveModel {
            r#type: Set(NodeType::SingleFileZ),
            title: Set(title),
            url: Set(url),
            file: Set(Some(relative_path.into())),
            created: Set(created),
            ..Default::default()
        }
        .insert(db)
        .await?;

        inserted_nodes.push(inserted);
    }

    Ok(inserted_nodes)
}
