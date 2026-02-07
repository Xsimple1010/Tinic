use std::path::PathBuf;
use std::sync::Arc;
use generics::constants::{cores_url, CORE_INFOS_URL};
use generics::error_handle::ErrorHandle;
use generics::retro_paths::RetroPaths;
use crate::download::download_file;
use crate::event::TinicSuperEventListener;
use crate::extract_files::extract_zip_file;

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

    extract_zip_file(
        path,
        retro_paths.infos.clone().to_string(),
        event_listener.clone(),
    )
        .unwrap();

    let core_url = cores_url()?;
    download_file(core_url, "cores.7z", temp_dir, force_update, event_listener)
        .await
        .map_err(|e| ErrorHandle::new(&e.to_string()))?;

    Ok(())
}