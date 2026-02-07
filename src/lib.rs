#[macro_use]
extern crate lazy_static;
extern crate retro_ab;
extern crate retro_ab_av;
extern crate retro_ab_gamepad;

mod game_loop;
mod game_window_handle;
mod retro_stack;
mod stack_handle;

use game_loop::GameThread;
use retro_ab_gamepad::context::GamepadContext;
use retro_stack::{RetroStack, StackCommand};
use std::ptr::addr_of;
use std::sync::{Arc, Mutex};

pub use retro_ab::args_manager;
pub use retro_ab::paths::Paths;
pub use retro_ab::test_tools;
pub use retro_ab_gamepad::retro_gamepad::RetroGamePad;
pub use retro_ab_gamepad::{GamePadState, GamepadStateListener};

static mut GAMEPAD_LISTENER: Option<GamepadStateListener> = None;
lazy_static! {
    static ref STACK: Arc<RetroStack> = RetroStack::new();
}

fn gamepad_state_listener(state: GamePadState, _gamepad: RetroGamePad) {
    unsafe {
        if let Some(listener) = &*addr_of!(GAMEPAD_LISTENER) {
            match &state {
                GamePadState::Connected | GamePadState::Disconnected => {
                    STACK.push(StackCommand::GamepadConnected(_gamepad.clone()));
                }
                _ => {}
            }
            listener(state, _gamepad);
        };
    }
}

pub struct Tinic {
    pub controller_ctx: Arc<Mutex<GamepadContext>>,
    game_thread: GameThread,
}

impl Drop for Tinic {
    fn drop(&mut self) {
        STACK.push(StackCommand::Quit);
    }
}

impl Tinic {
    pub fn new(listener: Option<GamepadStateListener>) -> Tinic {
        unsafe {
            GAMEPAD_LISTENER = listener;
        }

        let controller_ctx = Arc::new(Mutex::new(GamepadContext::new(Some(
            gamepad_state_listener,
        ))));

        Self {
            //TODO:o numero máximo de portas deve ser alterado no futuro
            game_thread: GameThread::new(controller_ctx.clone(), STACK.clone()),
            controller_ctx,
        }
    }

    pub fn load_core(
        &mut self,
        core_path: String,
        rom_path: String,
        paths: Paths,
    ) -> Result<(), String> {
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

    pub fn reset(&self) {
        STACK.push(StackCommand::Reset);
    }

    pub fn quit(&self) {
        self.game_thread.stop();
    }
}
