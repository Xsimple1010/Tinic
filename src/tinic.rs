use crate::game_thread::GameThread;
use crate::retro_stack::{RetroStack, StackCommand};
use retro_ab::erro_handle::ErroHandle;
use retro_ab::paths::Paths;
use retro_ab_gamepad::devices_manager::{Device, DeviceState, DeviceStateListener};
use retro_ab_gamepad::RetroAbController;
use std::ptr::addr_of;
use std::sync::{Arc, Mutex};

static mut CONTROLLER_STATE_LISTENER: Option<DeviceStateListener> = None;
lazy_static! {
    static ref STACK: Arc<RetroStack> = RetroStack::new();
}

fn device_state_listener(state: DeviceState, device: Device) {
    unsafe {
        if let Some(listener) = &*addr_of!(CONTROLLER_STATE_LISTENER) {
            match &state {
                DeviceState::Connected | DeviceState::Disconnected => {
                    STACK.push(StackCommand::GamepadConnected(device.clone()));
                }
                _ => {}
            }
            listener(state, device);
        };
    }
}

pub struct Tinic {
    pub retro_ab_controller: Arc<Mutex<RetroAbController>>,
    game_thread: GameThread,
}

impl Drop for Tinic {
    fn drop(&mut self) {
        STACK.push(StackCommand::Quit);
    }
}

impl Tinic {
    pub fn new(listener: Option<DeviceStateListener>) -> Result<Tinic, ErroHandle> {
        unsafe {
            CONTROLLER_STATE_LISTENER = listener;
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
        core_path: String,
        rom_path: String,
        paths: Paths,
    ) -> Result<(), ErroHandle> {
        self.game_thread.start(core_path, rom_path, paths)
    }

    pub fn pause(&self) {
        STACK.push(StackCommand::Pause);
    }

    pub fn resume(&self) {
        STACK.push(StackCommand::Resume);
    }

    pub fn save_state(&self) {
        STACK.push(StackCommand::SaveState);
    }

    pub fn load_state(&self) {
        STACK.push(StackCommand::LoadState);
    }

    pub fn connect_gamepad(device: Device) {
        STACK.push(StackCommand::GamepadConnected(device));
    }

    pub fn reset(&self) {
        STACK.push(StackCommand::Reset);
    }

    pub fn quit(&self) {
        self.game_thread.stop();
    }
}
