use crate::retro_stack::{
    RetroStack,
    StackCommand::{GamepadConnected, LoadGame, LoadState, Pause, Quit, Reset, Resume, SaveState},
};
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
    stack: &Arc<RetroStack>,
    core_ctx: &mut Option<RetroAB>,
    controller_ctx: &Arc<Mutex<RetroAbController>>,
    av_ctx: &mut Option<(RetroAvCtx, EventPump)>,
    pause_request_new_frames: &mut bool,
) -> bool {
    let mut need_stop = false;

    for cmd in stack.read() {
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
                        }

                        core_ctx.replace(retro_ab);
                        av_ctx.replace(av);
                    }
                    Err(e) => {
                        println!("{:?}", e);
                        need_stop = true;
                        break;
                    }
                }
            }
            LoadState => {} //ainda e preciso adicionar isso em retro_ab
            SaveState => {} //ainda e preciso adicionar isso em retro_ab
            Pause => {
                //habilita a thread de eventos novamente
                if let Ok(mut controller) = controller_ctx.lock() {
                    let _ = controller.resume_thread_events();
                    *pause_request_new_frames = true
                }
            }
            Resume => {
                //como a Rom estará em execução é necessário interromper a thread de eventos
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
            GamepadConnected(gamepad) => {
                if let Some(ctx) = core_ctx {
                    let _ = ctx
                        .core()
                        .connect_controller(gamepad.retro_port as u32, gamepad.retro_type);
                }
            }
        }
    }

    need_stop
}
