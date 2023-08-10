use std::{
    fmt::Display,
    path::{Path, PathBuf},
    time::Duration,
};

use eyre::ContextCompat;
use notify::{RecommendedWatcher, RecursiveMode};
use notify_debouncer_mini::{new_debouncer, DebounceEventResult, DebouncedEventKind, Debouncer};
use serde::de::DeserializeOwned;
use tokio::sync::{broadcast, mpsc};
use tracing::error;
use tryvial::try_block;

use crate::ext::ResultExt;

pub fn new_async_debouncer(
    timeout: Duration,
    tick_rate: Option<Duration>,
) -> Result<
    (
        Debouncer<RecommendedWatcher>,
        mpsc::Receiver<DebounceEventResult>,
    ),
    notify::Error,
> {
    let (tx, rx) = mpsc::channel(100);
    let debouncer = new_debouncer(timeout, tick_rate, move |result: DebounceEventResult| {
        if let Err(err) = tx.blocking_send(result) {
            error!("{err}");
        }
    })?;

    Ok((debouncer, rx))
}

pub fn ok_log_errors<T, E>(result: Result<T, Vec<E>>) -> Option<T>
where
    E: Display,
{
    match result {
        Ok(value) => Some(value),
        Err(errors) => {
            errors.iter().for_each(|err| error!("{err}"));
            None
        }
    }
}

pub fn new_file_watcher(
    timeout: Duration,
    tick_rate: Option<Duration>,
    file_path: impl AsRef<Path>,
) -> eyre::Result<(Debouncer<RecommendedWatcher>, broadcast::Receiver<()>)> {
    let file_path = std::fs::canonicalize(file_path.as_ref())?;

    let (mut debouncer, mut events_rx) = new_async_debouncer(timeout, tick_rate)?;
    debouncer.watcher().watch(
        file_path.parent().context("Path has no parent dir")?,
        RecursiveMode::NonRecursive,
    )?;

    // Since the message carries no data, only the notification matters
    let (change_tx, change_rx) = broadcast::channel(1);

    tokio::spawn(async move {
        while let Some(result) = events_rx.recv().await {
            if let Some(events) = ok_log_errors(result) {
                if events
                    .iter()
                    .find(|event| event.kind == DebouncedEventKind::Any && event.path == file_path)
                    .is_some()
                {
                    change_tx.send(()).ok_log_errors();
                }
            }
        }
    });

    Ok((debouncer, change_rx))
}

pub fn new_json_watcher<T>(
    timeout: Duration,
    tick_rate: Option<Duration>,
    file_path: PathBuf,
) -> eyre::Result<(Debouncer<RecommendedWatcher>, broadcast::Receiver<T>)>
where
    T: DeserializeOwned + Clone + Send + 'static,
{
    // Since the message carries no data, only the notification matters
    let (json_tx, json_rx) = broadcast::channel(1);

    let read_json = {
        let file_path = file_path.clone();
        move || {
            let result: eyre::Result<T> = try_block! {
                serde_json::from_str(&std::fs::read_to_string(&file_path)?)?
            };
            if let Some(value) = result.ok_log_errors() {
                json_tx.send(value).ok_log_errors();
            }
        }
    };
    read_json();

    let (debouncer, mut change_rx) = new_file_watcher(timeout, tick_rate, file_path)?;
    tokio::spawn({
        let read_json = read_json.clone();
        async move {
            while let Ok(_) = change_rx.recv().await {
                read_json();
            }
        }
    });

    Ok((debouncer, json_rx))
}
