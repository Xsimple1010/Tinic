use crate::{
    art::helper::ThumbnailEventType, cores::CoreEventType, infos::helper::InfoEventType,
    rdb_manager::helper::RdbEventType,
};

pub trait TinicSuperEventListener: Send + Sync {
    // fn downloading(&self, progress: DownloadProgress);
    // fn extract_file(&self, file_name: String);
    // fn core_installed(&self, core_name: String);

    fn on_thumbnail_evnt(&self, event: ThumbnailEventType);
    fn on_info_event(&self, event: InfoEventType);
    fn on_core_event(&self, event: CoreEventType);
    fn on_rdb_event(&self, state: RdbEventType);
}
