use crate::game_window_handle::game_window_handle;
use crate::retro_stack::RetroStack;
use crate::stack_commands_handle::stack_commands_handle;
use retro_ab::core::RetroContext;
use retro_ab_av::context::RetroAvCtx;
use retro_ab_av::EventPump;
use retro_ab_gamepad::context::GamepadContext;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn spawn_game_thread(
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
                            if let Err(e) = retro_ab::core::run(core_ctx) {
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
            let _ = retro_ab::core::de_init(core_ctx);
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
