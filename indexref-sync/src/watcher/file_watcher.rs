use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use eyre::{ContextCompat, Result};
use notify::RecursiveMode;
use notify_debouncer_mini::{new_debouncer, DebounceEventResult};
use tracing::error;

pub struct FileWatcher {
    file_path: PathBuf,
    parent: PathBuf,
}

impl FileWatcher {
    pub fn new(file_path: impl AsRef<Path>) -> Result<Self> {
        let file_path = std::fs::canonicalize(file_path.as_ref())?;
        let parent = file_path
            .parent()
            .context("Path has no parent dir")?
            .to_owned();

        Ok(Self { file_path, parent })
    }

    pub async fn watch(self, mut on_change: impl FnMut() -> ()) -> Result<()> {
        let (tx, mut rx) = tokio::sync::mpsc::channel(100);

        let mut debouncer = new_debouncer(
            Duration::from_secs(1),
            None,
            move |result: DebounceEventResult| match result {
                Ok(events) => {
                    if events
                        .iter()
                        .find(|event| event.path == self.file_path)
                        .is_some()
                    {
                        if let Err(err) = tx.blocking_send(()) {
                            error!("{err}")
                        }
                    }
                }
                Err(errors) => errors.iter().for_each(|err| error!("{err}")),
            },
        )?;
        debouncer
            .watcher()
            .watch(&self.parent, RecursiveMode::NonRecursive)?;

        while let Some(_) = rx.recv().await {
            on_change();
        }
        Ok(())
    }
}
