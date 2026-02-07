use crate::game_window_handle::game_window_handle;
use crate::retro_stack::{RetroStack, StackCommand};
use crate::stack_commands_handle::stack_commands_handle;
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
    stack: Arc<RetroStack>,
}

impl Drop for GameThread {
    fn drop(&mut self) {
        //isso garante que a thread vai morrer
        self.stop();
    }
}

impl GameThread {
    pub fn new(controller_ctx: Arc<Mutex<RetroAbController>>, stack: Arc<RetroStack>) -> Self {
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
            Err(op) => {
                let mut is_running = op.into_inner();
                *is_running = false;

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

        self.spawn_game_thread();

        Ok(())
    }

    fn spawn_game_thread(&self) {
        let stack = self.stack.clone();
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
                    &stack,
                    &mut retro_ab,
                    &controller_ctx,
                    &mut av_ctx,
                    &mut pause_request_new_frames,
                ) {
                    break;
                }

                if let Some((av, event_pump)) = &mut av_ctx {
                    if let Some(retro_ab) = &retro_ab {
                        if let Err(e) = try_render_frame(&retro_ab, av, pause_request_new_frames) {
                            println!("{:?}", e);
                            stack.push(StackCommand::Quit);
                            continue;
                        }
                    }

                    game_window_handle(event_pump, &stack, pause_request_new_frames);
                }
            }

            stack.clear();

            //gracas ao mutex is_running pode ser que algo externo atrapalhe a leitura do comandos da stack,
            //então so para garanti que essa thread sera fechada dando a posse da leitura dos inputs para a
            //thread de inputs novamente, o bom é fazer isso aqui mesmo!
            if let Ok(ctr) = &mut controller_ctx.lock() {
                let _ = ctr.resume_thread_events();
            }

            match is_running.lock() {
                Ok(mut is_running) => {
                    *is_running = false;
                }
                Err(op) => {
                    let mut is_running = op.into_inner();
                    *is_running = false;
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
