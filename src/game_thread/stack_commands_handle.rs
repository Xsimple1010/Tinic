#![allow(unused_imports)]
use crate::channel::{ChannelNotify, ThreadChannel};
use crate::thread_stack::game_stack::{
    GameStack,
    GameStackCommand::{
        DeviceConnected, DisableFullScreen, EnableFullScreen, LoadGame, LoadState, Pause, Quit,
        Reset, Resume, SaveState,
    },
};
use crate::thread_stack::main_stack::MainStackCommand::{
    GameLoaded, GameStateSaved, SaveStateLoaded,
};
use crate::thread_stack::model_stack::{ModelStackManager, RetroStackFn};
use retro_ab::{
    core::RetroEnvCallbacks,
    erro_handle::ErroHandle,
    paths::Paths,
    retro_ab::RetroAB,
    retro_sys::{retro_hw_context_type, retro_log_level},
};
use retro_ab_av::{
    audio_sample_batch_callback, audio_sample_callback, get_proc_address, retro_av::RetroAvCtx,
    video_refresh_callback, EventPump,
};
use retro_ab_gamepad::{
    input_poll_callback, input_state_callback, rumble_callback, RetroAbController,
};
use std::sync::{Arc, Mutex};

fn teste() {}

fn create_retro_contexts(
    core_path: String,
    rom_path: String,
    paths: Paths,
) -> Result<(RetroAB, (RetroAvCtx, EventPump)), ErroHandle> {
    let callbacks = RetroEnvCallbacks {
        audio_sample_batch_callback,
        audio_sample_callback,
        input_poll_callback,
        input_state_callback,
        video_refresh_callback,
        rumble_callback,
        get_proc_address,
        context_destroy: teste,
        context_reset: teste,
    };

    let retro_ab = RetroAB::new(
        &core_path,
        paths,
        callbacks,
        retro_hw_context_type::RETRO_HW_CONTEXT_OPENGL_CORE,
    )?;

    if retro_ab.core().load_game(&rom_path)? {
        let av = RetroAvCtx::new(retro_ab.core().av_info.clone())?;
        return Ok((retro_ab, av));
    }

    Err(ErroHandle {
        level: retro_log_level::RETRO_LOG_ERROR,
        message: "nao foi possível criar uma instancia retro_ab".to_string(),
    })
}

pub fn stack_commands_handle(
    channel_notify: &ChannelNotify,
    core_ctx: &mut Option<RetroAB>,
    controller_ctx: &Arc<Mutex<RetroAbController>>,
    av_ctx: &mut Option<(RetroAvCtx, EventPump)>,
    pause_request_new_frames: &mut bool,
    use_full_screen: &mut bool,
) -> bool {
    let mut need_stop = false;

    for cmd in channel_notify.read_game_stack() {
        match cmd {
            Quit => {
                need_stop = true;
                break;
            }
            LoadGame(core_path, rom_path, paths) => {
                if core_ctx.is_some() {
                    need_stop = true;
                    break;
                }

                match create_retro_contexts(core_path, rom_path, paths) {
                    Ok((retro_ab, av)) => {
                        if let Ok(mut ctr) = controller_ctx.lock() {
                            ctr.stop_thread_events();

                            //Pode ser que essa não seja a primeira vez que um game está sendo
                            //executada. Então por garantia o ideal é conectar todos os devices
                            //que ja existem agora! E depois os próximos conforme forem chegando.
                            for device in ctr.get_list() {
                                //-1 é uma porta invalida
                                if device.retro_port > -1 {
                                    let _ = retro_ab.core().connect_controller(
                                        device.retro_port as u32,
                                        device.retro_type,
                                    );
                                }
                            }
                        }

                        channel_notify
                            .notify_main_stack(GameLoaded(Some(retro_ab.core().options.clone())));

                        core_ctx.replace(retro_ab);
                        av_ctx.replace(av);
                    }
                    Err(e) => {
                        println!("{:?}", e);
                        channel_notify.notify_main_stack(GameLoaded(None));
                        need_stop = true;
                        break;
                    }
                }
            }
            LoadState(slot) => {
                if let Some(ctx) = core_ctx {
                    match ctx.core().load_state(slot) {
                        Ok(_) => {
                            channel_notify.notify_main_stack(SaveStateLoaded(true));
                        }
                        Err(e) => {
                            println!("{:?}", e);
                            channel_notify.notify_main_stack(SaveStateLoaded(false));
                            need_stop = true;
                            break;
                        }
                    }
                }
            }
            SaveState(slot) => {
                if let Some(ctx) = core_ctx {
                    let _ = ctx.core().save_state(slot);
                    channel_notify
                        .notify_main_stack(GameStateSaved("teste".to_owned(), "img".to_owned()));
                }
            }
            Pause => {
                //habilita a thread de eventos novamente
                if let Ok(mut controller) = controller_ctx.lock() {
                    let _ = controller.resume_thread_events();
                    *pause_request_new_frames = true
                }
            }
            Resume => {
                //como o game estará em execução é necessário interromper a thread de eventos
                if let Ok(mut controller) = controller_ctx.lock() {
                    controller.stop_thread_events();
                    *pause_request_new_frames = false
                }
            }
            Reset => {
                if let Some(ctx) = &core_ctx {
                    if let Err(e) = ctx.core().reset() {
                        println!("{:?}", e);
                        need_stop = true;
                        break;
                    };
                };
            }
            DeviceConnected(device) => {
                if let Some(ctx) = core_ctx {
                    let _ = ctx
                        .core()
                        .connect_controller(device.retro_port as u32, device.retro_type);
                }
            }

            //VIDEO
            EnableFullScreen => {
                if let Some((av_ctx, _)) = av_ctx {
                    av_ctx.video.full_screen();
                    *use_full_screen = true;
                }
            }
            DisableFullScreen => {
                if let Some((av_ctx, _)) = av_ctx {
                    av_ctx.video.disable_full_screen();
                    *use_full_screen = false;
                }
            }
        }
    }

    need_stop
}
