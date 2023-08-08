use std::path::PathBuf;

use eyre::Result;
use serde::de::DeserializeOwned;
use tracing::error;
use tryvial::try_block;

use super::FileWatcher;

pub struct JsonWatcher {
    file_path: PathBuf,
    file_watcher: FileWatcher,
}

impl JsonWatcher {
    pub fn new(file_path: PathBuf) -> Result<Self> {
        Ok(Self {
            file_watcher: FileWatcher::new(&file_path, false)?,
            file_path,
        })
    }

    pub async fn watch<T>(self, mut on_change: impl FnMut(T) -> ()) -> Result<()>
    where
        T: DeserializeOwned,
    {
        let mut notify_change = || {
            let result: Result<T> = try_block! {
                serde_json::from_str(&std::fs::read_to_string(&self.file_path)?)?
            };
            match result {
                Ok(value) => on_change(value),
                Err(err) => error!("{err}"),
            }
        };

        notify_change();
        self.file_watcher.watch(notify_change).await
    }
}
