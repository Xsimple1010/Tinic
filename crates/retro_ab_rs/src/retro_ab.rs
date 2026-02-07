use crate::{
    core::{CoreWrapperIns, RetroEnvCallbacks},
    erro_handle::ErroHandle,
    graphic_api::GraphicApi,
    paths::Paths,
    retro_context::{RetroContext, RetroCtxIns},
    retro_sys::retro_hw_context_type,
};

pub struct RetroAB {
    retro_ctx: RetroCtxIns,
}

impl Drop for RetroAB {
    fn drop(&mut self) {
        let _ = self.retro_ctx.delete();
    }
}

impl RetroAB {
    pub fn new(
        core_path: &str,
        paths: Paths,
        callbacks: RetroEnvCallbacks,
        hw_type: retro_hw_context_type,
    ) -> Result<Self, ErroHandle> {
        Ok(RetroAB {
            retro_ctx: RetroContext::new(core_path, paths, callbacks, GraphicApi::with(hw_type))?,
        })
    }

    pub fn core(&self) -> CoreWrapperIns {
        self.retro_ctx.core.clone()
    }
}
