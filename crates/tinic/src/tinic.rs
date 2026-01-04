use crate::app_dispatcher::GameInstanceActions;
use crate::device_listener::DeviceHandle;
use crate::{
    generics::error_handle::ErrorHandle,
    retro_controllers::{devices_manager::DeviceListener, RetroController},
    tinic_app::GameInstance,
    tinic_app_ctx::TinicGameCtx,
};
use generics::{retro_paths::RetroPaths, types::TMutex};
use std::sync::Arc;
use winit::event_loop::EventLoopProxy;
use winit::platform::run_on_demand::EventLoopExtRunOnDemand;
use winit::{
    event_loop::EventLoop,
    platform::pump_events::{EventLoopExtPumpEvents, PumpStatus},
};

pub struct Tinic {
    pub controller: Arc<RetroController>,
    event_loop: Option<EventLoop<GameInstanceActions>>,
    proxy: EventLoopProxy<GameInstanceActions>,
}

pub enum TinicPumpStatus {
    Continue,
    Exit(i32),
}

#[derive(Clone)]
pub struct TinicGameInfo {
    pub core: String,
    pub rom: String,
    pub sys_dir: String,
}

impl Tinic {
    pub fn new(listener: Box<dyn DeviceListener>) -> Result<Tinic, ErrorHandle> {
        let dispatcher = TMutex::new(None);

        let devices_listener = DeviceHandle {
            extern_listener: listener,
            app_proxy: dispatcher.clone(),
        };
        let controller = Arc::new(RetroController::new(Box::new(devices_listener))?);

        let event_loop = EventLoop::<GameInstanceActions>::with_user_event()
            .build()
            .unwrap();

        let proxy = event_loop.create_proxy();

        Ok(Self {
            controller,
            proxy,
            event_loop: Some(event_loop),
        })
    }

    pub fn create_game_instance(
        &mut self,
        game_info: TinicGameInfo,
    ) -> Result<GameInstance, ErrorHandle> {
        let ctx = TinicGameCtx::new(
            RetroPaths::from_base(game_info.sys_dir).unwrap(),
            game_info.core,
            game_info.rom,
            self.controller.clone(),
        )?;

        let game_instance = GameInstance::new(ctx, self.proxy.clone());

        Ok(game_instance)
    }

    pub fn run(&mut self, mut game_instance: GameInstance) -> Result<(), ErrorHandle> {
        if let Some(event_loop) = self.event_loop.take() {
            event_loop.run_app(&mut game_instance).unwrap();
        }
        Ok(())
    }

    pub fn pop_event(&mut self, game_instance: &mut GameInstance) -> TinicPumpStatus {
        if let Some(event_loop) = self.event_loop.as_mut() {
            match event_loop.pump_app_events(None, game_instance) {
                PumpStatus::Exit(code) => TinicPumpStatus::Exit(code),
                PumpStatus::Continue => TinicPumpStatus::Continue,
            }
        } else {
            TinicPumpStatus::Exit(0)
        }
    }

    pub fn run_app_on_demand(&mut self, mut game_instance: GameInstance) -> TinicPumpStatus {
        if let Some(event_loop) = self.event_loop.as_mut() {
            match event_loop.run_app_on_demand(&mut game_instance) {
                Ok(()) => TinicPumpStatus::Continue,
                Err(_e) => TinicPumpStatus::Exit(1),
            }
        } else {
            TinicPumpStatus::Exit(0)
        }
    }
}
