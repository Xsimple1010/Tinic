use crate::DownloadProgress;
use crate::event::TinicSuperEventListener;
use crate::infos::download::download_info;
use crate::infos::get_all_core_infos::get_all_core_infos;
use crate::infos::get_compatibility_core_infos::get_compatibility_core_infos;
use crate::infos::has_installed::has_installed;
use crate::infos::model::CoreInfo;
use crate::infos::read_file::read_info_file;
use crate::tools::extract_files::ExtractProgress;
use generics::error_handle::ErrorHandle;
use generics::retro_paths::RetroPaths;
use std::path::PathBuf;
use std::sync::Arc;

pub struct InfoHelper {
    pub(crate) event_listener: Arc<dyn TinicSuperEventListener>,
    pub(crate) retro_paths: RetroPaths,
}

#[derive(Debug)]
pub enum InfoEventType {
    Downloading(DownloadProgress),
    Extraction(ExtractProgress),
}

impl InfoHelper {
    pub async fn download(&self, force_update: bool) -> Result<(), ErrorHandle> {
        download_info(
            &self.retro_paths,
            force_update,
            false,
            self.event_listener.clone(),
        )
        .await
    }

    pub async fn download_blocking(&self, force_update: bool) -> Result<(), ErrorHandle> {
        download_info(
            &self.retro_paths,
            force_update,
            true,
            self.event_listener.clone(),
        )
        .await
    }

    pub async fn read_file(&self, file_path: &PathBuf) -> Result<CoreInfo, ErrorHandle> {
        read_info_file(file_path, &mut self.retro_paths.cores.to_string().into()).await
    }

    pub async fn get_infos(&self) -> Vec<CoreInfo> {
        get_all_core_infos(
            &self.retro_paths.infos.to_string(),
            &mut self.retro_paths.cores.to_string().into(),
        )
        .await
    }

    pub async fn get_compatibility_core_infos(&self, rom_path: &PathBuf) -> Vec<CoreInfo> {
        get_compatibility_core_infos(rom_path, &self.retro_paths).await
    }

    pub fn has_infos_installed(&self) -> bool {
        has_installed(PathBuf::from(self.retro_paths.infos.to_string())).unwrap_or(false)
    }
}
