use std::{
    fmt::Display,
    path::PathBuf,
    sync::{Arc, Mutex, RwLock},
    time::Duration,
};

use eyre::eyre;
use notify_debouncer_mini::{new_debouncer, DebounceEventResult, DebouncedEventKind};
use opensearch::OpenSearch;
use sea_orm::DatabaseConnection;
use tokio::task::JoinHandle;
use tracing::{debug, error, info};

mod file_watcher;
mod json_watcher;
mod multi_watcher;

use crate::{ext::ResultExt, sources_config::SourcesConfig};

use self::{json_watcher::new_json_watcher, multi_watcher::MultiWatcher};

fn ok_log_errors<T, E>(result: Result<T, Vec<E>>) -> Option<T>
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

pub fn sync_and_watch(
    config_path: PathBuf,
    db: DatabaseConnection,
    oss: OpenSearch,
) -> eyre::Result<()> {
    let current_config = Arc::new(RwLock::new(None::<SourcesConfig>));
    let last_task = Arc::new(Mutex::new(None::<JoinHandle<()>>));

    fn sync_files(
        db: DatabaseConnection,
        oss: OpenSearch,
        sources: SourcesConfig,
        last_task: Arc<Mutex<Option<JoinHandle<()>>>>,
    ) -> eyre::Result<()> {
        let mut last_task = last_task.lock().map_err(|err| eyre!("{err}"))?;
        if let Some(last_task) = last_task.as_ref() {
            last_task.abort();
        }

        *last_task = Some(tokio::spawn(async move {
            if let Err(err) = crate::file_sync::sync_db_with_sources(&db, &oss, &sources).await {
                error!("Sync failed: {err}");
            }
        }));

        Ok(())
    }

    let mut multi_watcher = MultiWatcher::new(new_debouncer(Duration::from_secs(1), None, {
        let db = db.clone();
        let oss = oss.clone();
        let current_config = current_config.clone();
        let last_task = last_task.clone();

        move |result: DebounceEventResult| {
            debug!("{result:#?}");
            if let Some(events) = ok_log_errors(result) {
                if events
                    .iter()
                    .find(|event| event.kind == DebouncedEventKind::Any)
                    .is_some()
                {
                    if let Some(sources) = current_config.read().as_ref().ok_log_errors() {
                        match sources.as_ref() {
                            Some(sources) => {
                                sync_files(
                                    db.clone(),
                                    oss.clone(),
                                    sources.clone(),
                                    last_task.clone(),
                                )
                                .ok_log_errors();
                            }
                            None => error!("Sync task spawned before config was ready"),
                        }
                    }
                }
            }
        }
    })?);

    // let mut first_run = false;
    let _debouncer = new_json_watcher(
        Duration::from_secs(1),
        None,
        config_path,
        move |sources: SourcesConfig| {
            info!("Reloaded config: {sources:#?}");
            multi_watcher.update_paths(sources.iter_paths().map(ToOwned::to_owned).collect());
            if let Some(mut current_config) = current_config.write().ok_log_errors() {
                *current_config = Some(sources.clone());
            }

            // if first_run {
            sync_files(db.clone(), oss.clone(), sources, last_task.clone()).ok_log_errors();
            //     first_run = false;
            // }
        },
    )?;

    loop {}
}
