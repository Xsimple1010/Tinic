use crate::rdb_manager::game::GameInfo;

pub trait TinicSuperEventListener: Send + Sync {
    fn downloading(&self, file_name: String, percent: f32);
    fn extract_file(&self, file_name: String);
    fn download_completed(&self, file_name: String);
    fn rdb_read(&self, game_info: Vec<GameInfo>);
}
