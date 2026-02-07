use crate::core::CoreWrapper;
use crate::{core::CoreWrapperIns, graphic_api::GraphicApi, RetroEnvCallbacks};
use generics::erro_handle::ErroHandle;
use generics::retro_paths::RetroPaths;
use libretro_sys::binding_libretro::retro_hw_context_type;
use std::sync::atomic::Ordering;

pub struct RetroCore {
    pub core: CoreWrapperIns,
}

impl Drop for RetroCore {
    fn drop(&mut self) {
        if self.core.initialized.load(Ordering::SeqCst) {
            let _ = self.core.de_init();
        }
    }
}

impl RetroCore {
    pub fn new(
        core_path: &str,
        paths: RetroPaths,
        callbacks: RetroEnvCallbacks,
        hw_type: retro_hw_context_type,
    ) -> Result<Self, ErroHandle> {
        let core = CoreWrapper::new(core_path, paths, callbacks, GraphicApi::with(hw_type))?;

        core.init()?;

        Ok(RetroCore { core })
    }
}
