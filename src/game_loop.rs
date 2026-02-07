use std::{
    sync::{Arc, Mutex},
    thread,
};

use retro_ab::core::{self, RetroContext};
use retro_ab_av::{context::RetroAvCtx, Event, Keycode};
use retro_ab_gamepad::retro_gamepad::RetroGamePad;

pub fn init_game_loop(core_ctx: Arc<RetroContext>, gamepads: Arc<Mutex<Vec<RetroGamePad>>>) {
    for gamepad in &*gamepads.lock().unwrap() {
        if gamepad.retro_port >= 0 {
            core::connect_controller(&core_ctx, gamepad.retro_port as u32, gamepad.retro_type);
        }
    }

    let th = thread::spawn(move || {
        let (mut av_ctx, mut event_pump) = RetroAvCtx::new(core_ctx.core.av_info.clone())
            .expect("nao foi possível incia retro_ab_av");

        'running: loop {
            core::run(&core_ctx).expect("msg");
            av_ctx.get_new_frame().expect("");

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,

                    _ => {}
                }
            }
        }

        core::de_init(core_ctx).expect("ds");
    });

    th.join().expect("erro na thread");
}
