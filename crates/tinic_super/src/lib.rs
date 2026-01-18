extern crate reqwest;
extern crate zip;

pub mod art;
pub mod core_info;
pub mod event;
pub mod rdb_manager;
pub mod tinic_super;
mod tools;

pub use generics::{error_handle::ErrorHandle, retro_paths::RetroPaths};
pub use rdb_manager::game_identifier::GameIdentifier;
pub use tools::download::FileProgress;

#[cfg(test)]
mod test {
    use crate::{
        event::TinicSuperEventListener, rdb_manager::game_model::GameInfo, tinic_super::TinicSuper,
    };
    use generics::retro_paths::RetroPaths;
    use std::sync::Arc;

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

    fn setup(base_path: &str) -> TinicSuper {
        let retro_paths = RetroPaths::from_base(base_path).unwrap();
        TinicSuper::new(retro_paths, Arc::new(TinicSuperListener))
    }

    #[tokio::test]
    async fn install_core() {
        let work_dir = create_work_dir_path("tinic_super..install_core");
        let tinic_super = setup(&work_dir);
        tokio::fs::remove_dir_all(&work_dir).await.unwrap();

        tinic_super
            .core_info_helper
            .try_update_core_infos(false)
            .await
            .unwrap();

        tokio::fs::remove_dir_all(work_dir).await.unwrap();
    }
}
