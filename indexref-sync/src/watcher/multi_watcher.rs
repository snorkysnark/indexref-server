use std::{collections::HashSet, path::PathBuf, time::Duration};

use eyre::Result;
use notify::{RecommendedWatcher, RecursiveMode};
use notify_debouncer_mini::{new_debouncer, DebounceEventResult, Debouncer};
use tokio::sync::mpsc::Receiver;
use tracing::error;

pub struct MultiWatcher {
    debouncer: Debouncer<RecommendedWatcher>,
    watcher_paths: HashSet<PathBuf>,
}

pub fn create_multi_watcher() -> Result<(MultiWatcher, Receiver<DebounceEventResult>)> {
    let (tx, rx) = tokio::sync::mpsc::channel(100);

    let debouncer = new_debouncer(
        Duration::from_secs(1),
        None,
        move |result: DebounceEventResult| {
            if let Err(err) = tx.blocking_send(result) {
                error!("{err}")
            }
        },
    )?;

    Ok((
        MultiWatcher {
            debouncer,
            watcher_paths: HashSet::new(),
        },
        rx,
    ))
}

impl MultiWatcher {
    pub fn update_paths(&mut self, new_paths: HashSet<PathBuf>) {
        let to_delete: Vec<PathBuf> = self
            .watcher_paths
            .difference(&new_paths)
            .map(ToOwned::to_owned)
            .collect();

        let to_add: Vec<PathBuf> = new_paths
            .into_iter()
            .filter(|path| !self.watcher_paths.contains(path))
            .collect();

        for path in to_delete.iter() {
            match self.debouncer.watcher().unwatch(path) {
                Ok(_) => {
                    self.watcher_paths.remove(path);
                }
                Err(err) => error!("Can't unwatch {path}: {err}", path = path.display()),
            }
        }
        for path in to_add.into_iter() {
            match self
                .debouncer
                .watcher()
                .watch(&path, RecursiveMode::Recursive)
            {
                Ok(_) => {
                    self.watcher_paths.insert(path);
                }
                Err(err) => error!("Can't watch {path}: {err}", path = path.display()),
            }
        }
    }
}
