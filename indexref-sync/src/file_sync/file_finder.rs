use std::path::{Path, PathBuf};

use entity::types::FileType;
use walkdir::WalkDir;

use crate::{
    sources_config::SourcesConfig,
    ext::{OptionOsStrExt, ResultExt},
};

#[derive(Debug)]
pub struct FoundFile {
    pub file_type: FileType,
    pub path: PathBuf,
}

pub fn find_all<'a>(config: &'a SourcesConfig) -> impl Iterator<Item = FoundFile> + 'a {
    (config.telegram().iter().map(find_telegram).flatten())
        .chain(
            config
                .single_file_z()
                .iter()
                .map(find_singlefile_z)
                .flatten(),
        )
        .chain(config.scrapbook().iter().map(find_scrapbooks).flatten())
        .chain(config.onetab().iter().map(find_onetab).flatten())
}

fn find_telegram(source_dir: impl AsRef<Path>) -> impl Iterator<Item = FoundFile> {
    walk_dir(source_dir.as_ref())
        .filter(|path| path.file_name().to_str() == Some("result.json"))
        .map(|path| FoundFile {
            file_type: FileType::Telegram,
            path,
        })
}

fn find_singlefile_z(source_dir: impl AsRef<Path>) -> impl Iterator<Item = FoundFile> {
    walk_dir(source_dir.as_ref())
        .filter(|path| path.extension().to_str() == Some("html"))
        .map(|path| FoundFile {
            file_type: FileType::SingleFileZ,
            path,
        })
}

fn find_scrapbooks(source_dir: impl AsRef<Path>) -> impl Iterator<Item = FoundFile> {
    walk_dir(source_dir.as_ref())
        .filter(|path| path.file_name().to_str() == Some("scrapbook.rdf"))
        .map(|path| FoundFile {
            file_type: FileType::Scrapbook,
            path,
        })
}

fn find_onetab(source_dir: impl AsRef<Path>) -> impl Iterator<Item = FoundFile> {
    walk_dir(source_dir.as_ref())
        .filter(|path| path.extension().to_str() == Some("json"))
        .map(|path| FoundFile {
            file_type: FileType::OneTab,
            path,
        })
}

fn walk_dir(dir: &Path) -> impl Iterator<Item = PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|result| result.ok_log_errors())
        .map(|entry| entry.into_path())
}
