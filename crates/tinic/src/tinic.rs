use crate::app_dispatcher::{GameInstanceActions, GameInstanceDispatchers};
use crate::{
    generics::error_handle::ErrorHandle,
    retro_controllers::{RetroController, devices_manager::DeviceListener},
    tinic_app::GameInstance,
    tinic_app_ctx::TinicGameCtx,
};
use generics::{
    retro_paths::RetroPaths,
    types::{ArcTMutex, TMutex},
};
use retro_controllers::RetroGamePad;
use std::{path::PathBuf, sync::Arc};
use tinic_super::{core_info::CoreInfo, core_info_helper::CoreInfoHelper};
use winit::{
    event_loop::EventLoop,
    platform::pump_events::{EventLoopExtPumpEvents, PumpStatus},
};

pub struct Tinic {
    pub controller: Arc<RetroController>,
    dispatcher: ArcTMutex<Option<GameInstanceDispatchers>>,
    event_loop: Option<EventLoop<GameInstanceActions>>,
}

impl Tinic {
    pub fn new(listener: Box<dyn DeviceListener>) -> Result<Tinic, ErrorHandle> {
        let dispatcher = TMutex::new(None);

        let devices_listener = DeviceHandle {
            listener,
            dispatcher: dispatcher.clone(),
        };
        let controller = Arc::new(RetroController::new(Box::new(devices_listener))?);

        Ok(Self {
            controller,
            dispatcher,
            event_loop: None,
        })
    }

    pub fn build(
        &mut self,
        core_path: String,
        rom_path: String,
        retro_paths: RetroPaths,
    ) -> Result<GameInstance, ErrorHandle> {
        let ctx = TinicGameCtx::new(retro_paths, core_path, rom_path, self.controller.clone())?;

        let (game_instance, event_loop) = GameInstance::new(ctx);

        self.dispatcher
            .store(Some(game_instance.create_dispatcher()));
        self.event_loop.replace(event_loop);

        Ok(game_instance)
    }

    pub fn run(&mut self, mut game_instance: GameInstance) -> Result<(), ErrorHandle> {
        if let Some(event_loop) = self.event_loop.take() {
            event_loop.run_app(&mut game_instance).unwrap();
        }

        Ok(())
    }

    pub fn pop_event(
        &mut self,
        game_instance: &mut GameInstance,
    ) -> Result<PumpStatus, ErrorHandle> {
        if let Some(event_loop) = self.event_loop.as_mut() {
            Ok(event_loop.pump_app_events(None, game_instance))
        } else {
            Err(ErrorHandle::new(""))
        }
    }

    pub async fn try_update_core_infos(
        &mut self,
        force_update: bool,
        retro_paths: &RetroPaths,
    ) -> Result<(), ErrorHandle> {
        match CoreInfoHelper::try_update_core_infos(retro_paths, force_update).await {
            Ok(_) => Ok(()),
            Err(e) => Err(ErrorHandle::new(e.to_string().as_str())),
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
    dispatcher: ArcTMutex<Option<GameInstanceDispatchers>>,
}

impl DeviceListener for DeviceHandle {
    fn connected(&self, device: RetroGamePad) {
        let mut invalid_proxy = false;

        if let Some(dispatcher) = self.dispatcher.load_or(None).as_ref() {
            if dispatcher.disable_keybaord().is_err() {
                invalid_proxy = true;
            }

            if dispatcher.connect_device(device.clone()).is_err() {
                invalid_proxy = true;
            }
        }

        if invalid_proxy {
            self.dispatcher.store(None);
        }

        self.listener.connected(device);
    }

    fn disconnected(&self, device: RetroGamePad) {
        let mut invalid_proxy = false;

        if let Some(dispatcher) = self.dispatcher.load_or(None).as_ref() {
            if dispatcher.enable_keybaord().is_err() {
                invalid_proxy = true;
            }
        }

        if invalid_proxy {
            self.dispatcher.store(None);
        }

        self.listener.disconnected(device);
    }

    fn button_pressed(&self, button: String, device: RetroGamePad) {
        self.listener.button_pressed(button, device);
    }
}
