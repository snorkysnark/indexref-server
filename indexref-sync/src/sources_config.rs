use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
pub struct SourcesConfig {
    #[serde(default)]
    telegram: Vec<PathBuf>,
    #[serde(default)]
    single_file_z: Vec<PathBuf>,
    #[serde(default)]
    scrapbook: Vec<PathBuf>,
    #[serde(default)]
    onetab: Vec<PathBuf>,
}

impl SourcesConfig {
    pub fn telegram(&self) -> &[PathBuf] {
        self.telegram.as_ref()
    }

    pub fn single_file_z(&self) -> &[PathBuf] {
        self.single_file_z.as_ref()
    }

    pub fn scrapbook(&self) -> &[PathBuf] {
        self.scrapbook.as_ref()
    }

    pub fn onetab(&self) -> &[PathBuf] {
        self.onetab.as_ref()
    }

    pub fn iter_paths(&self) -> impl Iterator<Item = &PathBuf> {
        self.telegram
            .iter()
            .chain(self.single_file_z.iter())
            .chain(self.scrapbook.iter())
            .chain(self.onetab.iter())
    }
}
