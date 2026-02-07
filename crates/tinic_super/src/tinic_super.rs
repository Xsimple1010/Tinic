use crate::art::{get_thumbnail_url, ThumbnailType};
use crate::core_info::helper::CoreInfoHelper;
use crate::core_info::model::CoreInfo;
use crate::database::game::GameInfo;
use crate::database::helper::{DatabaseHelper, RDBDatabase};
use crate::FileProgress;
use generics::error_handle::ErrorHandle;
use generics::retro_paths::RetroPaths;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::sync::Arc;

pub struct TinicSuper {
    pub retro_paths: RetroPaths,
}

impl TinicSuper {
    pub async fn try_update_core<CP>(
        &self,
        force_update: bool,
        on_progress: Arc<dyn Fn(FileProgress) + Send + Sync>,
    ) -> Result<(), ErrorHandle> {
        CoreInfoHelper::try_update_core_infos(&self.retro_paths, force_update, on_progress).await
    }

    pub async fn install_cores(
        &self,
        core_info: &Vec<CoreInfo>,
        force_update: bool,
        on_progress: Arc<dyn Fn(FileProgress) + Send + Sync>,
    ) -> Result<(), ErrorHandle> {
        let core_names: Vec<String> = core_info.iter().map(|c| c.file_name.clone()).collect();
        CoreInfoHelper::install_core(&self.retro_paths, &core_names);

        for core in core_info {
            let retro_path = self.retro_paths.clone();
            let on_progress = on_progress.clone();
            let database = core.database.clone();

            tokio::spawn(async move {
                let _ =
                    DatabaseHelper::download_db(&retro_path, &database, force_update, on_progress)
                        .await;
            });
        }

        Ok(())
    }

    pub async fn get_all_game_infos(rdb_file: String) -> Result<Vec<GameInfo>, ErrorHandle> {
        DatabaseHelper { rdb_file }.get_all_games()
    }

    pub fn get_compatibility_core_infos(&self, rom_file: &str) -> Vec<CoreInfo> {
        CoreInfoHelper::get_compatibility_core_infos(&rom_file.into(), &self.retro_paths)
    }

    pub fn identifier_rom_file(&self, rom_file: &str, cores: &Vec<CoreInfo>) -> Option<(GameInfo, RDBDatabase)> {
        cores.par_iter().find_map_any(|core| {
            DatabaseHelper::identifier_rom_file_with_any_rdb(
                rom_file,
                core,
                &self.retro_paths.databases,
            )
            .unwrap_or_else(|_| None)
        })
    }

    pub fn get_thumbnail(
        &self,
        thumbnail_type: ThumbnailType,
        sys_name: &str,
        name: &str,
    ) -> String {
        get_thumbnail_url(thumbnail_type, sys_name, name)
    }
}
