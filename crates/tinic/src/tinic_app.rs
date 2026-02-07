use generics::retro_paths::RetroPaths;
use libretro_sys::binding_libretro::retro_hw_context_type;
use retro_av::RetroAv;
use retro_controllers::RetroController;
use retro_core::{graphic_api::GraphicApi, RetroCore, RetroCoreIns, RetroEnvCallbacks};
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow},
    window::WindowId,
};

pub struct TinicApp {
    retro_paths: RetroPaths,
    core_path: String,
    rom_path: String,
    controller: Arc<RetroController>,
    retro_av: RetroAv,
    retro_core: Option<RetroCoreIns>,
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
        self.create_context(event_loop);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.retro_av.redraw_request();
        event_loop.set_control_flow(ControlFlow::Poll);
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {}

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => self.redraw_request(),
            _ => {}
        }
    }
}

impl TinicApp {
    pub fn create_context(&mut self, event_loop: &ActiveEventLoop) {
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
        )
        .unwrap();

        let av_info = retro_core.load_game(&self.rom_path).unwrap();
        self.retro_core.replace(retro_core);
        self.retro_av.build_window(&av_info, event_loop).unwrap();
    }

    pub fn redraw_request(&mut self) {
        if let Some(retro_core) = &mut self.retro_core {
            if self.retro_av.sync() {
                retro_core.run().unwrap();
                self.retro_av.get_new_frame().unwrap()
            }
        }
    }
}
