use crate::{
    event::TinicSuperEventListener,
    tools::{download::download_file, extract_files::extract_zip_file},
};
use generics::{constants::RDB_URL, error_handle::ErrorHandle, retro_paths::RetroPaths};
use std::sync::Arc;

pub async fn download_rdb(
    paths: RetroPaths,
    force_update: bool,
    event_listener: Arc<dyn TinicSuperEventListener>,
) -> Result<(), ErrorHandle> {
    tokio::task::spawn(async move {
        let d = download_file(
            RDB_URL,
            "database-rdb.zip",
            paths.temps.to_string().into(),
            force_update,
            event_listener.clone(),
        )
        .await
        .ok();

        println!("{d:?}");

        if let Some(path) = d {
            let _ = tokio::task::spawn_blocking(move || {
                let _ = extract_zip_file(path, paths.databases.to_string(), event_listener);
            })
            .await;
        }
    })
    .await
    .map_err(|e| ErrorHandle::new(&e.to_string()))
}
