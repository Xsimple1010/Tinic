#![allow(unused_imports)]
use crate::channel::ChannelNotify;
use crate::thread_stack::game_stack::GameStackCommand::{
    DeviceConnected, DisableFullScreen, EnableFullScreen, LoadGame, LoadState, Pause, Reset,
    Resume, SaveState,
};
use crate::thread_stack::main_stack::MainStackCommand::{
    GameLoaded, GameStateSaved, SaveStateLoaded,
};
use generics::constants::INVALID_CONTROLLER_PORT;
use generics::erro_handle::ErroHandle;
use generics::retro_paths::RetroPaths;
use libretro_sys::binding_libretro::{
    retro_hw_context_type::RETRO_HW_CONTEXT_OPENGL_CORE, retro_log_level::RETRO_LOG_ERROR,
};
use retro_av::{
    audio_sample_batch_callback, audio_sample_callback, context_destroy, context_reset,
    get_proc_address, video_refresh_callback, EventPump, RetroAv,
};
use retro_controllers::devices_manager::Device;
use retro_controllers::{
    input_poll_callback, input_state_callback, rumble_callback, RetroController,
};
use retro_core::{RetroCore, RetroEnvCallbacks};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

fn create_retro_contexts(
    core_path: String,
    rom_path: String,
    paths: RetroPaths,
) -> Result<(RetroCore, (RetroAv, EventPump)), ErroHandle> {
    let callbacks = RetroEnvCallbacks {
        input_poll_callback,
        input_state_callback,
        rumble_callback,
        audio_sample_batch_callback,
        audio_sample_callback,
        video_refresh_callback,
        get_proc_address,
        context_destroy,
        context_reset,
    };

    let retro_core = RetroCore::new(&core_path, paths, callbacks, RETRO_HW_CONTEXT_OPENGL_CORE)?;
    let av_info = retro_core.core.load_game(&rom_path)?;
    let retro_av = RetroAv::new(av_info)?;

    Ok((retro_core, retro_av))
}

pub fn stack_commands_handle(
    channel_notify: &ChannelNotify,
    retro_core: &mut Option<RetroCore>,
    controller_ctx: &Arc<Mutex<RetroController>>,
    retro_av: &mut Option<(RetroAv, EventPump)>,
    pause_request_new_frames: &mut bool,
    use_full_screen: &mut bool,
) -> bool {
    let mut need_stop = false;

    for cmd in channel_notify.read_game_stack() {
        match cmd {
            LoadGame(core_path, rom_path, paths) => {
                if retro_core.is_some() {
                    need_stop = true;
                    break;
                }

                match create_retro_contexts(core_path, rom_path, paths) {
                    Ok((new_retro_core, new_retro_av)) => {
                        if let Ok(mut ctr) = controller_ctx.lock() {
                            ctr.stop_thread_events();

                            //Pode ser que essa não seja a primeira vez que um game está sendo
                            //executada. Então por garantia o ideal é conectar todos os devices
                            //que ja existem agora! E depois os próximos conforme forem chegando.
                            for gamepad in ctr.get_list() {
                                channel_notify.notify_game_stack(DeviceConnected(
                                    Device::from_gamepad(&gamepad),
                                ))
                            }
                        }

                        channel_notify.notify_main_stack(GameLoaded(Some(
                            new_retro_core.core.options.clone(),
                        )));

                        retro_core.replace(new_retro_core);
                        retro_av.replace(new_retro_av);
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
                if let Some(ctx) = retro_core {
                    match ctx.core.load_state(slot) {
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
                if let Some(ctx) = retro_core {
                    match ctx.core.save_state(slot) {
                        Ok(saved_path) => {
                            let mut img_path: PathBuf = PathBuf::new();

                            if let Some((av, _)) = retro_av {
                                if let Ok(path) = av
                                    .video
                                    .print_screen(saved_path.parent().unwrap(), &slot.to_string())
                                {
                                    img_path = path;
                                };
                            }

                            channel_notify
                                .notify_main_stack(GameStateSaved(Some((saved_path, img_path))));
                        }

                        Err(e) => {
                            println!("{:?}", e);
                            channel_notify.notify_main_stack(GameStateSaved(None));
                            need_stop = true;
                        }
                    }
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
                if let Some(ctx) = &retro_core {
                    if let Err(e) = ctx.core.reset() {
                        println!("{:?}", e);
                        need_stop = true;
                        break;
                    };
                };
            }
            DeviceConnected(device) => {
                if let Some(ctx) = retro_core {
                    let _ = ctx
                        .core
                        .connect_controller(device.retro_port, device.retro_type);
                }
            }
            //VIDEO
            EnableFullScreen => {
                if let Some((retro_av, _)) = retro_av {
                    retro_av.video.enable_full_screen();
                    *use_full_screen = true;
                }
            }
            DisableFullScreen => {
                if let Some((retro_av, _)) = retro_av {
                    retro_av.video.disable_full_screen();
                    *use_full_screen = false;
                }
            }
        }
    }

    need_stop
}
