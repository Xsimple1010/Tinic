#[macro_use]
extern crate lazy_static;
extern crate retro_ab;
extern crate retro_ab_av;
extern crate retro_ab_gamepad;

mod game_loop;
mod retro_stack;
mod stack_actions;

use game_loop::init_game_loop;
use retro_ab::core::{self, RetroContext, RetroEnvCallbacks};
use retro_ab::retro_sys::retro_rumble_effect;
use retro_ab_av::{audio_sample_batch_callback, audio_sample_callback, video_refresh_callback};
use retro_ab_gamepad::context::{input_poll_callback, input_state_callback, GamepadContext};
use retro_stack::{RetroStack, StackCommand};
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

pub struct Tinic {
    // stack: Arc<RetroStack>,
    pub controller_ctx: Arc<Mutex<GamepadContext>>,
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
                GamePadState::ButtonPressed(b) => {
                    println!("{:?}", b);
                }
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

        let controller_ctx = Arc::new(Mutex::new(GamepadContext::new(Some(
            gamepad_state_listener,
        ))));

        match controller_ctx.clone().lock() {
            Ok(_controller) => {
                // let gamepads = controller.get_list();
                init_game_loop(controller_ctx.clone(), STACK.clone());
            }
            Err(..) => {}
        }

        Self {
            // stack: RetroStack::new(),
            //TODO:o numero máximo de portas deve ser alterado no futuro
            controller_ctx,
            core_ctx: None,
        }
    }

    pub fn load_core(&mut self, core_path: &String, paths: Paths) -> Result<(), String> {
        STACK.push(StackCommand::LoadCore(
            core_path.to_owned(),
            paths,
            RetroEnvCallbacks {
                audio_sample_batch_callback,
                audio_sample_callback,
                input_poll_callback,
                input_state_callback,
                video_refresh_callback,
                rumble_callback,
            },
        ));

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

    pub fn change_controller_pending(&self) {
        // STACK.push(StackCommand::UpdateControllers);
    }
}
