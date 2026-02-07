use crate::game_thread::game_thread::GameThread;
use crate::thread_stack::game_stack::{GameStack, GameStackCommand};
use crate::thread_stack::model_stack::RetroStackFn;
use retro_ab::erro_handle::ErroHandle;
use retro_ab::paths::Paths;
use retro_ab::retro_sys::retro_log_level;
use retro_ab_gamepad::devices_manager::{Device, DeviceState, DeviceStateListener};
use retro_ab_gamepad::RetroAbController;
use std::sync::{Arc, Mutex, RwLock};

static DEVICE_STATE_LISTENER: RwLock<Option<DeviceStateListener>> = RwLock::new(None);

lazy_static! {
    static ref STACK: Arc<GameStack> = Arc::new(GameStack::new());
}

fn device_state_listener(state: DeviceState, device: Device) {
    if let Some(listener) = DEVICE_STATE_LISTENER.read().unwrap().as_ref() {
        match &state {
            DeviceState::Connected | DeviceState::Disconnected => {
                STACK.push(GameStackCommand::DeviceConnected(device.clone()));
            }
            _ => {}
        }
        listener(state, device);
    };
}

pub struct Tinic {
    pub retro_ab_controller: Arc<Mutex<RetroAbController>>,
    game_thread: GameThread,
}

impl Drop for Tinic {
    fn drop(&mut self) {
        STACK.push(GameStackCommand::Quit);
    }
}

impl Tinic {
    //noinspection RsPlaceExpression
    pub fn new(listener: Option<DeviceStateListener>) -> Result<Tinic, ErroHandle> {
        match DEVICE_STATE_LISTENER.write() {
            Ok(mut device_listener) => *device_listener = listener,
            Err(e) => {
                return Err(ErroHandle {
                    level: retro_log_level::RETRO_LOG_ERROR,
                    message: e.to_string(),
                })
            }
        }

        let retro_ab_controller = Arc::new(Mutex::new(RetroAbController::new(Some(
            device_state_listener,
        ))?));

        Ok(Self {
            game_thread: GameThread::new(retro_ab_controller.clone(), STACK.clone()),
            retro_ab_controller,
        })
    }

    pub fn load_core(
        &mut self,
        core_path: &str,
        rom_path: &str,
        paths: Paths,
    ) -> Result<(), ErroHandle> {
        self.game_thread.start(core_path, rom_path, paths)
    }

    pub fn pause(&self) {
        STACK.push(GameStackCommand::Pause);
    }

    pub fn resume(&self) {
        STACK.push(GameStackCommand::Resume);
    }

    pub fn save_state(&self, slot: usize) {
        STACK.push(GameStackCommand::SaveState(slot));
    }

    pub fn load_state(&self, slot: usize) {
        STACK.push(GameStackCommand::LoadState(slot));
    }

    pub fn connect_device(device: Device) {
        STACK.push(GameStackCommand::DeviceConnected(device));
    }

    pub fn reset(&self) {
        STACK.push(GameStackCommand::Reset);
    }

    pub fn quit(&self) {
        self.game_thread.stop();
    }
}
