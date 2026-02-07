use crate::retro_stack::{RetroStack, StackCommand};
use retro_ab_av::{Event, EventPump, Keycode};
use std::sync::Arc;

pub fn game_window_handle(
    event_pump: &mut EventPump,
    stack: &Arc<RetroStack>,
    pause_request_new_frames: bool,
) {
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
            } => stack.push(StackCommand::SaveState),
            Event::KeyDown {
                keycode: Some(Keycode::F2),
                ..
            } => stack.push(StackCommand::LoadState),
            Event::KeyDown {
                keycode: Some(Keycode::F3),
                ..
            } => {
                if pause_request_new_frames {
                    stack.push(StackCommand::Resume);
                } else {
                    stack.push(StackCommand::Pause);
                }
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
