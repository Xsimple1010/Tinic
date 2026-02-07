use std::sync::{Arc, Mutex};

use retro_ab::core::{self, RetroContext};
use retro_ab_av::{context::RetroAvCtx, EventPump};
use retro_ab_gamepad::context::GamepadContext;

use crate::retro_stack::{RetroStack, StackCommand};

pub fn stack_handle(
    stack: &Arc<RetroStack>,
    core_ctx: &mut Option<Arc<RetroContext>>,
    controller_ctx: &Arc<Mutex<GamepadContext>>,
    av_ctx: &mut Option<(RetroAvCtx, EventPump)>,
    pause_request_new_frames: &mut bool,
) -> bool {
    let mut need_stop = false;

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

                                        match RetroAvCtx::new(ctx.core.av_info.clone()) {
                                            Ok(ctx) => {
                                                av_ctx.replace(ctx);
                                            }
                                            Err(e) => {
                                                println!("{:?}", e);
                                                break;
                                            }
                                        }

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
            StackCommand::Quit => {
                need_stop = true;
                break;
            }
            StackCommand::LoadState => {} //ainda e preciso adicionar isso em retro_ab
            StackCommand::SaveState => {} //ainda e preciso adicionar isso em retro_ab
            StackCommand::Pause => {
                if let Ok(mut controller) = controller_ctx.lock() {
                    controller.resume_thread_events();
                    *pause_request_new_frames = true
                }
            }
            StackCommand::Resume => {
                if let Ok(mut controller) = controller_ctx.lock() {
                    controller.pause_thread_events();
                    *pause_request_new_frames = false
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

    need_stop
}
