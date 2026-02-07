use generics::{
    constants::SAVE_IMAGE_EXTENSION_FILE, erro_handle::ErroHandle, retro_paths::RetroPaths,
};
use libretro_sys::binding_libretro::retro_hw_context_type;
use retro_av::RetroAv;
use retro_controllers::RetroController;
use retro_core::{graphic_api::GraphicApi, RetroCore, RetroCoreIns, RetroEnvCallbacks};
use std::{path::PathBuf, sync::Arc};
use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop, keyboard::{KeyCode, PhysicalKey}, monitor::VideoModeHandle, window::{Fullscreen, WindowId}
};

pub struct TinicApp {
    retro_paths: RetroPaths,
    core_path: String,
    rom_path: String,
    controller: Arc<RetroController>,
    retro_av: RetroAv,
    retro_core: Option<RetroCoreIns>,
    current_full_creen_mode: Fullscreen,
}

impl TinicApp {
    pub fn new(
        retro_paths: RetroPaths,
        core_path: String,
        rom_path: String,
        controller: Arc<RetroController>,
    ) -> Self {
        Self {
            retro_paths,
            core_path,
            rom_path,
            retro_av: RetroAv::new().unwrap(),
            retro_core: None,
            controller,
            current_full_creen_mode: Fullscreen::Exclusive()
        }
    }
}

impl Drop for TinicApp {
    fn drop(&mut self) {
        if let Some(retro_core) = &mut self.retro_core {
            let _ = retro_core.de_init();
        }
    }
}

impl ApplicationHandler for TinicApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Err(e) = self.create_retro_context(event_loop) {
            println!("{:?}", e);
            event_loop.exit();
        }
    }

    fn suspended(&mut self, _: &ActiveEventLoop) {
        self.destroy_window();
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Err(e) = self.retro_av.redraw_request() {
            println!("{:?}", e);
            event_loop.exit();
        }
    }

    fn exiting(&mut self, _: &ActiveEventLoop) {
        if let Some(retro_core) = &mut self.retro_core {
            let _ = retro_core.de_init();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let result: Result<(), ErroHandle> = match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
                Ok(())
            }
            WindowEvent::RedrawRequested => self.redraw_request(),
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => match event.physical_key {
                PhysicalKey::Code(KeyCode::F5) => self.reset(),
                PhysicalKey::Code(KeyCode::F1) => self.save_state(1),
                PhysicalKey::Code(KeyCode::F2) => self.load_state(1),
                PhysicalKey::Code(KeyCode::F11) => self.toggle_full_screen_mode(),
                _ => Ok(()),
            },
            _ => Ok(()),
        };

        if let Err(e) = result {
            println!("{:?}", e);
            event_loop.exit();
        }
    }
}

impl TinicApp {
    pub fn create_retro_context(&mut self, event_loop: &ActiveEventLoop) -> Result<(), ErroHandle> {
        let (video_cb, audio_cb) = self.retro_av.get_core_cb();

        let callbacks = RetroEnvCallbacks {
            audio: Box::new(audio_cb),
            video: Box::new(video_cb),
            controller: Box::new(self.controller.get_core_cb()),
        };

        let retro_core = RetroCore::new(
            &self.core_path,
            self.retro_paths.clone(),
            callbacks,
            GraphicApi::with(retro_hw_context_type::RETRO_HW_CONTEXT_OPENGL_CORE),
        )?;

        let av_info = retro_core.load_game(&self.rom_path)?;
        self.retro_core.replace(retro_core);
        self.retro_av.build_window(&av_info, event_loop)?;

        Ok(())
    }

    fn destroy_window(&mut self) {
        self.retro_av.destroy_window();
    }

    pub fn redraw_request(&mut self) -> Result<(), ErroHandle> {
        if let Some(retro_core) = &mut self.retro_core {
            if self.retro_av.sync() {
                retro_core.run()?;
                self.retro_av.get_new_frame()?
            }
        }

        Ok(())
    }

    pub fn reset(&self) -> Result<(), ErroHandle> {
        self.retro_core.as_ref().unwrap().reset()
    }

    fn save_state(&self, slot: usize) -> Result<(), ErroHandle> {
        let save_path = self.retro_core.as_ref().unwrap().save_state(slot)?;

        let mut img_path = save_path.clone();
        img_path.set_extension(SAVE_IMAGE_EXTENSION_FILE);

        self.print_screen(&img_path)?;
        Ok(())
    }

    fn load_state(&self, slot: usize) -> Result<(), ErroHandle> {
        self.retro_core.as_ref().unwrap().load_state(slot)?;
        Ok(())
    }

    fn print_screen(&self, out_path: &PathBuf) -> Result<(), ErroHandle> {
        self.retro_av.print_screen(out_path)
    }

    fn toggle_full_screen_mode(&mut self) -> Result<(), ErroHandle> {

        let mode = match self.current_full_creen_mode {

        }

        self.retro_av.set_full_screen(mode)
    }
}
