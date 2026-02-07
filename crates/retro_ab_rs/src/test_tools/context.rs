use crate::retro_context::RetroContext;
use crate::test_tools::{constants::CORE_TEST_RELATIVE_PATH, core, paths};
use generics::erro_handle::ErroHandle;
use std::sync::Arc;

pub fn get_context() -> Result<Arc<RetroContext>, ErroHandle> {
    RetroContext::new(
        CORE_TEST_RELATIVE_PATH,
        paths::get_paths()?,
        core::get_callbacks(),
        crate::graphic_api::GraphicApi::default(),
    )
}
