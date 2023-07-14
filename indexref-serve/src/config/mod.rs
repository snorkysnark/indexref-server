mod sources;

use std::path::Path;

use fs_err as fs;
use serde::Deserialize;

pub use self::sources::*;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub sources: SourcesConfig,
}

impl AppConfig {
    pub fn load(path: &Path) -> eyre::Result<Self> {
        Ok(toml::from_str(&fs::read_to_string(path)?)?)
    }
}
