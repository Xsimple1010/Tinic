use crate::channel::ChannelNotify;
use crate::thread_stack::game_stack::GameStackCommand;
use retro_ab_av::{Event, EventPump, Keycode};

pub fn game_window_handle(
    event_pump: &mut EventPump,
    channel_notify: &ChannelNotify,
    pause_request_new_frames: bool,
    use_full_screen: bool,
) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => channel_notify.notify_game_stack(GameStackCommand::Quit),
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

    // need_stop
}
