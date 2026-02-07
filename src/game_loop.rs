use crate::retro_stack::{RetroStack, StackCommand};
use retro_ab::core::{self, RetroContext};
use retro_ab_av::{
    context::{RetroAvCtx, RetroAvEvents},
    Event, Key, KeyEvent, NamedKey, WindowEvent,
};
use retro_ab_gamepad::context::GamepadContext;
use std::{
    sync::{Arc, Mutex},
    thread,
};

fn game_window_handle(
    av_ctx: &mut Option<RetroAvCtx>,
    core_ctx: &mut Option<Arc<RetroContext>>,
    av_events: &mut RetroAvEvents,
) {
    if let Some(core) = &core_ctx {
        core::run(&core).unwrap();
    }
    if let Some(av) = &av_ctx {
        av.request_redraw();
    }
    println!("game_window_handle");

    av_events.pump(|event, window_target| match event {
        Event::Resumed => {
            if let Some(av) = av_ctx {
                av.resume(window_target);
            }
        }
        Event::Suspended => {
            if let Some(av) = av_ctx {
                av.suspended();
            }
        }
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: Key::Named(NamedKey::Escape),
                        ..
                    },
                ..
            } => {
                window_target.exit();
                println!("exist");
            }
            WindowEvent::Destroyed => {
                if let Some(core) = core_ctx.take() {
                    let _ = core::de_init(core);
                }
                //drop
                av_ctx.take();
            }
            WindowEvent::RedrawRequested => {
                if let Some(av) = av_ctx {
                    let _ = av.get_new_frame();
                }
            }
            _ => (),
        },
        _ => (),
    });
}

//TODO: criar uma callback para avisar a interface de possíveis erros
pub fn init_game_loop(
    // gamepads: Arc<Mutex<Vec<RetroGamePad>>>,
    controller_ctx: Arc<Mutex<GamepadContext>>,
    stack: Arc<RetroStack>,
) {
    thread::spawn(move || {
        let mut running = true;

        let mut av_events = RetroAvEvents::new().unwrap();
        let mut core_ctx = None;
        let mut av_ctx = None;

        while running {
            for cmd in stack.read() {
                match cmd {
                    StackCommand::GameQuit => {
                        if let Some(c_ctx) = core_ctx.take() {
                            core::de_init(c_ctx);
                        }

                        //drop
                        av_ctx.take();
                    }
                    StackCommand::LoadCore(path, paths, callbacks) => {
                        match core::load(&path, paths, callbacks) {
                            Ok(ctx) => match core::init(&ctx) {
                                Ok(..) => {
                                    core_ctx.replace(ctx.clone());
                                }
                                Err(e) => {
                                    println!("{:?}", e);
                                    running = true;
                                    break;
                                }
                            },
                            Err(e) => {
                                println!("{:?}", e);
                                running = true;
                                break;
                            }
                        };
                    }
                    StackCommand::LoadGame(path) => {
                        match &core_ctx {
                            Some(ctx) => {
                                match core::load_game(&ctx, path.as_str()) {
                                    Ok(loaded) => {
                                        if loaded {
                                            if let Ok(mut controller) = controller_ctx.lock() {
                                                controller.pause_thread_events();
                                            }

                                            av_ctx.replace(RetroAvCtx::new(
                                                ctx.core.av_info.clone(),
                                                &av_events,
                                            ));
                                        }
                                    }
                                    Err(e) => {
                                        println!("{:?}", e);
                                        running = true;
                                        break;
                                    }
                                };
                            }
                            None => {
                                println!("core context nao existe!");
                            }
                        };
                    }
                    _ => {}
                }
            }

            if av_ctx.is_some() && core_ctx.is_some() {
                game_window_handle(&mut av_ctx, &mut core_ctx, &mut av_events);
            }
        }
    });
}
