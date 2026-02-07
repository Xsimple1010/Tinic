use crate::core_info::helper::CoreInfoHelper;
use crate::event::TinicSuperEventListener;
use generics::retro_paths::RetroPaths;
use std::sync::Arc;

pub struct TinicSuper {
    pub retro_paths: RetroPaths,
    pub core_info_helper: CoreInfoHelper,
}

impl TinicSuper {
    pub fn new(retro_paths: RetroPaths, event_listener: Arc<dyn TinicSuperEventListener>) -> Self {
        Self {
            core_info_helper: CoreInfoHelper {
                event_listener: event_listener.clone(),
                retro_paths: retro_paths.clone(),
            },
            retro_paths,
        }
    }
}
