use std::{path::PathBuf, time::Duration};

use eyre::Result;
use notify::RecommendedWatcher;
use notify_debouncer_mini::Debouncer;
use serde::de::DeserializeOwned;
use tracing::error;
use tryvial::try_block;

use super::file_watcher::new_file_watcher;

pub fn new_json_watcher<T: DeserializeOwned>(
    timeout: Duration,
    tick_rate: Option<Duration>,
    file_path: PathBuf,
    mut callback: impl FnMut(T) -> () + Send + 'static,
) -> Result<Debouncer<RecommendedWatcher>> {
    let mut notify_change = {
        let file_path = file_path.clone();
        move || {
            let result: Result<T> = try_block! {
                serde_json::from_str(&std::fs::read_to_string(&file_path)?)?
            };
            match result {
                Ok(value) => callback(value),
                Err(err) => error!("{err}"),
            }
        }
    };

    notify_change();
    let debouncer = new_file_watcher(timeout, tick_rate, &file_path, notify_change)?;
    Ok(debouncer)
}
