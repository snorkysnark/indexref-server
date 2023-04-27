use std::{fs, path::PathBuf};

use directories_next::ProjectDirs;

use crate::{
    ext::PathExt,
    result::{AppError, AppResult},
};

pub struct ProjectPaths {
    #[allow(dead_code)]
    dirs: ProjectDirs,
    db_path: PathBuf,
    config_path: PathBuf,
}

impl ProjectPaths {
    pub fn init(qualifier: &str, organization: &str, application: &str) -> AppResult<Self> {
        let dirs = ProjectDirs::from(qualifier, organization, application)
            .ok_or(AppError::ProjectDirsNotFound)?;

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

    pub fn db_connection_string(&self) -> AppResult<String> {
        Ok(format!("sqlite://{}?mode=rwc", self.db_path.try_to_str()?))
    }
}
