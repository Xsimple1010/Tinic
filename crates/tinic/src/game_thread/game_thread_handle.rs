use super::stack_commands_handle::stack_commands_handle;
use super::{game_thread_state::ThreadState, game_window_handle::game_window_handle};
use crate::channel::ChannelNotify;
use generics::erro_handle::ErroHandle;
use retro_controllers::RetroController;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{
    sync::{Arc, Mutex},
    thread,
};

pub struct GameThread {
    is_running: Arc<AtomicBool>,
    controller_ctx: Arc<Mutex<RetroController>>,
}

impl GameThread {
    pub fn new(controller_ctx: Arc<Mutex<RetroController>>) -> Self {
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
            controller_ctx,
        }
    }

    pub fn start(&mut self, channel_notify: ChannelNotify) -> Result<(), ErroHandle> {
        if self.is_running.load(Ordering::SeqCst) {
            return Ok(());
        }

        self.is_running.store(true, Ordering::SeqCst);
        self.spawn_game_thread(channel_notify);

        Ok(())
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }

    fn spawn_game_thread(&self, channel_notify: ChannelNotify) {
        let is_running = self.is_running.clone();
        let controller_ctx = self.controller_ctx.clone();

        thread::spawn(move || {
            let mut state = ThreadState::new(channel_notify, controller_ctx, is_running);

            while state.is_running() {
                if let Err(e) = stack_commands_handle(&mut state) {
                    println!("stack_commands_handle -> {:?}", e);
                    break;
                }

                game_window_handle(&mut state);

                if let Err(e) = state.try_render_frame() {
                    println!("try_render_frame -> {:?}", e);
                    break;
                }
            }
        });
    }
}
