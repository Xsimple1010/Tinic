use crate::event::TinicSuperEventListener;
use crate::tools::download::download_file;
use generics::constants::cores_url;
use generics::error_handle::ErrorHandle;
use generics::retro_paths::RetroPaths;
use std::path::PathBuf;
use std::sync::Arc;

pub async fn download_core(
    retro_paths: &RetroPaths,
    force_update: bool,
    blocking: bool,
    event_listener: Arc<dyn TinicSuperEventListener>,
) -> Result<(), ErrorHandle> {
    let temp_dir = PathBuf::from(&retro_paths.temps.to_string());
    let url = cores_url()?;

    let task = async move || {
        let s = download_file(url, "cores.7z", temp_dir, force_update, event_listener).await;

        println!("{s:?}");
    };

    if blocking {
        task().await;
    } else {
        tokio::task::spawn_blocking(task);
    }

    Ok(())
}
