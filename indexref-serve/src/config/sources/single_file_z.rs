use std::path::{Path, PathBuf};

use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct SingleFileZConfig {
    path: PathBuf,
    date_regex: Option<Regex>,
}

impl SingleFileZConfig {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn date_regex(&self) -> Option<&Regex> {
        self.date_regex.as_ref()
    }
}

impl<'de> Deserialize<'de> for SingleFileZConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum RawConfig {
            PathOnly(PathBuf),
            Full {
                path: PathBuf,
                #[serde(with = "serde_regex", default)]
                date_regex: Option<Regex>,
            },
        }

        match RawConfig::deserialize(deserializer)? {
            RawConfig::PathOnly(path) => Ok(Self {
                path,
                date_regex: None,
            }),
            RawConfig::Full { path, date_regex } => Ok(Self { path, date_regex }),
        }
    }
}
