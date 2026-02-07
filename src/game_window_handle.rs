use crate::retro_stack::{RetroStack, StackCommand};
use retro_ab_av::{Event, EventPump, Keycode};
use std::sync::Arc;

pub fn game_window_handle(event_pump: &mut EventPump, stack: &Arc<RetroStack>) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => stack.push(StackCommand::Quit),
            Event::KeyDown {
                keycode: Some(Keycode::F1),
                ..
            } => {
                //reservado para o save state
                stack.push(StackCommand::SaveState)
            }
            Event::KeyDown {
                keycode: Some(Keycode::F2),
                ..
            } => {
                //reservado para o save state
                stack.push(StackCommand::LoadState)
            }
            //pausa a rom
            Event::KeyDown {
                keycode: Some(Keycode::F3),
                ..
            } => {
                // if *pause_request_new_frames {
                //     if let Ok(mut controller) = controller_ctx.lock() {
                //         controller.stop_thread_events();
                //         *pause_request_new_frames = false
                //     }
                // } else if let Ok(mut controller) = controller_ctx.lock() {
                //     controller.resume_thread_events();
                //     *pause_request_new_frames = true
                // }
            }
            Event::KeyDown {
                keycode: Some(Keycode::F5),
                ..
            } => stack.push(StackCommand::Reset),
            _ => {}
        }
    }

    // need_stop
}
