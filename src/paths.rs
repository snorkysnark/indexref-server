use std::{
    fs,
    path::{Path, PathBuf},
};

use directories_next::ProjectDirs;
use eyre::ContextCompat;

use crate::ext::PathExt;

pub struct ProjectPaths {
    #[allow(dead_code)]
    dirs: ProjectDirs,
    db_path: PathBuf,
    config_path: PathBuf,
}

impl ProjectPaths {
    pub fn init(qualifier: &str, organization: &str, application: &str) -> eyre::Result<Self> {
        let dirs = ProjectDirs::from(qualifier, organization, application)
            .context("Project dirs not found")?;

        let data_dir = dirs.data_dir();
        let config_dir = dirs.config_dir();

        fs::create_dir_all(data_dir)?;
        fs::create_dir_all(config_dir)?;

        Ok(Self {
            db_path: data_dir.join("index.db"),
            config_path: config_dir.join("config.toml"),
            dirs,
        })
    }

    pub fn db_connection_string(&self) -> eyre::Result<String> {
        Ok(format!("sqlite://{}?mode=rwc", self.db_path.try_to_str()?))
    }

    pub fn config_path(&self) -> &Path {
        &self.config_path
    }
}
