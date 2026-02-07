use std::sync::{Arc, Mutex};

use retro_ab::core::{self, RetroContext};
use retro_ab_av::{
    context::{RetroAvCtx, RetroAvEvents},
    EventLoop,
};
use retro_ab_gamepad::{context::GamepadContext, retro_gamepad::RetroGamePad};

use crate::retro_stack::{RetroStack, StackCommand};

//return true to stop thread
pub fn stack_actions(
    core_ctx_ptr: &mut Option<Arc<RetroContext>>,
    gamepads: &Arc<Mutex<Vec<RetroGamePad>>>,
    controller_ctx: &Arc<Mutex<GamepadContext>>,
    stack: &Arc<RetroStack>,
    retro_av: &mut Option<RetroAvCtx>,
    av_events: &RetroAvEvents,
    pause_request_new_frames: &mut bool,
) -> bool {
    let mut need_break = false;

    for cmd in stack.read() {
        match cmd {
            StackCommand::LoadCore(path, paths, callbacks) => {
                match core::load(&path, paths, callbacks) {
                    Ok(core_ctx) => match core::init(&core_ctx) {
                        Ok(..) => {
                            core_ctx_ptr.replace(core_ctx.clone());
                        }
                        Err(e) => {
                            println!("{:?}", e);
                            need_break = true;
                            break;
                        }
                    },
                    Err(e) => {
                        println!("{:?}", e);
                        need_break = true;
                        break;
                    }
                };
            }
            StackCommand::LoadGame(path) => {
                match core_ctx_ptr {
                    Some(core_ctx) => {
                        match core::load_game(&core_ctx, path.as_str()) {
                            Ok(loaded) => {
                                if loaded {
                                    if let Ok(mut controller) = controller_ctx.lock() {
                                        controller.pause_thread_events();
                                    }

                                    retro_av.replace(RetroAvCtx::new(
                                        core_ctx.core.av_info.clone(),
                                        av_events,
                                    ));
                                }
                            }
                            Err(e) => {
                                println!("{:?}", e);
                                need_break = true;
                                break;
                            }
                        };
                    }
                    None => {
                        println!("core context nao existe!");
                    }
                };
            }
            StackCommand::UnloadGame => {
                match core_ctx_ptr {
                    Some(core_ctx) => {
                        match core::unload_game(&core_ctx) {
                            Ok(..) => {
                                if let Ok(mut controller) = controller_ctx.lock() {
                                    controller.resume_thread_events();
                                }
                                // retro_av = None;
                            }
                            Err(e) => {
                                println!("{:?}", e);
                                need_break = true;
                                break;
                            }
                        }
                    }
                    None => {
                        println!("core context nao existe!");
                        need_break = true;
                        break;
                    }
                };
            }
            StackCommand::GameQuit => {
                need_break = true;
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
            // StackCommand::Reset => {
            //     if let Err(e) = core::reset(&core_ctx) {
            //         println!("{:?}", e);
            //         need_break = true;
            //         break;
            //     };
            // }
            // StackCommand::UpdateControllers => {
            //     for gamepad in &*gamepads.lock().unwrap() {
            //         if gamepad.retro_port >= 0 {
            //             let result = core::connect_controller(
            //                 &core_ctx,
            //                 gamepad.retro_port as u32,
            //                 gamepad.retro_type,
            //             );

            //             match result {
            //                 Ok(..) => {}
            //                 Err(e) => {
            //                     println!("{:?}", e);
            //                     need_break = true;
            //                     break;
            //                 }
            //             }
            //         }
            //     }
            // }
            _ => {}
        }
    }

    return need_break;
}
