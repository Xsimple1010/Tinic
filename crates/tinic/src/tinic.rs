use crate::app::listener::WindowListener;
use crate::app::GameInstance;
use crate::app_dispatcher::GameInstanceActions;
use crate::device_listener::DeviceHandle;
use crate::{
    generics::error_handle::ErrorHandle,
    retro_controllers::{devices_manager::DeviceListener, RetroController},
    GameInstanceDispatchers,
};
use std::sync::Arc;
use winit::platform::run_on_demand::EventLoopExtRunOnDemand;
use winit::{
    event_loop::EventLoop,
    platform::pump_events::{EventLoopExtPumpEvents, PumpStatus},
};

pub struct Tinic {
    pub retro_controle: Option<Arc<RetroController>>,
    event_loop: Option<EventLoop<GameInstanceActions>>,
    game_dispatchers: GameInstanceDispatchers,
    window_listener: Option<Arc<Box<dyn WindowListener>>>,
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
    pub fn new() -> Result<Tinic, ErrorHandle> {
        let event_loop = EventLoop::<GameInstanceActions>::with_user_event()
            .build()
            .unwrap();
        let game_dispatchers = GameInstanceDispatchers::new(event_loop.create_proxy());

        Ok(Self {
            game_dispatchers,
            retro_controle: None,
            event_loop: Some(event_loop),
            window_listener: None,
        })
    }

    pub fn set_controle_listener(
        &mut self,
        listener: Box<dyn DeviceListener>,
    ) -> Result<(), ErrorHandle> {
        let devices_listener = DeviceHandle {
            extern_listener: listener,
            game_dispatchers: self.game_dispatchers.clone(),
        };

        let retro_controle = RetroController::new(Box::new(devices_listener))?;

        self.retro_controle.replace(Arc::new(retro_controle));
        Ok(())
    }

    pub fn set_window_listener(&mut self, listener: Box<dyn WindowListener>) {
        self.window_listener.replace(Arc::new(listener));
    }

    pub fn get_game_dispatchers(&self) -> GameInstanceDispatchers {
        self.game_dispatchers.clone()
    }

    pub fn create_game_instance(
        &mut self,
        game_info: TinicGameInfo,
    ) -> Result<GameInstance, ErrorHandle> {
        let retro_controle = match &self.retro_controle {
            Some(re) => re.clone(),
            None => {
                return Err(ErrorHandle::new(
                    "To create a game_instance first create a controle listener with Tinic::set_controle_listener()",
                ));
            }
        };

        let window_listener = match &self.window_listener {
            Some(re) => re.clone(),
            None => {
                return Err(ErrorHandle::new(
                    "To create a game_instance first create a controle listener with Tinic::set_controle_listener()",
                ));
            }
        };

        let game_instance = GameInstance::new(
            game_info,
            retro_controle,
            window_listener,
            self.game_dispatchers.clone(),
        )?;

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
