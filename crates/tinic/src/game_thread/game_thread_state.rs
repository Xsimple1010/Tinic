use crate::channel::ChannelNotify;
use crate::thread_stack::game_stack::GameStackCommand::DeviceConnected;
use crate::thread_stack::main_stack::MainStackCommand::{
    self, GameLoaded, GameStateSaved, SaveStateLoaded,
};
use generics::constants::SAVE_IMAGE_EXTENSION_FILE;
use generics::{constants::THREAD_SLEEP_TIME, erro_handle::ErroHandle, retro_paths::RetroPaths};
use libretro_sys::{
    binding_libretro::retro_hw_context_type::RETRO_HW_CONTEXT_OPENGL_CORE,
    binding_libretro::retro_log_level,
};
use retro_av::{EventPump, RetroAv};
use retro_controllers::{
    devices_manager::Device, input_poll_callback, input_state_callback, rumble_callback,
    RetroController,
};
use retro_core::{
    graphic_api::GraphicApi, option_manager::OptionManager, RetroCore, RetroCoreIns,
    RetroEnvCallbacks,
};
use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex, MutexGuard,
    },
    thread,
    time::Duration,
};

pub struct ThreadState {
    pub channel_notify: ChannelNotify,
    pub is_running: Arc<AtomicBool>,
    pub pause_request_new_frames: bool,
    pub use_full_screen_mode: bool,
    pub event_pump: Option<EventPump>,
    controller_ctx: Arc<Mutex<RetroController>>,
    retro_core: Option<RetroCoreIns>,
    retro_av: Option<RetroAv>,
}

impl ThreadState {
    //a thread main sera notificada do encerramento em impl Drop
    pub fn quit(&self) {
        self.is_running.store(false, Ordering::SeqCst);
    }

    pub fn load_game(
        &mut self,
        core_path: String,
        rom_path: String,
        paths: RetroPaths,
    ) -> Result<(), ErroHandle> {
        if self.retro_core.is_some() {
            return Ok(());
        }

        match self.create_retro_contexts(core_path, rom_path, paths) {
            Ok(option) => self
                .channel_notify
                .notify_main_stack(GameLoaded(Some(option))),
            Err(e) => {
                self.channel_notify.notify_main_stack(GameLoaded(None));
                return Err(e);
            }
        };

        if let Ok(mut ctr) = self.controller_ctx.lock() {
            ctr.stop_thread_events();

            //Pode ser que essa não seja a primeira vez que um game está sendo
            //executada. Então por garantia o ideal é conectar todos os devices
            //que ja existem agora! E depois os próximos conforme forem chegando.
            for gamepad in ctr.get_list()? {
                self.channel_notify
                    .notify_game_stack(DeviceConnected(Device::from_gamepad(&gamepad)))
            }
        }

        Ok(())
    }

    pub fn load_state(&mut self, slot: usize) -> Result<(), ErroHandle> {
        match self.try_get_retro_core_ctx()?.load_state(slot) {
            Ok(_) => {
                self.channel_notify.notify_main_stack(SaveStateLoaded(true));

                Ok(())
            }
            Err(e) => {
                self.channel_notify
                    .notify_main_stack(SaveStateLoaded(false));

                Err(e)
            }
        }
    }

    pub fn save_state(&mut self, slot: usize) -> Result<(), ErroHandle> {
        let retro_av = self.try_get_retro_av_ctx()?;
        let retro_core = self.try_get_retro_core_ctx()?;

        match retro_core.save_state(slot) {
            Ok(saved_path) => {
                let mut img_path: PathBuf = saved_path.clone();
                img_path.set_extension(SAVE_IMAGE_EXTENSION_FILE);

                if let Ok(path) = retro_av.video.print_screen(&img_path, &retro_core.av_info) {
                    img_path = path;
                };

                self.channel_notify
                    .notify_main_stack(GameStateSaved(Some((saved_path, img_path))));

                Ok(())
            }

            Err(e) => {
                self.channel_notify.notify_main_stack(GameStateSaved(None));
                Err(e)
            }
        }
    }

    pub fn pause(&mut self) -> Result<(), ErroHandle> {
        self.try_get_controller_ctx()?.resume_thread_events()?;
        self.pause_request_new_frames = true;

        Ok(())
    }

    pub fn resume(&mut self) -> Result<(), ErroHandle> {
        self.try_get_controller_ctx()?.stop_thread_events();
        self.pause_request_new_frames = false;

        Ok(())
    }

    pub fn reset(&self) -> Result<(), ErroHandle> {
        self.try_get_retro_core_ctx()?.reset()?;

        Ok(())
    }

