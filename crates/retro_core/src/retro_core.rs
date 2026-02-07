use crate::{
    core::{CoreWrapperIns, RetroEnvCallbacks},
    graphic_api::GraphicApi,
    retro_context::{RetroContext, RetroCtxIns},
};
use generics::erro_handle::ErroHandle;
use generics::retro_paths::RetroPaths;
use libretro_sys::binding_libretro::retro_hw_context_type;

pub struct RetroCore {
    retro_ctx: RetroCtxIns,
}

impl Drop for RetroCore {
    fn drop(&mut self) {
        let _ = self.retro_ctx.delete();
    }
}

impl RetroCore {
    pub fn new(
        core_path: &str,
        paths: RetroPaths,
        callbacks: RetroEnvCallbacks,
        hw_type: retro_hw_context_type,
    ) -> Result<Self, ErroHandle> {
        Ok(RetroCore {
            retro_ctx: RetroContext::new(core_path, paths, callbacks, GraphicApi::with(hw_type))?,
        })
    }

    pub fn core(&self) -> CoreWrapperIns {
        self.retro_ctx.core.clone()
    }
}
