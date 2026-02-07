use crate::core_info::helper::CoreInfoHelper;
use crate::event::TinicSuperEventListener;
use crate::rdb_manager::helper::RdbManager;
use generics::retro_paths::RetroPaths;
use std::sync::Arc;

pub struct TinicSuper {
    pub retro_paths: RetroPaths,
    pub core_info_helper: CoreInfoHelper,
    pub rdb_helper: RdbManager,
}

impl TinicSuper {
    pub fn new(retro_paths: RetroPaths, event_listener: Arc<dyn TinicSuperEventListener>) -> Self {
        Self {
            core_info_helper: CoreInfoHelper {
                event_listener: event_listener.clone(),
                retro_paths: retro_paths.clone(),
            },
            rdb_helper: RdbManager {
                retro_path: retro_paths.clone(),
                event_listener: event_listener.clone(),
            },
            retro_paths,
        }
    }
}
