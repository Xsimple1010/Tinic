use crate::{
    game_window_handle::game_window_handle,
    retro_stack::{RetroStack, StackCommand},
    stack_commands_handle::stack_commands_handle,
};
use retro_ab::{
    core::{self, RetroContext},
    paths::Paths,
};
use retro_ab_av::{context::RetroAvCtx, EventPump};
use retro_ab_gamepad::context::GamepadContext;
use std::{
    sync::{Arc, Mutex},
    thread::{self},
};

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
    ) -> Result<(), String> {
        match self.is_running.lock() {
            Ok(mut is_running) => {
                if !(*is_running) {
                    *is_running = true;
                } else {
                    return Err(String::from("thread game ja esta iniciada"));
                }
            }
            Err(_e) => {
                return Err(String::from("erro ao tentar cria a thread de game"));
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

fn spawn_game_thread(
    is_running: Arc<Mutex<bool>>,
    controller_ctx: Arc<Mutex<GamepadContext>>,
    stack: Arc<RetroStack>,
) {
    thread::spawn(move || {
        let mut pause_request_new_frames = false;
        let mut core_ctx: Option<Arc<RetroContext>> = None;
        let mut av_ctx: Option<(RetroAvCtx, EventPump)> = None;

        while *is_running.lock().unwrap() {
            if stack_commands_handle(
                &stack,
                &mut core_ctx,
                &controller_ctx,
                &mut av_ctx,
                &mut pause_request_new_frames,
            ) {
                break;
            } else if !pause_request_new_frames {
                if let Some((av, _)) = &mut av_ctx {
                    if av.sync() {
                        if let Some(core_ctx) = &core_ctx {
                            if let Err(e) = core::run(core_ctx) {
                                println!("{:?}", e);
                                break;
                            };
                        }

                        let _ = av.get_new_frame();
                    }
                }
            }

            if let Some((_, event_pump)) = &mut av_ctx {
                game_window_handle(event_pump, &stack, &mut pause_request_new_frames);
            }
        }

        //TODO: preciso adiciona Drop ao RetroContext
        if let Some(core_ctx) = core_ctx.take() {
            let _ = core::de_init(core_ctx);
        };

        //TODO: isso pode fica na stack_handle
        if let Ok(ctr) = &mut controller_ctx.lock() {
            ctr.resume_thread_events();
        }

        stack.clear();

        match is_running.lock() {
            Ok(mut is_running) => {
                *is_running = false;
            }
            Err(_e) => {}
        }
    });
}
