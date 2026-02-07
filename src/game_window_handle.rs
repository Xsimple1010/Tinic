use std::sync::Arc;

use retro_ab::core::{self, RetroContext};
use retro_ab_av::{
    context::{RetroAvCtx, RetroAvEvents},
    Event, Key, KeyEvent, NamedKey, WindowEvent,
};

pub fn game_window_handle(
    pause_request_new_frames: &bool,
    av_events: &mut RetroAvEvents,
    core_ctx: &Arc<RetroContext>,
    av_ctx: &mut RetroAvCtx,
) -> bool {
    if !*pause_request_new_frames {
        core::run(&core_ctx).unwrap();
        av_ctx.request_redraw();
    }

    let mut need_stop = false;

    av_events.pump(|event, window_target| match event {
        Event::Resumed => {
            av_ctx.resume(window_target);
        }
        Event::Suspended => {
            av_ctx.suspended();
        }
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Destroyed => {}
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: Key::Named(NamedKey::Escape),
                        ..
                    },
                ..
            } => {
                av_ctx.close_window(window_target);
                need_stop = true;
            }
            WindowEvent::RedrawRequested => {
                let _ = av_ctx.get_new_frame();
            }
            _ => {}
        },
        Event::LoopExiting => {
            println!("saindo do loop");
        }
        _ => {}
    });

    need_stop
}
