use crate::app_dispatcher::GameInstanceActions;
use crate::device_listener::DeviceHandle;
use crate::{
    generics::error_handle::ErrorHandle,
    retro_controllers::{devices_manager::DeviceListener, RetroController},
    tinic_app::GameInstance,
    tinic_app_ctx::TinicGameCtx,
    GameInstanceDispatchers,
};
use generics::retro_paths::RetroPaths;
use std::sync::Arc;
use winit::platform::run_on_demand::EventLoopExtRunOnDemand;
use winit::{
    event_loop::EventLoop,
    platform::pump_events::{EventLoopExtPumpEvents, PumpStatus},
};

pub struct Tinic {
    pub controller: Arc<RetroController>,
    event_loop: Option<EventLoop<GameInstanceActions>>,
    game_dispatchers: GameInstanceDispatchers,
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
        let event_loop = EventLoop::<GameInstanceActions>::with_user_event()
            .build()
            .unwrap();
        let game_dispatchers = GameInstanceDispatchers::new(event_loop.create_proxy());

        let devices_listener = DeviceHandle {
            extern_listener: listener,
            game_dispatchers: game_dispatchers.clone(),
        };
        let controller = Arc::new(RetroController::new(Box::new(devices_listener))?);

        Ok(Self {
            controller,
            game_dispatchers,
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

        let game_instance = GameInstance::new(ctx, self.game_dispatchers.clone());

        Ok(game_instance)
    }

    pub fn run(&mut self, mut game_instance: GameInstance) -> Result<(), ErrorHandle> {
        let event_loop = match self.event_loop.take() {
            Some(event_loop) => event_loop,
            None => return Ok(()),
        };

        event_loop.run_app(&mut game_instance).map_err(|e| {
            let erro_message = format!("Error on Tinic::Run -> {}", e.to_string());
            ErrorHandle::new(&erro_message)
        })
    }

    pub fn pop_event(&mut self, game_instance: &mut GameInstance) -> TinicPumpStatus {
        let event_loop = match self.event_loop.as_mut() {
            Some(event_loop) => event_loop,
            None => return TinicPumpStatus::Exit(0),
        };

        match event_loop.pump_app_events(None, game_instance) {
            PumpStatus::Exit(code) => TinicPumpStatus::Exit(code),
            PumpStatus::Continue => TinicPumpStatus::Continue,
        }
    }

    pub fn run_app_on_demand(&mut self, mut game_instance: GameInstance) -> TinicPumpStatus {
        let event_loop = match self.event_loop.as_mut() {
            Some(event_loop) => event_loop,
            None => return TinicPumpStatus::Exit(0),
        };

        match event_loop.run_app_on_demand(&mut game_instance) {
            Ok(()) => TinicPumpStatus::Continue,
            Err(_e) => TinicPumpStatus::Exit(1),
        }
    }
}
