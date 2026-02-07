use std::sync::{Arc, Mutex};

use retro_ab::core::{self, RetroContext, RetroEnvCallbacks};
use retro_ab_av::{
    audio_sample_batch_callback, audio_sample_callback, context::RetroAvCtx,
    video_refresh_callback, EventPump,
};
use retro_ab_gamepad::context::{
    input_poll_callback, input_state_callback, rumble_callback, GamepadContext,
};

use crate::retro_stack::{
    RetroStack,
    StackCommand::{GamepadConnected, LoadGame, LoadState, Pause, Quit, Reset, Resume, SaveState},
};

pub fn stack_commands_handle(
    stack: &Arc<RetroStack>,
    core_ctx: &mut Option<Arc<RetroContext>>,
    controller_ctx: &Arc<Mutex<GamepadContext>>,
    av_ctx: &mut Option<(RetroAvCtx, EventPump)>,
    pause_request_new_frames: &mut bool,
) -> bool {
    let mut need_stop = false;

    for cmd in stack.read() {
        match cmd {
            Quit => {
                need_stop = true;
                break;
            }
            LoadGame(core_path, rom_path, paths) => {
                if core_ctx.is_some() {
                    break;
                }

                let callbacks = RetroEnvCallbacks {
                    audio_sample_batch_callback,
                    audio_sample_callback,
                    input_poll_callback,
                    input_state_callback,
                    video_refresh_callback,
                    rumble_callback,
                };

                match core::load(&core_path, paths, callbacks) {
                    Ok(ctx) => {
                        match core::init(&ctx) {
                            Ok(..) => match core::load_game(&ctx, rom_path.as_str()) {
                                Ok(loaded) => {
                                    if loaded {
                                        if let Ok(mut controller) = controller_ctx.lock() {
                                            controller.stop_thread_events();
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
            LoadState => {} //ainda e preciso adicionar isso em retro_ab
            SaveState => {} //ainda e preciso adicionar isso em retro_ab
            Pause => {
                //habilita a thread de eventos novamente
                if let Ok(mut controller) = controller_ctx.lock() {
                    controller.resume_thread_events();
                    *pause_request_new_frames = true
                }
            }
            Resume => {
                //como a rom estará em execução é necessário interromper a thread de eventos
                if let Ok(mut controller) = controller_ctx.lock() {
                    controller.stop_thread_events();
                    *pause_request_new_frames = false
                }
            }
            Reset => {
                if let Some(ctx) = &core_ctx {
                    if let Err(e) = core::reset(ctx) {
                        println!("{:?}", e);
                        if let Err(e) = core::de_init(ctx.to_owned()) {
                            println!("{:?}", e)
                        };

                        need_stop = true;
                    };
                };
            }
            GamepadConnected(gamepad) => {
                if let Some(ctx) = core_ctx {
                    let _ = core::connect_controller(
                        ctx,
                        gamepad.retro_port as u32,
                        gamepad.retro_type,
                    );
                }
            }
        }
    }

    need_stop
}
