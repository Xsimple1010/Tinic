use crate::art::helper::ArtHelper;
use crate::cores::CoreHelper;
use crate::event::TinicSuperEventListener;
use crate::infos::helper::InfoHelper;
use crate::rdb_manager::helper::RdbManager;
use generics::retro_paths::RetroPaths;
use std::sync::Arc;

pub struct TinicSuper {
    pub retro_paths: RetroPaths,
    pub art_helper: ArtHelper,
    pub info_helper: InfoHelper,
    pub rdb_helper: RdbManager,
    pub core_helper: CoreHelper,
}

impl TinicSuper {
    pub fn new(retro_paths: RetroPaths, event_listener: Arc<dyn TinicSuperEventListener>) -> Self {
        Self {
            art_helper: ArtHelper {
                event_listener: event_listener.clone(),
                retro_paths: retro_paths.clone(),
            },
            info_helper: InfoHelper {
                event_listener: event_listener.clone(),
                retro_paths: retro_paths.clone(),
            },
            rdb_helper: RdbManager {
                retro_path: retro_paths.clone(),
                event_listener: event_listener.clone(),
            },
            core_helper: CoreHelper {
                retro_paths: retro_paths.clone(),
                event_listener: event_listener.clone(),
            },
            retro_paths,
        }
    }
}
