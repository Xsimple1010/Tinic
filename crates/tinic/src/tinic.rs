use crate::{
    generics::erro_handle::ErroHandle,
    retro_controllers::{devices_manager::DeviceListener, RetroController},
    tinic_app::{TinicApp, TinicAppActions},
};
use generics::{
    retro_paths::RetroPaths,
    types::{ArcTMuxte, TMutex},
};
use retro_controllers::devices_manager::Device;
use std::sync::Arc;
use winit::event_loop::{EventLoop, EventLoopProxy};

pub struct Tinic {
    pub controller: Arc<RetroController>,
    proxy: ArcTMuxte<Option<EventLoopProxy<TinicAppActions>>>,
}

impl Tinic {
    pub fn new(listener: Box<dyn DeviceListener>) -> Result<Tinic, ErroHandle> {
        let proxy = TMutex::new(None);

        let tinic_listener = DeviceHandle {
            listener,
            proxy: proxy.clone(),
        };
        let controller = Arc::new(RetroController::new(Box::new(tinic_listener))?);

        Ok(Self { controller, proxy })
    }

    pub fn make_context(
        &mut self,
        core_path: &String,
        rom_path: &String,
        retro_paths: RetroPaths,
    ) -> Result<TinicApp, ErroHandle> {
        let app = TinicApp::new(
            retro_paths,
            core_path.clone(),
            rom_path.clone(),
            self.controller.clone(),
        )?;

        Ok(app)
    }

    pub fn run(&mut self, mut ctx: TinicApp) -> Result<(), ErroHandle> {
        let event_loop = EventLoop::<TinicAppActions>::with_user_event()
            .build()
            .unwrap();

        self.proxy.store(Some(event_loop.create_proxy()));
        event_loop.run_app(&mut ctx).unwrap();
        self.proxy.store(None);

        Ok(())
    }
}

#[derive(Debug)]
struct DeviceHandle {
    listener: Box<dyn DeviceListener>,
    proxy: ArcTMuxte<Option<EventLoopProxy<TinicAppActions>>>,
}

impl DeviceListener for DeviceHandle {
    fn connected(&self, device: Device) {
        if let Some(proxy) = self.proxy.load_or(None).as_ref() {
            let _ = proxy.send_event(TinicAppActions::ConnectDevice(device.clone()));
        }

        self.listener.connected(device);
    }

    fn disconnected(&self, device: retro_controllers::devices_manager::Device) {
        self.listener.disconnected(device);
    }

    fn button_pressed(&self, button: String, device: retro_controllers::devices_manager::Device) {
        self.listener.button_pressed(button, device);
    }
}
