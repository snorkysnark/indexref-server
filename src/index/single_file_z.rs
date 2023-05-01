use std::{fs, path::Path};

use scraper::{Html, Selector};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use walkdir::WalkDir;

use crate::{
    ext::{PathExt, ResultExt},
    path_convert::ToRelativePath,
    result::AppResult,
};
use entity::{node, types::NodeType};

pub async fn insert_from_folder(
    db: &DatabaseConnection,
    folder: &Path,
) -> AppResult<Vec<node::Model>> {
    let sel_title = Selector::parse("title").unwrap();
    let sel_canonical_link = Selector::parse(r#"link[rel="canonical"]"#).unwrap();

    let mut inserted_nodes = vec![];

    for entry in WalkDir::new(folder)
        .into_iter()
        .filter_map(|result| result.ok_log_errors())
        .filter(|e| matches!(e.path().extension_str(), Some("html")))
    {
        let relative_path = entry.path().to_relative_path(folder)?;

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

        // TODO: Extract date from filename

        let inserted = node::ActiveModel {
            r#type: Set(NodeType::SingleFileZ),
            title: Set(title),
            url: Set(url),
            file: Set(Some(relative_path.into())),
            ..Default::default()
        }
        .insert(db)
        .await?;

        inserted_nodes.push(inserted);
    }

    Ok(inserted_nodes)
}
