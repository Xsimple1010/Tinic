use crate::event::TinicSuperEventListener;
use crate::infos::helper::InfoEventType;
use crate::tools::download::download_file;
use crate::tools::extract_files::{ExtractProgress, extract_zip_file};
use generics::constants::CORE_INFOS_URL;
use generics::error_handle::ErrorHandle;
use generics::retro_paths::RetroPaths;
use std::path::PathBuf;
use std::sync::Arc;

pub async fn download_info(
    retro_paths: &RetroPaths,
    force_update: bool,
    blocking: bool,
    event_listener: Arc<dyn TinicSuperEventListener>,
) -> Result<(), ErrorHandle> {
    let temp_dir = PathBuf::from(&retro_paths.temps.to_string());

    let path = download_file(
        CORE_INFOS_URL,
        "info.zip",
        temp_dir.clone(),
        force_update,
        |event| {
            event_listener.on_info_event(InfoEventType::Downloading(event));
        },
    )
    .await?;

    let info_out_dir = retro_paths.infos.to_string();

    let handle = move |event: ExtractProgress| {
        event_listener.on_info_event(InfoEventType::Extraction(event));
    };

    if blocking {
        let _ = tokio::task::spawn_blocking(move || {
            extract_zip_file(path, info_out_dir, handle).unwrap();
        })
        .await;
    } else {
        tokio::task::spawn_blocking(move || {
            extract_zip_file(path, info_out_dir, handle).unwrap();
        });
    }

    Ok(())
}
