use crate::{
    game_window_handle::game_window_handle,
    retro_stack::{RetroStack, StackCommand},
};
use retro_ab::core::{self, RetroContext};
use retro_ab_av::context::{RetroAvCtx, RetroAvEvents};
use retro_ab_gamepad::context::GamepadContext;
use retro_ab_gamepad::retro_gamepad::RetroGamePad;
use std::{
    sync::{Arc, Mutex},
    thread,
};

//TODO: criar uma callback para avisar a interface de possíveis erros
pub fn init_game_loop(
    _gamepads: Arc<Mutex<Vec<RetroGamePad>>>,
    controller_ctx: Arc<Mutex<GamepadContext>>,
    stack: Arc<RetroStack>,
) {
    thread::spawn(move || {
        let mut pause_request_new_frames = false;
        let mut core_ctx: Option<Arc<RetroContext>> = None;
        let mut av_events = RetroAvEvents::new().unwrap();
        let mut av_ctx: Option<RetroAvCtx> = None;
        let mut need_stop_game = false;

        'running: loop {
            for cmd in stack.read() {
                match cmd {
                    StackCommand::LoadGame(core_path, rom_path, paths, callbacks) => {
                        if core_ctx.is_some() {
                            break;
                        }

                        match core::load(&core_path, paths, callbacks) {
                            Ok(ctx) => {
                                match core::init(&ctx) {
                                    Ok(..) => match core::load_game(&ctx, rom_path.as_str()) {
                                        Ok(loaded) => {
                                            if loaded {
                                                if let Ok(mut controller) = controller_ctx.lock() {
                                                    controller.pause_thread_events();
                                                }

                                                av_ctx.replace(RetroAvCtx::new(
                                                    ctx.core.av_info.clone(),
                                                    &av_events,
                                                ));
                                                core_ctx.replace(ctx);
                                            };
                                        }

                                        Err(e) => {
                                            println!("{:?}", e);
                                            break;
                                        }
                                    },
                                    Err(e) => {
                                        println!("{:?}", e);
                                        break;
                                    }
                                };
                            }
                            Err(e) => {
                                println!("{:?}", e);
                                break;
                            }
                        }
                    }
                    StackCommand::StopGame => {
                        if let Some(ctx) = core_ctx.take() {
                            match core::de_init(ctx) {
                                Ok(..) => {}
                                Err(e) => {
                                    println!("{:?}", e);
                                }
                            };

                            av_ctx.take();
                        }
                    }
                    StackCommand::Quit => {
                        if let Some(ctx) = core_ctx.take() {
                            if let Err(e) = core::de_init(ctx) {
                                println!("{:?}", e);
                            }
                        }
                        break 'running;
                    }
                    StackCommand::LoadState => {} //ainda e preciso adicionar isso em retro_ab
                    StackCommand::SaveState => {} //ainda e preciso adicionar isso em retro_ab
                    StackCommand::Pause => {
                        if let Ok(mut controller) = controller_ctx.lock() {
                            controller.resume_thread_events();
                            pause_request_new_frames = true
                        }
                    }
                    StackCommand::Resume => {
                        if let Ok(mut controller) = controller_ctx.lock() {
                            controller.pause_thread_events();
                            pause_request_new_frames = false
                        }
                    }
                    StackCommand::Reset => {
                        if let Some(ctx) = &core_ctx {
                            if let Err(e) = core::reset(ctx) {
                                println!("{:?}", e);
                                if let Err(e) = core::de_init(ctx.to_owned()) {
                                    println!("{:?}", e)
                                };
                            };
                        };
                    }
                    _ => {}
                }
            }

            if let Some(core) = &core_ctx {
                if let Some(av) = &mut av_ctx {
                    need_stop_game =
                        game_window_handle(&pause_request_new_frames, &mut av_events, core, av);
                }
            }

            if need_stop_game {
                core_ctx.take();
                av_ctx.take();
            }
        }
    });
}
