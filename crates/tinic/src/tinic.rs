use crate::{
    generics::erro_handle::ErroHandle,
    retro_controllers::{devices_manager::DeviceListener, RetroController},
    retro_core::{option_manager::OptionManager, test_tools},
    tinic_app::TinicApp,
};
use std::sync::Arc;
use winit::event_loop::EventLoop;

pub struct Tinic {
    pub controller: Arc<RetroController>,
    pub options: Arc<Option<OptionManager>>,
}

impl Tinic {
    pub fn new(listener: Box<dyn DeviceListener>) -> Result<Tinic, ErroHandle> {
        let controller = Arc::new(RetroController::new(listener)?);

        Ok(Self {
            controller,
            options: Arc::new(None),
        })
    }

    pub fn make_context(
        &self,
        core_path: &String,
        rom_path: &String,
    ) -> Result<TinicApp, ErroHandle> {
        let retro_path = test_tools::paths::get_paths().unwrap();

        let app = TinicApp::new(
            retro_path,
            core_path.clone(),
            rom_path.clone(),
            self.controller.clone(),
        );

        Ok(app)
    }

    pub fn run(&mut self, mut ctx: TinicApp) -> Result<(), ErroHandle> {
        let event_loop = EventLoop::new().unwrap();

        event_loop.run_app(&mut ctx).unwrap();

        Ok(())
    }
}
