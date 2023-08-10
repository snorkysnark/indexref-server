use std::{path::PathBuf, sync::Arc, time::Duration};

use notify_debouncer_mini::DebouncedEventKind;
use opensearch::OpenSearch;
use sea_orm::DatabaseConnection;
use tokio::{
    sync::{Mutex, RwLock},
    task::JoinHandle,
};
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tracing::{error, info, trace};

use crate::{ext::ResultExt, file_sync, sources_config::SourcesConfig};

use self::{
    async_debouncer::{new_async_debouncer, new_json_watcher, ok_log_errors},
    multi_watcher::MultiWatcher,
};

mod async_debouncer;
mod multi_watcher;

pub async fn sync_and_watch(
    config_path: PathBuf,
    db: DatabaseConnection,
    oss: OpenSearch,
) -> eyre::Result<()> {
    let (_json_watcher, mut json_rx) =
        new_json_watcher::<SourcesConfig>(Duration::from_secs(1), None, config_path.clone())?;

    let (mut dir_watcher, dir_rx) = {
        let (debouncer, rx) = new_async_debouncer(Duration::from_secs(1), None)?;
        (MultiWatcher::new(debouncer), rx)
    };

    let current_task = Arc::new(Mutex::new(None::<JoinHandle<()>>));
    let current_config = Arc::new(RwLock::new(None));

    let spawn_task = {
        let current_config = current_config.clone();
        move || {
            let db = db.clone();
            let oss = oss.clone();
            let current_config = current_config.clone();

            tokio::spawn(async move {
                match current_config.read().await.as_ref() {
                    Some(sources) => {
                        file_sync::sync_db_with_sources(&db, &oss, sources)
                            .await
                            .ok_log_errors();
                    }
                    None => error!("Sync task spawned before config was ready"),
                }
            })
        }
    };

    tokio::spawn({
        let spawn_task = spawn_task.clone();
        let current_task = current_task.clone();
        async move {
            while let Ok(sources) = json_rx.recv().await {
                info!("Reloaded config: {sources:#?}");

                let source_dirs = sources.iter_paths().map(ToOwned::to_owned).collect();

                let mut current_task = current_task.lock().await;
                if let Some(current_task) = current_task.as_ref() {
                    current_task.abort();
                }
                *current_config.write().await = Some(sources);
                *current_task = Some(spawn_task());

                dir_watcher.update_paths(source_dirs);
            }
        }
    });

    let mut dir_stream = ReceiverStream::new(dir_rx)
        .filter_map(ok_log_errors)
        .filter(|events| {
            events
                .iter()
                .find(|event| event.kind == DebouncedEventKind::Any)
                .is_some()
        });
    while let Some(events) = dir_stream.next().await {
        info!("Files changed: {events:#?}");

        let mut current_task = current_task.lock().await;
        if let Some(current_task) = current_task.as_ref() {
            current_task.abort();
        }
        *current_task = Some(spawn_task());
    }

    Ok(())
}
