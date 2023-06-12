use std::path::{Path, PathBuf};

use eyre::ContextCompat;
use fs_err as fs;

pub struct ProjectPaths {
    config_path: PathBuf,
}

impl ProjectPaths {
    pub fn init(program_name: &str) -> eyre::Result<Self> {
        let config_dir = dirs_next::config_dir()
            .context("config dir not found")?
            .join(program_name);

        fs::create_dir_all(&config_dir)?;

        Ok(Self {
            config_path: config_dir.join("config.toml"),
        })
    }

    pub fn config_path(&self) -> &Path {
        &self.config_path
    }
}
