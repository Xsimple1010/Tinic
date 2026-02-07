use crate::thread_stack::game_stack::{GameStack, GameStackCommand};
use crate::thread_stack::model_stack::RetroStackFn;
use retro_ab_av::{Event, EventPump, Keycode};
use std::sync::Arc;

pub fn game_window_handle(
    event_pump: &mut EventPump,
    stack: &Arc<GameStack>,
    pause_request_new_frames: bool,
) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => stack.push(GameStackCommand::Quit),
            Event::KeyDown {
                keycode: Some(Keycode::F1),
                ..
            } => stack.push(GameStackCommand::SaveState(1)),
            Event::KeyDown {
                keycode: Some(Keycode::F2),
                ..
            } => stack.push(GameStackCommand::LoadState(1)),
            Event::KeyDown {
                keycode: Some(Keycode::F8),
                ..
            } => {
                if pause_request_new_frames {
                    stack.push(GameStackCommand::Resume);
                } else {
                    stack.push(GameStackCommand::Pause);
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::F5),
                ..
            } => stack.push(GameStackCommand::Reset),
            _ => {}
        }
    }

    // need_stop
}
