use crate::channel::ChannelNotify;
use crate::game_thread::game_window_handle::game_window_handle;
use crate::game_thread::stack_commands_handle::stack_commands_handle;
use retro_ab::erro_handle::ErroHandle;
use retro_ab::retro_ab::RetroAB;
use retro_ab::retro_sys::retro_log_level;
use retro_ab_av::retro_av::RetroAvCtx;
use retro_ab_av::EventPump;
use retro_ab_gamepad::RetroAbController;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct GameThread {
    pub is_running: Arc<Mutex<bool>>,
    controller_ctx: Arc<Mutex<RetroAbController>>,
}

impl Drop for GameThread {
    fn drop(&mut self) {
        //isso garante que a thread vai morrer
        self.stop();
    }
}

impl GameThread {
    pub fn new(controller_ctx: Arc<Mutex<RetroAbController>>) -> Self {
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
                        level: retro_log_level::RETRO_LOG_DUMMY,
                        message: String::from("thread game ja esta iniciada"),
                    });
                }
            }
            Err(op) => {
                *op.into_inner() = false;

                return Err(ErroHandle {
                    level: retro_log_level::RETRO_LOG_ERROR,
                    message: String::from("erro ao tentar cria a thread de game"),
                });
            }
        }

        self.spawn_game_thread(channel_notify);

        Ok(())
    }

    fn spawn_game_thread(&self, channel_notify: ChannelNotify) {
        let controller_ctx = self.controller_ctx.clone();
        let is_running = self.is_running.clone();

        thread::spawn(move || {
            let mut pause_request_new_frames = false;
            let mut use_full_screen_mode = false;
            let mut retro_ab: Option<RetroAB> = None;
            let mut av_ctx: Option<(RetroAvCtx, EventPump)> = None;

            while *is_running.lock().unwrap_or_else(|op| {
                let mut can_run = op.into_inner();
                *can_run = false;

                can_run
            }) {
                if stack_commands_handle(
                    &channel_notify,
                    &mut retro_ab,
                    &controller_ctx,
                    &mut av_ctx,
                    &mut pause_request_new_frames,
                    &mut use_full_screen_mode,
                ) {
                    break;
                }

                if let Some((av, event_pump)) = &mut av_ctx {
                    if let Some(retro_ab) = &retro_ab {
                        if let Err(e) = try_render_frame(retro_ab, av, pause_request_new_frames) {
                            println!("{:?}", e);
                            break;
                        }
                    }

                    game_window_handle(
                        event_pump,
                        &channel_notify,
                        pause_request_new_frames,
                        use_full_screen_mode,
                    );
                }
            }

            channel_notify.clear_game_stack();

            //Gracas ao mutex is-running pode ser que algo externo atrapalhe a leitura dos comandos da stack,
            //então so para garantir que essa thread será fechada dando a posse da leitura dos inputs para a
            //thread de inputs novamente, o bom é fazer isso aqui mesmo!
            if let Ok(ctr) = &mut controller_ctx.lock() {
                let _ = ctr.resume_thread_events();
            }

            match is_running.lock() {
                Ok(mut is_running) => {
                    *is_running = false;
                }
                Err(op) => {
                    *op.into_inner() = false;
                }
            }
        });
    }
}

fn try_render_frame(
    retro_ab: &RetroAB,
    av: &mut RetroAvCtx,
    paused: bool,
) -> Result<(), ErroHandle> {
    if !av.sync() || paused {
        return Ok(());
    }

    retro_ab.core().run()?;

    av.get_new_frame();

    Ok(())
}
