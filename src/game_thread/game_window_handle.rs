use crate::channel::ThreadChannel;
use retro_ab_av::{Event, EventPump, Keycode};
use std::sync::Arc;

pub fn game_window_handle(
    event_pump: &mut EventPump,
    channel: &Arc<ThreadChannel>,
    pause_request_new_frames: bool,
    use_full_screen: bool,
) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => channel.quit(),
            Event::KeyDown {
                keycode: Some(Keycode::F1),
                ..
            } => channel.save_state(1),
            Event::KeyDown {
                keycode: Some(Keycode::F2),
                ..
            } => channel.load_state(1),
            Event::KeyDown {
                keycode: Some(Keycode::F8),
                ..
            } => {
                if pause_request_new_frames {
                    channel.resume_game();
                } else {
                    channel.pause_game();
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::F5),
                ..
            } => channel.reset_game(),

            Event::KeyDown {
                keycode: Some(Keycode::F11),
                ..
            } => {
                if use_full_screen {
                    channel.disable_full_screen()
                } else {
                    channel.enable_full_screen()
                }
            }
            _ => {}
        }
    }

    // need_stop
}
