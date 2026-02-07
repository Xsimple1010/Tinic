use crate::app::GameInstance;
use crate::app::listener::WindowListener;
use crate::app_dispatcher::GameInstanceActions;
use crate::device_listener::DeviceHandle;
use crate::{
    GameInstanceDispatchers,
    generics::error_handle::ErrorHandle,
    retro_controllers::{RetroController, devices_manager::DeviceListener},
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

pub enum TinicGameInstanceStatus {
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

    #[doc = "
        # Set Control Listener

        Sets the control listener used to handle control events.

        The control listener must implement the **DeviceListener** trait.

        * **Warning:** The control listener must be set **before creating a `GameInstance`**.
    "]
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

    #[doc = "
        # Set Window Listener

        Sets the window listener used to handle window events.
        The window listener must implement the **WindowListener** trait.

        * **Warning**: The window listener must be set **before creating a `GameInstance`**.
    "]
    pub fn set_window_listener(&mut self, listener: Box<dyn WindowListener>) {
        self.window_listener.replace(Arc::new(listener));
    }

    #[doc = "
        # Get Game Dispatchers

        Retrieves the game dispatchers responsible for handling game events.
        The returned dispatchers must be used to send events to the **GameInstance**.
    "]
    pub fn get_game_dispatchers(&self) -> GameInstanceDispatchers {
        self.game_dispatchers.clone()
    }

    #[doc = "
        # Create Game Instance

        Creates a **GameInstance** responsible for running the game.

        Use the **Tinic::get_game_dispatchers()** function to retrieve the game dispatchers,
        which are used to handle game events.
    "]
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

    #[doc = "
        # Run

        Consumes the **GameInstance** and runs the game

        Please note that this function **blocks the current thread** until the game finishes

        * **Warning**: You **cannot create another** **GameInstance** after calling this function.
        If you attempt to create a new `GameInstance`, it will return an error.
    "]
    pub fn run(&mut self, mut game_instance: GameInstance) -> Result<(), ErrorHandle> {
        let event_loop = match self.event_loop.take() {
            Some(event_loop) => event_loop,
            None => return Ok(()),
        };

        event_loop.run_app(&mut game_instance).map_err(|e| {
            let erro_message = format!("Error on Tinic::Run -> {e}");
            ErrorHandle::new(&erro_message)
        })
    }

    #[doc = "
        # Pop Event

        This function is useful if you need to run the `GameInstance` inside your own loop.
        You only need to call it on each iteration of the loop.
        Use the returned **`TinicGameInstanceStatus`** to determine whether the `GameInstance`
        window has been closed.

        * **Warning:** Like the `run` function, after calling this function you will no longer be
        able to create a new `GameInstance`.
        However, **unlike `run`, this function does not block the main thread**.
    "]
    pub fn pop_event(&mut self, game_instance: &mut GameInstance) -> TinicGameInstanceStatus {
        let event_loop = match self.event_loop.as_mut() {
            Some(event_loop) => event_loop,
            None => return TinicGameInstanceStatus::Exit(0),
        };

        match event_loop.pump_app_events(None, game_instance) {
            PumpStatus::Exit(code) => TinicGameInstanceStatus::Exit(code),
            PumpStatus::Continue => TinicGameInstanceStatus::Continue,
        }
    }

    #[doc = "
        # Run On Demand

        This function allows you to create multiple `GameInstance`s.
        However, the main thread will be **blocked** until the window is closed.
        Use the returned **`TinicGameInstanceStatus`** to determine whether the window has been closed.
    "]
    pub fn run_app_on_demand(
        &mut self,
        mut game_instance: GameInstance,
    ) -> TinicGameInstanceStatus {
        let event_loop = match self.event_loop.as_mut() {
            Some(event_loop) => event_loop,
            None => return TinicGameInstanceStatus::Exit(0),
        };

        match event_loop.run_app_on_demand(&mut game_instance) {
            Ok(()) => TinicGameInstanceStatus::Continue,
            Err(_e) => TinicGameInstanceStatus::Exit(1),
        }
    }
}
