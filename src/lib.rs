extern crate retro_ab;
extern crate retro_ab_av;
extern crate retro_ab_gamepad;
#[macro_use]
extern crate lazy_static;

mod game_loop;
mod retro_stack;

use game_loop::init_game_loop;
use retro_ab::core::{self, RetroEnvCallbacks};
use retro_ab_av::{audio_sample_batch_callback, audio_sample_callback, video_refresh_callback};
use retro_ab_gamepad::context::{input_poll_callback, input_state_callback, GamepadContext};
use retro_stack::RetroStack;
use std::sync::{Arc, Mutex};

pub use retro_ab::args_manager;
pub use retro_ab::paths::Paths;
pub use retro_ab::test_tools;

lazy_static! {
    static ref STACK: Arc<RetroStack> = RetroStack::new();
    static ref CONTROLLER_CTX: Mutex<GamepadContext> = Mutex::new(GamepadContext::new(2));
}

pub fn pause() {}

pub fn resume() {}

pub fn get_gamepads() {}

pub fn change_controller_pending() {}

pub fn load(core_path: &str, rom_path: String, paths: Paths) {
    let core_ctx = core::load(
        core_path,
        paths,
        RetroEnvCallbacks {
            audio_sample_batch_callback,
            audio_sample_callback,
            input_poll_callback,
            input_state_callback,
            video_refresh_callback,
        },
    )
    .expect("Erro ao tentar inicia o Core");

    let gamepads = CONTROLLER_CTX.lock().unwrap().search();
    STACK.push(retro_stack::StackCommand::UpdateControllers);

    init_game_loop(rom_path, core_ctx, gamepads, STACK.clone());
}
