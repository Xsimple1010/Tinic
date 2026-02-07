use std::{path::PathBuf, sync::Arc};

use futures_util::{StreamExt, stream::FuturesUnordered};
use generics::{constants::RDB_BASE_URL, error_handle::ErrorHandle, retro_paths::RetroPaths};

use crate::{event::TinicSuperEventListener, tools::download::download_file};

pub async fn download_rdb(
    paths: &RetroPaths,
    rdbs: &Vec<String>,
    force_update: bool,
    event_listener: Arc<dyn TinicSuperEventListener>,
) -> Result<(), ErrorHandle> {
    if rdbs.is_empty() {
        return Err(ErrorHandle::new("dbs is empty"));
    }

    let mut dbs: Vec<String> = Vec::new();
    for rdb in rdbs {
        if !rdb.ends_with(".rdb") {
            dbs.push(format!("{rdb}.rdb"));
        }
    }

    let mut tasks = FuturesUnordered::new();

    for rdb_name in dbs {
        let rdb_path = PathBuf::from(paths.databases.to_string()).join(rdb_name.clone());

        if rdb_path.exists() {
            continue;
        }

        let url = format!("{RDB_BASE_URL}/{rdb_name}");
        let databases_dir = PathBuf::from(paths.databases.to_string());
        let event_listener = event_listener.clone();

        tasks.push(async move {
            download_file(&url, &rdb_name, databases_dir, force_update, event_listener).await
        });
    }

    while let Some(_) = tasks.next().await {}

    Ok(())
}
