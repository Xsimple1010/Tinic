extern crate reqwest;
extern crate zip;

pub mod art;
pub mod cores;
pub mod event;
pub mod infos;
pub mod rdb_manager;
pub mod tinic_super;
mod tools;

pub use generics::{error_handle::ErrorHandle, retro_paths::RetroPaths};
pub use rdb_manager::game_identifier::GameIdentifier;
pub use tools::download::FileProgress;

#[cfg(test)]
mod test {
    use crate::{
        event::TinicSuperEventListener, infos::model::CoreInfo, rdb_manager::game_model::GameInfo,
        tinic_super::TinicSuper,
    };
    use generics::retro_paths::RetroPaths;
    use std::{collections::HashSet, path::PathBuf, sync::Arc};

    struct TinicSuperListener;

    impl TinicSuperEventListener for TinicSuperListener {
        fn downloading(&self, file_name: String, percent: f32) {
            println!("{file_name}: {percent}%")
        }

        fn extract_file(&self, file_name: String) {
            println!("extracting: {file_name}")
        }

        fn download_completed(&self, file_name: String) {
            println!("{file_name} downloaded")
        }

        fn rdb_read(&self, game_info: Vec<GameInfo>) {
            println!("{game_info:?}")
        }
    }

    fn create_work_dir_path(test_dir: &str) -> String {
        format!("tinic_test_workspace/{test_dir}")
    }

    async fn setup(base_path: &str) -> (TinicSuper, String) {
        let work_dir = create_work_dir_path(base_path);
        tokio::fs::remove_dir_all(&work_dir).await.unwrap();
        let retro_paths = RetroPaths::from_base(&work_dir).unwrap();
        (
            TinicSuper::new(retro_paths, Arc::new(TinicSuperListener)),
            work_dir,
        )
    }

    async fn clean_up(work_dir: &String) {
        tokio::fs::remove_dir_all(work_dir).await.unwrap();
    }

    #[tokio::test]
    async fn info_helper() {
        let (tinic_super, work_dir) = setup("tinic_super..install_core").await;

        tinic_super
            .core_info_helper
            .download_blocking(false)
            .await
            .unwrap();

        // vai ser usado para o read file
        let info: Option<CoreInfo>;

        // get_infos
        {
            let infos = tinic_super.core_info_helper.get_infos().await;
            assert_eq!(infos.len(), 294);
            info = infos
                .into_iter()
                .find(|info| info.file_name == "snes9x_libretro");
        }

        // read_file
        {
            let info = info.as_ref().expect("core info not found");
            let path = info.path.clone();

            let new_info = tinic_super
                .core_info_helper
                .read_file(&path)
                .await
                .expect("info não foi encontrada verifique o caminho do arquivo");

            assert_eq!(new_info.file_name, info.file_name);
            assert_eq!(new_info.description, info.description);
            assert_eq!(new_info.database, info.database);
        }

        // get_compatibility_core_infos
        {
            let rom = PathBuf::from("./mario.smc");

            let infos = tinic_super
                .core_info_helper
                .get_compatibility_core_infos(&rom)
                .await;

            assert_eq!(infos.len(), 22);
        }

        clean_up(&work_dir).await;
    }

    #[tokio::test]
    async fn rdb_helper() {
        let (tinic_super, work_dir) = setup("tinic_super..download_required_files").await;

        tinic_super.rdb_helper.download(false).await.unwrap();

        // read_rdb
        {
            let rdb_names =
                HashSet::from(["snes9x_libretro".to_string(), "snes9x_libretro".to_string()]);

            tinic_super.rdb_helper.read_rdbs(rdb_names).await;
        }

        clean_up(&work_dir).await;
    }
}
