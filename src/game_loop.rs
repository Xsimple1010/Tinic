use crate::{
    game_window_handle::game_window_handle, retro_stack::RetroStack, stack_handle::stack_handle,
};
use retro_ab::core::RetroContext;
use retro_ab_av::{context::RetroAvCtx, EventPump};
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
        let mut av_ctx: Option<(RetroAvCtx, EventPump)> = None;
        let mut need_stop_game = false;

        'running: loop {
            if stack_handle(
                &stack,
                &mut core_ctx,
                &controller_ctx,
                &mut av_ctx,
                &mut pause_request_new_frames,
            ) {
                break 'running;
            }

            if let Some(core) = &core_ctx {
                if let Some((av, event_pump)) = &mut av_ctx {
                    need_stop_game = game_window_handle(
                        &mut pause_request_new_frames,
                        event_pump,
                        core,
                        av,
                        &controller_ctx,
                    );
                }
            }

            if need_stop_game {
                break 'running;
            }
        }

        if let Some(core_ctx) = core_ctx.take() {
            let _ = retro_ab::core::de_init(core_ctx);
        };
        if let Ok(ctr) = &mut controller_ctx.lock() {
            ctr.resume_thread_events();
        }
        av_ctx.take();
    });
}
