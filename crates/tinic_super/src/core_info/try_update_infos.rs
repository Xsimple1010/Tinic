use crate::event::TinicSuperEventListener;
use crate::tools::download::download_file;
use crate::tools::extract_files::extract_zip_file;
use generics::constants::{CORE_INFOS_URL, cores_url};
use generics::error_handle::ErrorHandle;
use generics::retro_paths::RetroPaths;
use std::path::PathBuf;
use std::sync::Arc;

pub async fn try_update_core_infos(
    retro_paths: &RetroPaths,
    force_update: bool,
    event_listener: Arc<dyn TinicSuperEventListener>,
) -> Result<(), ErrorHandle> {
    let temp_dir = PathBuf::from(&retro_paths.temps.to_string());

    let path = download_file(
        CORE_INFOS_URL,
        "info.zip",
        temp_dir.clone(),
        force_update,
        event_listener.clone(),
    )
    .await
    .map_err(|e| ErrorHandle::new(&e.to_string()))?;

    let info_out_dir = retro_paths.infos.to_string();
    let event_listener_2 = event_listener.clone();

    tokio::task::spawn_blocking(move || {
        extract_zip_file(path, info_out_dir, event_listener_2).unwrap();
    });

    let core_url = cores_url()?;
    download_file(core_url, "cores.7z", temp_dir, force_update, event_listener)
        .await
        .map_err(|e| ErrorHandle::new(&e.to_string()))?;

    Ok(())
}
