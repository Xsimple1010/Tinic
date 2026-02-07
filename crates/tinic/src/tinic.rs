use crate::app_dispatcher::{GameInstanceActions, GameInstanceDispatchers};
use crate::{
    generics::error_handle::ErrorHandle,
    retro_controllers::{devices_manager::DeviceListener, RetroController},
    tinic_app::GameInstance,
    tinic_app_ctx::TinicGameCtx,
};
use generics::{
    retro_paths::RetroPaths,
    types::{ArcTMutex, TMutex},
};
use std::sync::Arc;
use winit::{
    event_loop::EventLoop,
    platform::pump_events::{EventLoopExtPumpEvents, PumpStatus},
};
use crate::device_listener::DeviceHandle;

pub struct Tinic {
    pub controller: Arc<RetroController>,
    dispatcher: ArcTMutex<Option<GameInstanceDispatchers>>,
    event_loop: Option<EventLoop<GameInstanceActions>>,
}

pub enum TinicPumpStatus {
    Continue,
    Exit(i32),
}

impl Tinic {
    pub fn new(listener: Box<dyn DeviceListener>) -> Result<Tinic, ErrorHandle> {
        let dispatcher = TMutex::new(None);

        let devices_listener = DeviceHandle {
            extern_listener: listener,
            app_proxy: dispatcher.clone(),
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
    ) -> TinicPumpStatus {
        if let Some(event_loop) = self.event_loop.as_mut() {
            match event_loop.pump_app_events(None, game_instance) { 
                PumpStatus::Exit(code) => TinicPumpStatus::Exit(code),
                PumpStatus::Continue => TinicPumpStatus::Continue,
            }
        } else {
            TinicPumpStatus::Exit(0)
        }
    }
}
