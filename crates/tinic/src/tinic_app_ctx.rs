use std::{path::Path, sync::Arc};

use generics::{
    constants::SAVE_IMAGE_EXTENSION_FILE, erro_handle::ErroHandle, retro_paths::RetroPaths,
};
use libretro_sys::binding_libretro::retro_hw_context_type;
use retro_av::RetroAv;
use retro_controllers::{devices_manager::Device, RetroController};
use retro_core::{graphic_api::GraphicApi, RetroCore, RetroCoreIns, RetroEnvCallbacks};
use winit::{event_loop::ActiveEventLoop, window::Fullscreen};

pub struct TinicAppCtx {
    retro_av: RetroAv,
    retro_core: RetroCoreIns,
    current_full_screen_mode: Fullscreen,
    can_request_new_frames: bool,
    controller: Arc<RetroController>,
}

impl TinicAppCtx {
    pub fn new(
        retro_paths: RetroPaths,
        core_path: String,
        rom_path: String,
        controller: Arc<RetroController>,
    ) -> Result<Self, ErroHandle> {
        let retro_av = RetroAv::new()?;
        let (video_cb, audio_cb) = retro_av.get_core_cb();

        let callbacks = RetroEnvCallbacks {
            audio: Box::new(audio_cb),
            video: Box::new(video_cb),
            controller: Box::new(controller.get_core_cb()),
        };

        let retro_core = RetroCore::new(
            &core_path,
            retro_paths.clone(),
            callbacks,
            GraphicApi::with(retro_hw_context_type::RETRO_HW_CONTEXT_OPENGL_CORE),
        )?;

        retro_core.load_game(&rom_path)?;

        for gamepad in controller.get_list()? {
            retro_core.connect_controller(gamepad.retro_port, gamepad.retro_type)?;
        }

        Ok(Self {
            retro_av,
            retro_core,
            controller,
            can_request_new_frames: true,
            current_full_screen_mode: Fullscreen::Borderless(None),
        })
    }

    pub fn create_window(&mut self, event_loop: &ActiveEventLoop) -> Result<(), ErroHandle> {
        self.retro_av
            .build_window(&self.retro_core.av_info.clone(), event_loop)?;
        self.controller.stop_thread_events();

        Ok(())
    }

    pub fn suspend_window(&mut self) {
        self.retro_av.destroy_window();
        self.controller.resume_thread_events();
    }

    pub fn close_retro_ctx(&self) -> Result<(), ErroHandle> {
        self.retro_core.de_init()?;
        self.controller.resume_thread_events();

        Ok(())
    }

    pub fn redraw_request(&self) -> Result<(), ErroHandle> {
        self.retro_av.redraw_request()
    }

    pub fn draw_new_frame(&mut self) -> Result<(), ErroHandle> {
        if self.retro_av.sync() {
            if !self.can_request_new_frames {
                return Ok(());
            }

            self.retro_core.run()?;
            self.retro_av.get_new_frame()?
        }

        Ok(())
    }

    pub fn reset(&self) -> Result<(), ErroHandle> {
        self.retro_core.reset()
    }

    pub fn save_state(&self, slot: usize) -> Result<(), ErroHandle> {
        let save_path = self.retro_core.save_state(slot)?;

        let mut img_path = save_path.clone();
        img_path.set_extension(SAVE_IMAGE_EXTENSION_FILE);

        self.print_screen(&img_path)?;
        Ok(())
    }

    pub fn load_state(&self, slot: usize) -> Result<(), ErroHandle> {
        self.retro_core.load_state(slot)?;
        Ok(())
    }

    pub fn print_screen(&self, out_path: &Path) -> Result<(), ErroHandle> {
        self.retro_av.print_screen(out_path)
    }

    pub fn toggle_full_screen_mode(&mut self) -> Result<(), ErroHandle> {
        self.retro_av
            .set_full_screen(self.current_full_screen_mode.clone())
    }

    pub fn toggle_can_request_new_frames(&mut self) {
        if self.can_request_new_frames {
            self.controller.resume_thread_events();
            self.can_request_new_frames = false;
        } else {
            self.controller.stop_thread_events();
            self.can_request_new_frames = true;
        }
    }

    pub fn connect_controller(&self, device: Device) -> Result<(), ErroHandle> {
        self.retro_core
            .connect_controller(device.retro_port, device.retro_type)
    }
}
