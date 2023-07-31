use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct SourcesConfig {
    #[serde(default)]
    pub telegram: Vec<PathBuf>,
    #[serde(default)]
    pub single_file_z: Vec<PathBuf>,
    #[serde(default)]
    pub scrapbook: Vec<PathBuf>,
    #[serde(default)]
    pub onetab: Vec<PathBuf>,
}
