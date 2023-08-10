use std::{path::PathBuf, time::Duration};

use opensearch::OpenSearch;
use sea_orm::DatabaseConnection;
use tracing::{error, info, trace};

use crate::{ext::ResultExt, file_sync, sources_config::SourcesConfig};

use self::async_debouncer::new_json_watcher;

mod async_debouncer;

pub async fn sync_and_watch(
    config_path: PathBuf,
    db: DatabaseConnection,
    oss: OpenSearch,
) -> eyre::Result<()> {
    let (_debouncer, mut json_rx) =
        new_json_watcher::<SourcesConfig>(Duration::from_secs(1), None, config_path.clone())?;

    while let Ok(sources) = json_rx.recv().await {
        info!("Reloaded config: {sources:#?}");
        tokio::spawn({
            let db = db.clone();
            let oss = oss.clone();
            async move {
                file_sync::sync_db_with_sources(&db, &oss, &sources)
                    .await
                    .ok_log_errors();
            }
        });
    }

    Ok(())
}
