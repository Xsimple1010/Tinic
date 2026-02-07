use crate::thread_stack::game_stack::GameStackCommand;
use generics::erro_handle::ErroHandle;
use retro_av::{Event, Keycode};
use std::sync::atomic::Ordering;

use super::game_thread_state::ThreadState;

pub fn game_window_handle(state: &mut ThreadState) -> Result<(), ErroHandle> {
    let (_, ref mut event_pump) = match &mut state.retro_av {
        Some(av) => av,
        None => return Ok(()),
    };
    let channel_notify = &state.channel_notify;
    let pause_request_new_frames = state.pause_request_new_frames;
    let use_full_screen = state.use_full_screen_mode;

    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => state.is_running.store(false, Ordering::SeqCst),
            Event::KeyDown {
                keycode: Some(Keycode::F1),
                ..
            } => channel_notify.notify_game_stack(GameStackCommand::SaveState(1)),
            Event::KeyDown {
                keycode: Some(Keycode::F2),
                ..
            } => channel_notify.notify_game_stack(GameStackCommand::LoadState(1)),

            Event::KeyDown {
                keycode: Some(Keycode::F8),
                ..
            } => {
                if pause_request_new_frames {
                    channel_notify.notify_game_stack(GameStackCommand::Resume);
                } else {
                    channel_notify.notify_game_stack(GameStackCommand::Pause);
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::F5),
                ..
            } => channel_notify.notify_game_stack(GameStackCommand::Reset),

            Event::KeyDown {
                keycode: Some(Keycode::F11),
                ..
            } => {
                if use_full_screen {
                    channel_notify.notify_game_stack(GameStackCommand::DisableFullScreen)
                } else {
                    channel_notify.notify_game_stack(GameStackCommand::EnableFullScreen)
                }
            }
            _ => {}
        }
    }

    Ok(())
}
