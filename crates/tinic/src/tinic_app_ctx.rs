use std::{path::Path, sync::Arc};

use generics::{
    constants::SAVE_IMAGE_EXTENSION_FILE, error_handle::ErrorHandle, retro_paths::RetroPaths,
};
use libretro_sys::binding_libretro::retro_hw_context_type;
use retro_av::RetroAv;
use retro_controllers::{RetroController, RetroGamePad};
use retro_core::{RetroCore, RetroCoreIns, RetroEnvCallbacks, graphic_api::GraphicApi};
use winit::keyboard::PhysicalKey;
use winit::{event_loop::ActiveEventLoop, window::Fullscreen};

pub struct TinicGameCtx {
    retro_av: RetroAv,
    retro_core: RetroCoreIns,
    current_full_screen_mode: Fullscreen,
    can_request_new_frames: bool,
    pub controller: Arc<RetroController>,
}

impl TinicGameCtx {
    pub fn new(
        retro_paths: RetroPaths,
        core_path: String,
        rom_path: String,
        controller: Arc<RetroController>,
    ) -> Result<Self, ErrorHandle> {
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

        let gamepads = controller.get_list()?;

        if gamepads.len().eq(&0) {
            let keyboard = controller.active_keyboard();
            retro_core.connect_controller(keyboard.retro_port, keyboard.retro_type)?;
        } else {
            for gamepad in gamepads {
                retro_core.connect_controller(gamepad.retro_port, gamepad.retro_type)?;
            }
        }

        Ok(Self {
            retro_av,
            retro_core,
            controller,
            can_request_new_frames: true,
            current_full_screen_mode: Fullscreen::Borderless(None),
        })
    }

    pub fn toggle_keyboard_usage(&self) -> Result<(), ErrorHandle> {
        if self.controller.is_using_keyboard() {
            self.disable_keyboard();
            Ok(())
        } else {
            self.active_keyboard()
        }
    }

    pub fn disable_keyboard(&self) {
        self.controller.disable_keyboard()
    }
    pub fn active_keyboard(&self) -> Result<(), ErrorHandle> {
        let keyboard = self.controller.active_keyboard();
        self.retro_core
            .connect_controller(keyboard.retro_port, keyboard.retro_type)
    }
    pub fn update_keyboard_state(&self, native: PhysicalKey, pressed: bool) {
        self.controller.update_keyboard(native, pressed)
    }

    pub fn create_window(&mut self, event_loop: &ActiveEventLoop) -> Result<(), ErrorHandle> {
        self.retro_av
            .build_window(&self.retro_core.av_info.clone(), event_loop)?;
        self.controller.stop_thread_events();

        Ok(())
    }

    pub fn suspend_window(&mut self) {
        self.retro_av.suspend_window();
        self.controller.resume_thread_events();
    }

    pub fn destroy_retro_ctx(&self) -> Result<(), ErrorHandle> {
        self.retro_core.de_init()?;
        self.controller.resume_thread_events();
        self.retro_av.destroy();

        Ok(())
    }

    pub fn redraw_request(&self) -> Result<(), ErrorHandle> {
        self.retro_av.redraw_request()
    }

    pub fn draw_new_frame(&mut self) -> Result<(), ErrorHandle> {
        if !self.can_request_new_frames {
            return Ok(());
        }

        self.retro_av.prepare_to_sync(&self.retro_core.av_info)?;
        self.retro_core.run()?;
        self.retro_av.sync_now()?;

        Ok(())
    }

    pub fn reset(&self) -> Result<(), ErrorHandle> {
        self.retro_core.reset()
    }

    pub fn save_state(&self, slot: usize) -> Result<(), ErrorHandle> {
        let save_path = self.retro_core.save_state(slot)?;

        let mut img_path = save_path.clone();
        img_path.set_extension(SAVE_IMAGE_EXTENSION_FILE);

        self.print_screen(&img_path)?;
        Ok(())
    }

    pub fn load_state(&self, slot: usize) -> Result<(), ErrorHandle> {
        self.retro_core.load_state(slot)?;
        Ok(())
    }

    pub fn print_screen(&self, out_path: &Path) -> Result<(), ErrorHandle> {
        self.retro_av
            .print_screen(out_path, &self.retro_core.av_info)
    }

    pub fn toggle_full_screen_mode(&mut self) -> Result<(), ErrorHandle> {
        match self.current_full_screen_mode {
            Fullscreen::Borderless(None) => self.disable_full_screen_mode(),
            _ => self.enable_full_screen_mode(),
        }
    }

    pub fn enable_full_screen_mode(&mut self) -> Result<(), ErrorHandle> {
        self.current_full_screen_mode = Fullscreen::Borderless(None);
        self.retro_av
            .set_full_screen(self.current_full_screen_mode.clone())
    }

    pub fn disable_full_screen_mode(&mut self) -> Result<(), ErrorHandle> {
        self.retro_av
            .set_full_screen(self.current_full_screen_mode.clone())
    }

    pub fn toggle_can_request_new_frames(&mut self) {
        if self.can_request_new_frames {
            self.pause();
        } else {
            self.resume();
        }
    }

    pub fn pause(&mut self) {
        self.controller.resume_thread_events();
        self.can_request_new_frames = false;
    }

    pub fn resume(&mut self) {
        self.controller.stop_thread_events();
        self.can_request_new_frames = true;
    }

    pub fn connect_controller(&self, device: RetroGamePad) -> Result<(), ErrorHandle> {
        self.retro_core
            .connect_controller(device.retro_port, device.retro_type)
    }
}
