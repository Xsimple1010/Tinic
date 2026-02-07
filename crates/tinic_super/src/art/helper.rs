use std::{fmt::Display, sync::Arc};

use generics::{error_handle::ErrorHandle, retro_paths::RetroPaths};

use crate::{
    art::{download_all_thumbnail_from_game, thumbnail::Thumbnails},
    event::TinicSuperEventListener,
};

pub struct ArtHelper {
    pub(crate) event_listener: Arc<dyn TinicSuperEventListener>,
    pub(crate) retro_paths: RetroPaths,
}

impl ArtHelper {
    pub fn new(retro_paths: RetroPaths, event_listener: Arc<dyn TinicSuperEventListener>) -> Self {
        ArtHelper {
            retro_paths,
            event_listener,
        }
    }

    pub async fn get_urls(
        &self,
        manufacturer_name: &impl Display,
        sys_name: &impl Display,
        name: &impl Display,
    ) -> Result<Thumbnails, ErrorHandle> {
        download_all_thumbnail_from_game(
            manufacturer_name,
            sys_name,
            name,
            &self.retro_paths.arts.to_string(),
            self.event_listener.clone(),
        )
        .await
    }
}
