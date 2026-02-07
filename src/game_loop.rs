use std::{
    sync::{Arc, Mutex},
    thread,
};

use retro_ab::{
    core::{self, RetroContext},
    retro_sys::retro_log_level,
};
use retro_ab_av::{context::RetroAvCtx, Event, EventPump, Keycode};
use retro_ab_gamepad::retro_gamepad::RetroGamePad;

use crate::retro_stack::{RetroStack, StackCommand};

//TODO: criar uma callback para avisar a interface de possíveis erros
pub fn init_game_loop(
    core_ctx: Arc<RetroContext>,
    gamepads: Arc<Mutex<Vec<RetroGamePad>>>,
    stack: Arc<RetroStack>,
) {
    thread::spawn(move || {
        let mut _pause = false;
        let mut retro_av: Option<(RetroAvCtx, EventPump)> = None;

        'running: loop {
            for cmd in stack.read() {
                match cmd {
                    StackCommand::LoadGame(path) => {
                        let result = core::load_game(&core_ctx, path.as_str());

                        match result {
                            Ok(s) => {
                                if s {
                                    let result = RetroAvCtx::new(core_ctx.core.av_info.clone());

                                    match result {
                                        Ok(retro_av_) => {
                                            retro_av = Some(retro_av_);
                                        }
                                        Err(e) => println!("{:?}", e),
                                    }
                                }
                            }
                            Err(e) => {
                                println!("{:?}", e)
                            }
                        }
                    }
                    StackCommand::UnloadGame => match core::unload_game(&core_ctx) {
                        Ok(..) => {
                            retro_av = None;
                        }
                        Err(e) => {
                            println!("{:?}", e);
                            break 'running;
                        }
                    },
                    StackCommand::GameQuit => break 'running,
                    StackCommand::LoadState => {} //ainda e preciso adicionar isso em retro_ab
                    StackCommand::SaveState => {} //ainda e preciso adicionar isso em retro_ab
                    StackCommand::Pause => _pause = true,
                    StackCommand::Resume => _pause = false,
                    StackCommand::Reset => {
                        if core::reset(&core_ctx).is_err() {
                            break 'running;
                        }
                    }
                    StackCommand::UpdateControllers => {
                        for gamepad in &*gamepads.lock().unwrap() {
                            if gamepad.retro_port >= 0 {
                                let result = core::connect_controller(
                                    &core_ctx,
                                    gamepad.retro_port as u32,
                                    gamepad.retro_type,
                                );

                                match result {
                                    Ok(..) => {}
                                    Err(e) => println!("{:?}", e),
                                }
                            }
                        }
                    }
                }
            }

            if !_pause {
                match core::run(&core_ctx) {
                    Ok(..) => {
                        if let Some((av_ctx, _)) = &mut retro_av {
                            if av_ctx.get_new_frame().is_err() {
                                break 'running;
                            }
                        }
                    }
                    Err(e) => match e.level {
                        retro_log_level::RETRO_LOG_ERROR => break 'running,
                        retro_log_level::RETRO_LOG_WARN => {}
                        _ => println!("{:?}", e),
                    },
                };
            }

            if let Some((_, event_pump)) = &mut retro_av {
                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. }
                        | Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        } => break 'running,
                        Event::KeyDown {
                            keycode: Some(Keycode::F1),
                            ..
                        } => {
                            //reservado para o save state
                        }
                        Event::KeyDown {
                            keycode: Some(Keycode::F2),
                            ..
                        } => {
                            //reservado para o save state
                        }
                        Event::KeyDown {
                            keycode: Some(Keycode::F3),
                            ..
                        } => {
                            _pause = !_pause;
                        }
                        Event::KeyDown {
                            keycode: Some(Keycode::F5),
                            ..
                        } => {
                            if core::reset(&core_ctx).is_err() {
                                break 'running;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        match core::de_init(core_ctx) {
            Ok(..) => {}
            Err(e) => println!("{:?}", e),
        };
    });
}
