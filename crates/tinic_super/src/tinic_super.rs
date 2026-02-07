use crate::core_info::helper::CoreInfoHelper;
use crate::core_info::model::CoreInfo;
use crate::database::game::GameInfo;
use crate::database::helper::DatabaseHelper;
use crate::FileProgress;
use generics::error_handle::ErrorHandle;
use generics::retro_paths::RetroPaths;

pub struct TinicSuper {
    pub retro_paths: RetroPaths,
}

impl TinicSuper {
    pub async fn try_update_core<CP>(
        &self,
        force_update: bool,
        on_progress: CP,
    ) -> Result<(), ErrorHandle>
    where
        CP: Fn(FileProgress) + Copy,
    {
        CoreInfoHelper::try_update_core_infos(&self.retro_paths, force_update, on_progress).await
    }

    pub async fn install_core<CP>(
        &self,
        core_info: &CoreInfo,
        force_update: bool,
        on_progress: CP,
    ) -> Result<(), ErrorHandle>
    where
        CP: Fn(FileProgress) + Copy,
    {
        CoreInfoHelper::install_core(&self.retro_paths, core_info);

        let _ = DatabaseHelper::download_db(
            &self.retro_paths,
            &core_info.database,
            force_update,
            on_progress,
        )
        .await;

        Ok(())
    }

    pub async fn get_all_game_infos(rdb_file: String) -> Result<Vec<GameInfo>, ErrorHandle> {
        DatabaseHelper { rdb_file }.get_all_games()
    }

    pub fn get_compatibility_core_infos(&self, rom_file: &str) -> Vec<CoreInfo> {
        CoreInfoHelper::get_compatibility_core_infos(&rom_file.into(), &self.retro_paths)
    }

    pub fn identifier_rom_file(
        &self,
        rom_file: &str,
        cores: &Vec<CoreInfo>,
    ) -> Result<Option<GameInfo>, ErrorHandle> {
        for core in cores {
            let games_data =
                DatabaseHelper::identifier_rom_file(rom_file, &core, &self.retro_paths.databases)?;

            if let Some(game) = games_data {
                return Ok(Some(game));
            }
        }

        Ok(None)
    }
}
