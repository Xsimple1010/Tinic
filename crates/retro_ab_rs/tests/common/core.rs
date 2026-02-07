use retro_ab::erro_handle::ErroHandle;
use retro_ab::retro_ab::RetroAB;
use retro_ab::retro_sys::retro_hw_context_type;
use retro_ab::test_tools;

pub fn setup() -> Result<RetroAB, ErroHandle> {
    RetroAB::new(
        test_tools::constants::CORE_TEST_RELATIVE_PATH,
        test_tools::paths::get_paths().unwrap(),
        test_tools::core::get_callbacks(),
        retro_hw_context_type::RETRO_HW_CONTEXT_NONE,
    )
}
