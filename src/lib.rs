extern crate retro_ab;
extern crate retro_ab_av;
extern crate retro_ab_gamepad;

mod game_loop;
mod retro_stack;

use game_loop::init_game_loop;
use retro_ab::core::{self, RetroContext, RetroEnvCallbacks};
use retro_ab_av::{audio_sample_batch_callback, audio_sample_callback, video_refresh_callback};
use retro_ab_gamepad::context::{input_poll_callback, input_state_callback, GamepadContext};
use retro_stack::{RetroStack, StackCommand};
use std::sync::Arc;

pub use retro_ab::args_manager;
pub use retro_ab::paths::Paths;
pub use retro_ab::test_tools;

pub struct Tinic {
    stack: Arc<RetroStack>,
    pub controller_ctx: GamepadContext,
    pub core_ctx: Option<Arc<RetroContext>>,
}

impl Tinic {
    pub fn new() -> Tinic {
        Self {
            stack: RetroStack::new(),
            //TODO:o numero máximo de portas deve ser alterado no futuro
            controller_ctx: GamepadContext::new(2),
            core_ctx: None,
        }
    }

    pub fn load(&mut self, core_path: &str, rom_path: String, paths: Paths) -> Result<(), String> {
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
        )?;

        match core::init(&core_ctx) {
            Ok(..) => {
                self.core_ctx = Some(core_ctx.clone());

                let gamepads = self.controller_ctx.search();
                self.stack.push(StackCommand::UpdateControllers);

                init_game_loop(rom_path, core_ctx, gamepads, self.stack.clone());
            }
            Err(e) => {
                return Err(e.message);
            }
        }

        Ok(())
    }

    pub fn pause(&self) {
        self.stack.push(StackCommand::Pause);
    }

    pub fn resume(&self) {
        self.stack.push(StackCommand::Resume);
    }

    pub fn save_state(&self) {
        self.stack.push(StackCommand::SaveState);
    }

    pub fn load_state(&self) {
        self.stack.push(StackCommand::LoadState);
    }

    pub fn quit_game(&self) {
        self.stack.push(StackCommand::GameQuit);
    }

    // pub fn unload_game(&self) {
    //     self.stack.push(StackCommand::UnloadGame);
    // }

    // pub fn load_game(&self) {
    //     self.stack.push(StackCommand::LoadGame);
    // }

    pub fn change_controller_pending(&self) {
        self.stack.push(StackCommand::UpdateControllers);
    }
}
