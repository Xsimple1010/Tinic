use crate::game_thread::game_window_handle::game_window_handle;
use crate::game_thread::stack_commands_handle::stack_commands_handle;
use crate::thread_stack::game_stack::{GameStack, GameStackCommand};
use crate::thread_stack::model_stack::RetroStackFn;
use retro_ab::erro_handle::ErroHandle;
use retro_ab::paths::Paths;
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
    stack: Arc<GameStack>,
}

impl Drop for GameThread {
    fn drop(&mut self) {
        //isso garante que a thread vai morrer
        self.stop();
    }
}

impl GameThread {
    pub fn new(controller_ctx: Arc<Mutex<RetroAbController>>, stack: Arc<GameStack>) -> Self {
        Self {
            is_running: Arc::new(Mutex::new(false)),
            controller_ctx,
            stack,
        }
    }

    pub fn stop(&self) {
        self.stack.push(GameStackCommand::Quit);
    }

    pub fn start(
        &mut self,
        core_path: &str,
        rom_path: &str,
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
            Err(op) => {
                *op.into_inner() = false;

                return Err(ErroHandle {
                    level: retro_log_level::RETRO_LOG_ERROR,
                    message: String::from("erro ao tentar cria a thread de game"),
                });
            }
        }

        self.stack.clear();
        self.stack.push(GameStackCommand::LoadGame(
            core_path.to_string(),
            rom_path.to_string(),
            paths,
        ));

        self.spawn_game_thread();

        Ok(())
    }

    fn spawn_game_thread(&self) {
        let game_stack = self.stack.clone();
        let controller_ctx = self.controller_ctx.clone();
        let is_running = self.is_running.clone();

        thread::spawn(move || {
            let mut pause_request_new_frames = false;
            let mut retro_ab: Option<RetroAB> = None;
            let mut av_ctx: Option<(RetroAvCtx, EventPump)> = None;

            while *is_running.lock().unwrap_or_else(|op| {
                let mut can_run = op.into_inner();
                *can_run = false;

                can_run
            }) {
                if stack_commands_handle(
                    &game_stack,
                    &mut retro_ab,
                    &controller_ctx,
                    &mut av_ctx,
                    &mut pause_request_new_frames,
                ) {
                    break;
                }

                if let Some((av, event_pump)) = &mut av_ctx {
                    if let Some(retro_ab) = &retro_ab {
                        if let Err(e) = try_render_frame(retro_ab, av, pause_request_new_frames) {
                            println!("{:?}", e);
                            game_stack.push(GameStackCommand::Quit);
                            continue;
                        }
                    }

                    game_window_handle(event_pump, &game_stack, pause_request_new_frames);
                }
            }

            game_stack.clear();

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
