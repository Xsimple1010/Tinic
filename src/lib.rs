#[macro_use]
extern crate lazy_static;
extern crate retro_ab;
extern crate retro_ab_av;
extern crate retro_ab_gamepad;

mod game_loop;
mod retro_stack;

use game_loop::init_game_loop;
use retro_ab::core::{self, RetroContext, RetroEnvCallbacks};
use retro_ab::retro_sys::retro_rumble_effect;
use retro_ab_av::{audio_sample_batch_callback, audio_sample_callback, video_refresh_callback};
use retro_ab_gamepad::context::{input_poll_callback, input_state_callback, GamepadContext};
use retro_stack::{RetroStack, StackCommand};
use std::sync::Arc;

pub use retro_ab::args_manager;
pub use retro_ab::paths::Paths;
pub use retro_ab::test_tools;
pub use retro_ab_gamepad::retro_gamepad::RetroGamePad;
pub use retro_ab_gamepad::{GamePadState, GamepadStateListener};

static mut GAMEPAD_LISTENER: Option<GamepadStateListener> = None;
lazy_static! {
    static ref STACK: Arc<RetroStack> = RetroStack::new();
}

pub struct Tinic {
    // stack: Arc<RetroStack>,
    pub controller_ctx: GamepadContext,
    pub core_ctx: Option<Arc<RetroContext>>,
}

fn rumble_callback(
    _port: ::std::os::raw::c_uint,
    _effect: retro_rumble_effect,
    _strength: u16,
) -> bool {
    true
}

fn gamepad_state_listener(state: GamePadState, _gamepad: RetroGamePad) {
    unsafe {
        if let Some(listener) = &GAMEPAD_LISTENER {
            match &state {
                GamePadState::Connected | GamePadState::Disconnected => {
                    STACK.push(StackCommand::UpdateControllers);
                }
                _ => {}
            }
            listener(state, _gamepad);
        };
    }
}

impl Tinic {
    pub fn new(listener: Option<GamepadStateListener>) -> Tinic {
        unsafe {
            GAMEPAD_LISTENER = listener;
        }
        Self {
            // stack: RetroStack::new(),
            //TODO:o numero máximo de portas deve ser alterado no futuro
            controller_ctx: GamepadContext::new(Some(gamepad_state_listener)),
            core_ctx: None,
        }
    }

    pub fn load_core(&mut self, core_path: &String, paths: Paths) -> Result<(), String> {
        let core_ctx = core::load(
            core_path,
            paths,
            RetroEnvCallbacks {
                audio_sample_batch_callback,
                audio_sample_callback,
                input_poll_callback,
                input_state_callback,
                video_refresh_callback,
                rumble_callback,
            },
        )?;

        match core::init(&core_ctx) {
            Ok(..) => {
                self.core_ctx = Some(core_ctx.clone());

                let gamepads = self.controller_ctx.get_list();

                init_game_loop(core_ctx, gamepads, STACK.clone());
            }
            Err(e) => {
                return Err(e.message);
            }
        }

        Ok(())
    }

    pub fn load_rom(&self, path: String) {
        STACK.push(StackCommand::LoadGame(path))
    }

    pub fn unload_rom(&self) {
        STACK.push(StackCommand::UnloadGame)
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

    pub fn quit_game(&self) {
        STACK.push(StackCommand::GameQuit);
    }

    // pub fn unload_game(&self) {
    //     STACK.push(StackCommand::UnloadGame);
    // }

    // pub fn load_game(&self) {
    //     STACK.push(StackCommand::LoadGame);
    // }

    pub fn change_controller_pending(&self) {
        // STACK.push(StackCommand::UpdateControllers);
    }
}
