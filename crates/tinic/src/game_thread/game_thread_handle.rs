use super::game_thread_channel::GameThreadChannel;
use super::stack_commands_handle::stack_commands_handle;
use super::{game_thread_state::ThreadState, game_window_handle::game_window_handle};
use generics::erro_handle::ErroHandle;
use retro_controllers::RetroController;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{sync::Arc, thread};

#[derive(Debug)]
pub struct GameThread {
    is_running: Arc<AtomicBool>,
    pub channel: GameThreadChannel,
}

impl GameThread {
    pub fn new() -> Self {
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
            channel: GameThreadChannel::new(),
        }
    }

    pub fn start(&self, controller_ctx: Arc<RetroController>) -> Result<(), ErroHandle> {
        if self.is_running.load(Ordering::SeqCst) {
            return Ok(());
        }

        self.is_running.store(true, Ordering::SeqCst);
        self.spawn_game_thread(controller_ctx);

        Ok(())
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }

    fn spawn_game_thread(&self, controller_ctx: Arc<RetroController>) {
        let is_running = self.is_running.clone();
        let controller_ctx = controller_ctx.clone();
        let channel_notify = self.channel.get_notify();

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
