use super::stack_commands_handle::stack_commands_handle;
use super::{game_thread_state::ThreadState, game_window_handle::game_window_handle};
use crate::channel::ChannelNotify;
use generics::erro_handle::ErroHandle;
use libretro_sys::binding_libretro::retro_log_level::{RETRO_LOG_DUMMY, RETRO_LOG_ERROR};
use retro_controllers::RetroController;
use std::{
    sync::{Arc, Mutex},
    thread,
};

pub struct GameThread {
    pub is_running: Arc<Mutex<bool>>,
    controller_ctx: Arc<Mutex<RetroController>>,
}

impl Drop for GameThread {
    fn drop(&mut self) {
        //isso garante que a thread vai morrer
        self.stop();
    }
}

impl GameThread {
    pub fn new(controller_ctx: Arc<Mutex<RetroController>>) -> Self {
        Self {
            is_running: Arc::new(Mutex::new(false)),
            controller_ctx,
        }
    }

    pub fn stop(&mut self) {
        match self.is_running.lock() {
            Ok(mut is_running) => {
                *is_running = false;
            }
            Err(op) => {
                *op.into_inner() = false;
            }
        }
    }

    pub fn start(&mut self, channel_notify: ChannelNotify) -> Result<(), ErroHandle> {
        match self.is_running.lock() {
            Ok(mut is_running) => {
                if !(*is_running) {
                    *is_running = true;
                } else {
                    return Err(ErroHandle {
                        level: RETRO_LOG_DUMMY,
                        message: String::from("thread game ja esta iniciada"),
                    });
                }
            }
            Err(op) => {
                *op.into_inner() = false;

                return Err(ErroHandle {
                    level: RETRO_LOG_ERROR,
                    message: String::from("erro ao tentar cria a thread de game"),
                });
            }
        }

        self.spawn_game_thread(channel_notify);

        Ok(())
    }

    fn spawn_game_thread(&self, channel_notify: ChannelNotify) {
        let is_running = self.is_running.clone();
        let controller_ctx = self.controller_ctx.clone();

        thread::spawn(move || {
            let mut state = ThreadState {
                pause_request_new_frames: false,
                retro_av: None,
                retro_core: None,
                use_full_screen_mode: false,
                channel_notify,
                controller_ctx,
                is_running,
            };

            while state.is_running() {
                if let Err(e) = stack_commands_handle(&mut state) {
                    println!("stack_commands_handle -> {:?}", e);
                    break;
                }

                if let Err(e) = game_window_handle(&mut state) {
                    println!("game_window_handle -> {:?}", e);
                    break;
                }

                if let Err(e) = state.try_render_frame() {
                    println!("try_render_frame -> {:?}", e);
                    break;
                }
            }
        });
    }
}