    pub fn enable_full_screen(&mut self) -> Result<(), ErroHandle> {
        self.try_get_retro_av_ctx()?.video.enable_full_screen();
        self.use_full_screen_mode = true;

        Ok(())
    }

    pub fn disable_full_screen(&mut self) -> Result<(), ErroHandle> {
        self.try_get_retro_av_ctx()?.video.disable_full_screen();
        self.use_full_screen_mode = false;

        Ok(())
    }

    pub fn connect_device(&mut self, device: Device) -> Result<(), ErroHandle> {
        self.try_get_retro_core_ctx()?
            .connect_controller(device.retro_port, device.retro_type)?;

        Ok(())
    }
}

impl ThreadState {
    pub fn new(
        channel_notify: ChannelNotify,
        controller_ctx: Arc<Mutex<RetroController>>,
        is_running: Arc<AtomicBool>,
    ) -> Self {
        Self {
            channel_notify,
            controller_ctx,
            is_running,
            pause_request_new_frames: false,
            use_full_screen_mode: false,
            retro_av: None,
            retro_core: None,
            event_pump: None,
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }

    fn try_get_retro_core_ctx(&self) -> Result<RetroCoreIns, ErroHandle> {
        match &self.retro_core {
            Some(retro_core) => Ok(retro_core.clone()),
            None => Err(ErroHandle {
                level: retro_log_level::RETRO_LOG_ERROR,
                message: "erro ao tentar recuperar retro_core".to_string(),
            }),
        }
    }

    pub fn try_get_retro_av_ctx(&self) -> Result<&RetroAv, ErroHandle> {
        match &self.retro_av {
            Some(retro_av) => Ok(retro_av),
            None => Err(ErroHandle {
                level: retro_log_level::RETRO_LOG_ERROR,
                message: "erro ao tentar recuperar retro_av".to_string(),
            }),
        }
    }

    fn try_get_controller_ctx(&mut self) -> Result<MutexGuard<'_, RetroController>, ErroHandle> {
        match self.controller_ctx.lock() {
            Ok(guard) => Ok(guard),
            Err(e) => Err(ErroHandle {
                level: retro_log_level::RETRO_LOG_ERROR,
                message: e.to_string(),
            }),
        }
    }

    fn create_retro_contexts(
        &mut self,
        core_path: String,
        rom_path: String,
        paths: RetroPaths,
    ) -> Result<Arc<OptionManager>, ErroHandle> {
        let mut retro_av = RetroAv::new()?;
        let (video_cb, audio_cb) = retro_av.get_core_cb();

        let callbacks = RetroEnvCallbacks {
            input_poll_callback,
            input_state_callback,
            rumble_callback,
            video: Box::new(video_cb),
            audio: Box::new(audio_cb),
        };

        let retro_core = RetroCore::new(
            &core_path,
            paths,
            callbacks,
            GraphicApi::with(RETRO_HW_CONTEXT_OPENGL_CORE),
        )?;

        let av_info = retro_core.load_game(&rom_path)?;
        let pump_event = retro_av.build_window(&av_info)?;

        let op_manager = retro_core.options.clone();

        self.retro_core.replace(retro_core);
        self.event_pump.replace(pump_event);
        self.retro_av.replace(retro_av);

        Ok(op_manager)
    }

    pub fn try_render_frame(&mut self) -> Result<(), ErroHandle> {
        if let Some(retro_core) = &self.retro_core {
            if let Some(retro_av) = &mut self.retro_av {
                if !retro_av.sync() || self.pause_request_new_frames {
                    return Ok(());
                }

                // Pede para core gerar novos buffers de video e audio
                retro_core.run()?;
                // Exibe os buffers gerados pelo core
                retro_av.get_new_frame();
            }
        } else {
            //WITHOUT THIS, WI HAVE A HIGH CPU UTILIZATION!
            thread::sleep(Duration::from_millis(THREAD_SLEEP_TIME));
        }

        Ok(())
    }
}

impl Drop for ThreadState {
    fn drop(&mut self) {
        self.channel_notify.clear_game_stack();

        //Para garantir que essa thread será fechada dando a posse da leitura dos inputs para a
        //thread de inputs novamente.
        if let Ok(mut ctr) = self.controller_ctx.lock() {
            let _ = ctr.resume_thread_events();
        }

        //retro-core nao implementa drop então chamar de_init() depois de terminar de usar é necessário.
        if let Some(core) = self.retro_core.take() {
            let _ = core.de_init();
        }

        self.is_running.store(false, Ordering::Relaxed);

        self.channel_notify
            .notify_main_stack(MainStackCommand::QuitSusses(true));
    }
}
