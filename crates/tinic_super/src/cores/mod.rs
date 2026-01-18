use crate::cores::download::download_core;
use crate::cores::installed::install_core;
use crate::event::TinicSuperEventListener;
use generics::error_handle::ErrorHandle;
use generics::retro_paths::RetroPaths;
use std::sync::Arc;

pub mod download;
pub mod installed;

pub struct CoreHelper {
    pub(crate) event_listener: Arc<dyn TinicSuperEventListener>,
    pub(crate) retro_paths: RetroPaths,
}

impl CoreHelper {
    pub async fn download(&self, force_update: bool) -> Result<(), ErrorHandle> {
        download_core(
            &self.retro_paths,
            force_update,
            false,
            self.event_listener.clone(),
        )
        .await
    }

    pub async fn download_blocking(&self, force_update: bool) -> Result<(), ErrorHandle> {
        download_core(
            &self.retro_paths,
            force_update,
            true,
            self.event_listener.clone(),
        )
        .await
    }

    pub async fn install(&self, core_file_names: Vec<String>) {
        install_core(
            self.retro_paths.clone(),
            core_file_names,
            false,
            self.event_listener.clone(),
        )
        .await
    }

    pub async fn install_blocking(&self, core_file_names: Vec<String>) {
        install_core(
            self.retro_paths.clone(),
            core_file_names,
            true,
            self.event_listener.clone(),
        )
        .await
    }
}
