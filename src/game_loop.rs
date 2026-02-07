use std::{
    sync::{Arc, Mutex},
    thread,
};

use retro_ab::core::{self, RetroContext};
use retro_ab_av::{context::RetroAvCtx, Event, EventPump, Keycode};
use retro_ab_gamepad::{context::GamepadContext, retro_gamepad::RetroGamePad};

use crate::retro_stack::{RetroStack, StackCommand};

//TODO: criar uma callback para avisar a interface de possíveis erros
pub fn init_game_loop(
    core_ctx: Arc<RetroContext>,
    gamepads: Arc<Mutex<Vec<RetroGamePad>>>,
    controller_ctx: Arc<Mutex<GamepadContext>>,
    stack: Arc<RetroStack>,
) {
    thread::spawn(move || {
        let mut _pause_request_new_frames = false;
        let mut retro_av: Option<(RetroAvCtx, EventPump)> = None;

        'running: loop {
            for cmd in stack.read() {
                match cmd {
                    StackCommand::LoadGame(path) => {
                        let result = core::load_game(&core_ctx, path.as_str());

                        match result {
                            Ok(s) => {
                                if s {
                                    if let Ok(mut controller) = controller_ctx.lock() {
                                        controller.pause_thread_events();
                                    }

                                    match RetroAvCtx::new(core_ctx.core.av_info.clone()) {
                                        Ok(retro_av_) => {
                                            retro_av = Some(retro_av_);
                                        }
                                        Err(e) => {
                                            println!("{:?}", e);
                                            break 'running;
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                println!("{:?}", e);
                                break 'running;
                            }
                        }
                    }
                    StackCommand::UnloadGame => match core::unload_game(&core_ctx) {
                        Ok(..) => {
                            if let Ok(mut controller) = controller_ctx.lock() {
                                controller.resume_thread_events();
                            }
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
                    StackCommand::Pause => {
                        if let Ok(mut controller) = controller_ctx.lock() {
                            controller.resume_thread_events();
                            _pause_request_new_frames = true
                        }
                    }
                    StackCommand::Resume => {
                        if let Ok(mut controller) = controller_ctx.lock() {
                            controller.pause_thread_events();
                            _pause_request_new_frames = false
                        }
                    }
                    StackCommand::Reset => {
                        if let Err(e) = core::reset(&core_ctx) {
                            println!("{:?}", e);
                            break 'running;
                        };
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
                                    Err(e) => {
                                        println!("{:?}", e);
                                        break 'running;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if !_pause_request_new_frames {
                match core::run(&core_ctx) {
                    Ok(..) => {
                        if let Some((av_ctx, _)) = &mut retro_av {
                            if av_ctx.get_new_frame().is_err() {
                                break 'running;
                            }
                        }
                    }
                    Err(e) => {
                        println!("{:?}", e);
                        break 'running;
                    }
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
                            if _pause_request_new_frames {
                                if let Ok(mut controller) = controller_ctx.lock() {
                                    controller.pause_thread_events();
                                    _pause_request_new_frames = false
                                }
                            } else {
                                if let Ok(mut controller) = controller_ctx.lock() {
                                    controller.resume_thread_events();
                                    _pause_request_new_frames = true
                                }
                            }
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

        match controller_ctx.lock() {
            Ok(mut controller) => {
                controller.resume_thread_events();
            }
            Err(..) => {}
        }
    });
}
