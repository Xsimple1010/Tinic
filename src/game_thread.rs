use crate::game_thread_handle::spawn_game_thread;
use crate::retro_stack::{RetroStack, StackCommand};
use retro_ab::erro_handle::ErroHandle;
use retro_ab::paths::Paths;
use retro_ab::retro_sys::retro_log_level;
use retro_ab_gamepad::context::GamepadContext;
use std::sync::{Arc, Mutex};

pub struct GameThread {
    pub is_running: Arc<Mutex<bool>>,
    controller_ctx: Arc<Mutex<GamepadContext>>,
    stack: Arc<RetroStack>,
}

impl Drop for GameThread {
    fn drop(&mut self) {
        //isso garante que a thread vai morrer
        self.stack.push(StackCommand::Quit);
    }
}

impl GameThread {
    pub fn new(controller_ctx: Arc<Mutex<GamepadContext>>, stack: Arc<RetroStack>) -> Self {
        Self {
            is_running: Arc::new(Mutex::new(false)),
            controller_ctx,
            stack,
        }
    }

    pub fn stop(&self) {
        self.stack.push(StackCommand::Quit);
    }

    pub fn start(
        &mut self,
        core_path: String,
        rom_path: String,
        paths: Paths,
    ) -> Result<(), ErroHandle> {
        match self.is_running.lock() {
            Ok(mut is_running) => {
                if !(*is_running) {
                    *is_running = true;
                } else {
                    return Err(ErroHandle {
                        level: retro_log_level::RETRO_LOG_DUMMY,
                        message: String::from("thread game ja esta iniciada"),
                    });
                }
            }
            Err(_e) => {
                return Err(ErroHandle {
                    level: retro_log_level::RETRO_LOG_ERROR,
                    message: String::from("erro ao tentar cria a thread de game"),
                });
            }
        }

        self.stack.push(StackCommand::LoadGame(
            core_path.to_owned(),
            rom_path.to_owned(),
            paths,
        ));

        spawn_game_thread(
            self.is_running.clone(),
            self.controller_ctx.clone(),
            self.stack.clone(),
        );

        Ok(())
    }
}
