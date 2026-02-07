use std::{
    sync::{Arc, Mutex},
    thread,
};

use retro_ab::core::{self, RetroContext};
use retro_ab_av::{context::RetroAvCtx, Event, Keycode};
use retro_ab_gamepad::retro_gamepad::RetroGamePad;

use crate::retro_stack::{RetroStack, StackCommand};

pub fn init_game_loop(
    rom_path: String,
    core_ctx: Arc<RetroContext>,
    gamepads: Arc<Mutex<Vec<RetroGamePad>>>,
    stack: Arc<RetroStack>,
) {
    let th = thread::spawn(move || {
        core::load_game(&core_ctx, rom_path.as_str()).expect("msg");

        let (mut av_ctx, mut event_pump) = RetroAvCtx::new(core_ctx.core.av_info.clone())
            .expect("nao foi possível incia retro_ab_av");

        let mut _pause = false;

        'running: loop {
            for cmd in stack.read() {
                match cmd {
                    // StackCommand::LoadGame => {} //preciso limpar o av_ctx quando um novo jogo for carregado
                    // StackCommand::UnloadGame => core::unload_game(&core_ctx).expect("msg"),
                    StackCommand::GameQuit => break 'running,
                    StackCommand::LoadState => {} //ainda e preciso adicionar isso em retro_ab
                    StackCommand::SaveState => {} //ainda e preciso adicionar isso em retro_ab
                    StackCommand::Pause => _pause = true,
                    StackCommand::Resume => _pause = false,
                    StackCommand::UpdateControllers => {
                        for gamepad in &*gamepads.lock().unwrap() {
                            if gamepad.retro_port >= 0 {
                                core::connect_controller(
                                    &core_ctx,
                                    gamepad.retro_port as u32,
                                    gamepad.retro_type,
                                );
                            }
                        }
                    }
                }
            }

            if !_pause && *core_ctx.core.game_loaded.lock().unwrap() {
                core::run(&core_ctx).expect("msg");
                av_ctx.get_new_frame().expect("");
            }

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
