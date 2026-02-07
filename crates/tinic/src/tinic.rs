use crate::{
    generics::error_handle::ErrorHandle,
    retro_controllers::{devices_manager::DeviceListener, RetroController},
    tinic_app::{GameInstanceActions, TinicGameInstance},
    tinic_app_ctx::TinicGameCtx,
};
use generics::{
    retro_paths::RetroPaths,
    types::{ArcTMutex, TMutex},
};
use retro_controllers::devices_manager::Device;
use std::{path::PathBuf, sync::Arc};
use tinic_super::{core_info::CoreInfo, core_info_helper::CoreInfoHelper};
use winit::{
    event_loop::{EventLoop, EventLoopProxy},
    platform::pump_events::{EventLoopExtPumpEvents, PumpStatus},
};

pub struct Tinic {
    pub controller: Arc<RetroController>,
    proxy: ArcTMutex<Option<EventLoopProxy<GameInstanceActions>>>,
    event_loop: Option<EventLoop<GameInstanceActions>>,
}

impl Tinic {
    pub fn new(listener: Box<dyn DeviceListener>) -> Result<Tinic, ErrorHandle> {
        let proxy = TMutex::new(None);

        let tinic_listener = DeviceHandle {
            listener,
            proxy: proxy.clone(),
        };
        let controller = Arc::new(RetroController::new(Box::new(tinic_listener))?);

        Ok(Self {
            controller,
            proxy,
            event_loop: None,
        })
    }

    pub fn build(
        &mut self,
        core_path: String,
        rom_path: String,
        retro_paths: RetroPaths,
    ) -> Result<TinicGameInstance, ErrorHandle> {
        let ctx = TinicGameCtx::new(retro_paths, core_path, rom_path, self.controller.clone())?;

        let (game_instance, event_loop) = TinicGameInstance::new(ctx);

        self.proxy.store(Some(event_loop.create_proxy()));
        self.event_loop.replace(event_loop);

        Ok(game_instance)
    }

    pub fn run(&mut self, mut game_instance: TinicGameInstance) -> Result<(), ErrorHandle> {
        if let Some(event_loop) = self.event_loop.take() {
            event_loop.run_app(&mut game_instance).unwrap();
        }

        Ok(())
    }

    pub fn pop_event(
        &mut self,
        game_instance: &mut TinicGameInstance,
    ) -> Result<PumpStatus, ErrorHandle> {
        if let Some(event_loop) = self.event_loop.as_mut() {
            Ok(event_loop.pump_app_events(None, game_instance))
        } else {
            Err(ErrorHandle {
                message: "".to_string(),
            })
        }
    }

    pub async fn try_update_core_infos(
        &mut self,
        force_update: bool,
        retro_paths: &RetroPaths,
    ) -> Result<(), ErrorHandle> {
        match CoreInfoHelper::try_update_core_infos(retro_paths, force_update).await {
            Ok(_) => Ok(()),
            Err(e) => Err(ErrorHandle {
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
    proxy: ArcTMutex<Option<EventLoopProxy<GameInstanceActions>>>,
}

impl DeviceListener for DeviceHandle {
    fn connected(&self, device: Device) {
        let mut invalid_proxy = false;

        if let Some(proxy) = self.proxy.load_or(None).as_ref() {
            if let Err(_) = proxy.send_event(GameInstanceActions::ConnectDevice(device.clone())) {
                invalid_proxy = true;
            }
        }

        if invalid_proxy {
            self.proxy.store(None);
        }

        self.listener.connected(device);
    }

    fn disconnected(&self, device: Device) {
        self.listener.disconnected(device);
    }

    fn button_pressed(&self, button: String, device: Device) {
        self.listener.button_pressed(button, device);
    }
}
