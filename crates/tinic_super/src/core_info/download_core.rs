use std::sync::Arc;

use generics::{constants::cores_url, error_handle::ErrorHandle, retro_paths::RetroPaths};

use crate::{event::TinicSuperEventListener, tools::download::download_file};

pub async fn download_core(
    retro_paths: &RetroPaths,
    force_update: bool,
    event_listener: Arc<dyn TinicSuperEventListener>,
) -> Result<(), ErrorHandle> {
    let url = cores_url()?;
    let dest = retro_paths.temps.to_string().into();

    download_file(url, "cores.7z", dest, force_update, event_listener).await?;
    Ok(())
}
