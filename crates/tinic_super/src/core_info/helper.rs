use crate::core_info::get_all_core_infos::get_all_core_infos;
use crate::core_info::get_compatibility_core_infos::get_compatibility_core_infos;
use crate::core_info::has_core_installed::has_core_installed;
use crate::core_info::install_core::install_core;
use crate::core_info::model::CoreInfo;
use crate::core_info::read_file::read_info_file;
use crate::core_info::try_update_infos::try_update_core_infos;
use crate::event::TinicSuperEventListener;
use generics::error_handle::ErrorHandle;
use generics::retro_paths::RetroPaths;
use std::path::PathBuf;
use std::sync::Arc;

pub struct CoreInfoHelper {
    pub(crate) event_listener: Arc<dyn TinicSuperEventListener>,
    pub(crate) retro_paths: RetroPaths,
}

impl CoreInfoHelper {
    pub async fn try_update_core_infos(&self, force_update: bool) -> Result<(), ErrorHandle> {
        try_update_core_infos(&self.retro_paths, force_update, self.event_listener.clone()).await
    }

    pub async fn read_info_file(&self, file_path: &PathBuf) -> Result<CoreInfo, ErrorHandle> {
        read_info_file(file_path).await
    }

    pub async fn install_core(&self, core_file_name: Vec<String>) {
        install_core(self.retro_paths.clone(), core_file_name).await;
    }

    pub fn has_core_installed(&self) -> bool {
        has_core_installed(&self.retro_paths)
    }

    pub async fn get_core_infos(&self, dir: &String) -> Vec<CoreInfo> {
        get_all_core_infos(dir).await
    }

    pub async fn get_compatibility_core_infos(&self, rom_path: &PathBuf) -> Vec<CoreInfo> {
        get_compatibility_core_infos(rom_path, &self.retro_paths).await
    }
}
