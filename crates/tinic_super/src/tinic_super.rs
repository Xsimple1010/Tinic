use crate::art::download_all_thumbnail_from_game;
use crate::core_info::helper::CoreInfoHelper;
use crate::core_info::model::CoreInfo;
use crate::event::TinicSuperEventListener;
use crate::rdb_manager::game::GameInfo;
use crate::rdb_manager::helper::RdbManager;
use generics::error_handle::ErrorHandle;
use generics::retro_paths::RetroPaths;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::sync::Arc;

pub struct TinicSuper {
    pub retro_paths: RetroPaths,
    pub event_listener: Arc<dyn TinicSuperEventListener>,
}

impl TinicSuper {
    pub async fn try_update_core(&self, force_update: bool) -> Result<(), ErrorHandle> {
        CoreInfoHelper::try_update_core_infos(
            &self.retro_paths,
            force_update,
            self.event_listener.clone(),
        )
        .await
    }

    pub async fn install_cores_and_rdb(
        &self,
        core_info: &Vec<CoreInfo>,
        force_update: bool,
    ) -> Result<(), ErrorHandle> {
        let core_names: Vec<String> = core_info.iter().map(|c| c.file_name.clone()).collect();
        CoreInfoHelper::install_core(&self.retro_paths, &core_names);

        for core in core_info {
            let retro_path = self.retro_paths.clone();
            let on_progress = self.event_listener.clone();
            let database = core.database.clone();

            let _ = tokio::spawn(async move {
                let _ =
                    RdbManager::download(&retro_path, &database, force_update, on_progress).await;
            })
            .await;
        }

        Ok(())
    }

    pub fn get_compatibility_core_infos(&self, rom_file: &str) -> Vec<CoreInfo> {
        CoreInfoHelper::get_compatibility_core_infos(&rom_file.into(), &self.retro_paths)
    }

    pub fn read_rdb_to_end<C: FnMut(Vec<GameInfo>) + Sync + Send + Copy>(
        &self,
        rdb_name: &Vec<String>,
        callback: C,
    ) -> Result<(), ErrorHandle> {
        rdb_name.par_iter().for_each(|rdb_name| {
            let rdb_path = format!("{}/{}.rdb", self.retro_paths.databases, rdb_name);
            let _ = RdbManager::read_to_end(&rdb_path, callback);
        });

        Ok(())
    }

    pub fn has_core_installed(&self) -> bool {
        CoreInfoHelper::has_core_installed(&self.retro_paths)
    }

    pub async fn download_all_thumbnail_from_game(&self, sys_name: &str, rom_name: &str) {
        download_all_thumbnail_from_game(
            sys_name,
            rom_name,
            &self.retro_paths.arts,
            self.event_listener.clone(),
        )
        .await;
    }
}
