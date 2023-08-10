use std::{path::Path, time::Duration};

use eyre::{ContextCompat, Result};
use notify::{RecommendedWatcher, RecursiveMode};
use notify_debouncer_mini::{new_debouncer, DebounceEventResult, DebouncedEventKind, Debouncer};
use tracing::debug;

use crate::watcher::ok_log_errors;

pub fn new_file_watcher(
    timeout: Duration,
    tick_rate: Option<Duration>,
    file_path: impl AsRef<Path>,
    mut callback: impl FnMut() -> () + Send + 'static,
) -> Result<Debouncer<RecommendedWatcher>> {
    let file_path = std::fs::canonicalize(file_path.as_ref())?;

    let mut debouncer = new_debouncer(timeout, tick_rate, {
        let file_path = file_path.clone();
        move |result: DebounceEventResult| {
            debug!("{result:#?}");
            if let Some(events) = ok_log_errors(result) {
                if events
                    .iter()
                    .find(|event| event.kind == DebouncedEventKind::Any && event.path == file_path)
                    .is_some()
                {
                    callback();
                }
            }
        }
    })?;
    println!(
        "Watching {}",
        file_path
            .parent()
            .context("Path has no parent dir")?
            .display()
    );
    debouncer.watcher().watch(
        file_path.parent().context("Path has no parent dir")?,
        RecursiveMode::NonRecursive,
    )?;

    Ok(debouncer)
}
