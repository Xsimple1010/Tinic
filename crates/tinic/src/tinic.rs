use crate::{
    generics::erro_handle::ErroHandle,
    retro_controllers::{devices_manager::DeviceListener, RetroController},
    tinic_app::{TinicApp, TinicAppActions},
    tinic_app_ctx::TinicAppCtx,
};
use generics::{
    retro_paths::RetroPaths,
    types::{ArcTMuxte, TMutex},
};
use retro_controllers::devices_manager::Device;
use std::{path::PathBuf, sync::Arc};
use tinic_super::{core_info::CoreInfo, core_info_helper::CoreInfoHelper};
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

    pub fn build(
        &mut self,
        core_path: String,
        rom_path: String,
        retro_paths: RetroPaths,
    ) -> Result<TinicApp, ErroHandle> {
        let ctx = TinicAppCtx::new(retro_paths, core_path, rom_path, self.controller.clone())?;

        Ok(TinicApp::new(ctx))
    }

    pub fn run(&mut self, mut tinic_app: TinicApp) -> Result<(), ErroHandle> {
        let event_loop = EventLoop::<TinicAppActions>::with_user_event()
            .build()
            .unwrap();

        self.proxy.store(Some(event_loop.create_proxy()));
        event_loop.run_app(&mut tinic_app).unwrap();
        self.proxy.store(None);

        Ok(())
    }

    pub async fn try_update_core_infos(
        &mut self,
        force_update: bool,
        retro_paths: &RetroPaths,
    ) -> Result<(), ErroHandle> {
        match CoreInfoHelper::try_update_core_infos(retro_paths, force_update).await {
            Ok(_) => Ok(()),
            Err(e) => Err(ErroHandle {
                message: e.to_string(),
            }),
        }
    }

    pub fn get_cores_infos(&mut self, retro_paths: &RetroPaths) -> Vec<CoreInfo> {
        CoreInfoHelper::get_core_infos(&retro_paths.infos.clone().to_owned())
    }

    pub fn get_compatibility_info_cores(&self, rom: &String) -> Vec<CoreInfo> {
        CoreInfoHelper::get_compatibility_core_infos(PathBuf::from(rom))
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
