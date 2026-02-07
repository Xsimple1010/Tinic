use crate::{rdb_manager::game_model::GameInfo, tools::download::DownloadProgress};

pub trait TinicSuperEventListener: Send + Sync {
    fn downloading(&self, progress: DownloadProgress);
    fn extract_file(&self, file_name: String);
    fn rdb_read(&self, game_info: Vec<GameInfo>);
    fn core_installed(&self, core_name: String);
}
