use generics::{
    constants::SAVE_IMAGE_EXTENSION_FILE, erro_handle::ErroHandle, retro_paths::RetroPaths,
};
use libretro_sys::binding_libretro::retro_hw_context_type;
use retro_av::RetroAv;
use retro_controllers::{devices_manager::Device, RetroController};
use retro_core::{graphic_api::GraphicApi, RetroCore, RetroCoreIns, RetroEnvCallbacks};
use std::{path::PathBuf, sync::Arc};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Fullscreen, WindowId},
};

pub struct TinicApp {
    controller: Arc<RetroController>,
    retro_av: RetroAv,
    retro_core: RetroCoreIns,
    current_full_screen_mode: Fullscreen,
    can_request_new_frames: bool,
}

pub enum TinicAppActions {
    ConnectDevice(Device),
}

impl TinicApp {
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
}

impl ApplicationHandler<TinicAppActions> for TinicApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Err(e) = self.create_window(event_loop) {
            println!("{:?}", e);
            event_loop.exit();
        }
    }

    fn suspended(&mut self, _: &ActiveEventLoop) {
        self.suspend_window();
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Err(e) = self.retro_av.redraw_request() {
            println!("{:?}", e);
            event_loop.exit();
        }
    }

    fn exiting(&mut self, _: &ActiveEventLoop) {
        if let Err(e) = self.close_retro_ctx() {
            println!("{:?}", e);
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: TinicAppActions) {
        let result = match event {
            TinicAppActions::ConnectDevice(device) => self.connect_controller(device),
        };

        if let Err(e) = result {
            println!("{:?}", e);
            event_loop.exiting();
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
            } => {
                if event.repeat || !event.state.is_pressed() {
                    return;
                }

                match event.physical_key {
                    PhysicalKey::Code(KeyCode::F1) => self.save_state(1),
                    PhysicalKey::Code(KeyCode::F2) => self.load_state(1),
                    PhysicalKey::Code(KeyCode::F5) => self.reset(),
                    PhysicalKey::Code(KeyCode::F8) => {
                        self.toggle_can_request_new_frames();
                        Ok(())
                    }
                    PhysicalKey::Code(KeyCode::F11) => self.toggle_full_screen_mode(),
                    _ => Ok(()),
                }
            }
            _ => Ok(()),
        };

        if let Err(e) = result {
            println!("{:?}", e);
            event_loop.exit();
        }
    }
}

impl TinicApp {
    fn create_window(&mut self, event_loop: &ActiveEventLoop) -> Result<(), ErroHandle> {
        self.retro_av
            .build_window(&self.retro_core.av_info.clone(), event_loop)?;
        self.controller.stop_thread_events();

        Ok(())
    }

    fn suspend_window(&mut self) {
        self.retro_av.destroy_window();
        self.controller.resume_thread_events();
    }

    fn close_retro_ctx(&self) -> Result<(), ErroHandle> {
        self.retro_core.de_init()?;
        self.controller.resume_thread_events();

        Ok(())
    }

    fn redraw_request(&mut self) -> Result<(), ErroHandle> {
        if self.retro_av.sync() {
            if !self.can_request_new_frames {
                return Ok(());
            }

            self.retro_core.run()?;
            self.retro_av.get_new_frame()?
        }

        Ok(())
    }

    fn reset(&self) -> Result<(), ErroHandle> {
        self.retro_core.reset()
    }

    fn save_state(&self, slot: usize) -> Result<(), ErroHandle> {
        let save_path = self.retro_core.save_state(slot)?;

        let mut img_path = save_path.clone();
        img_path.set_extension(SAVE_IMAGE_EXTENSION_FILE);

        self.print_screen(&img_path)?;
        Ok(())
    }

    fn load_state(&self, slot: usize) -> Result<(), ErroHandle> {
        self.retro_core.load_state(slot)?;
        Ok(())
    }

    fn print_screen(&self, out_path: &PathBuf) -> Result<(), ErroHandle> {
        self.retro_av.print_screen(out_path)
    }

    fn toggle_full_screen_mode(&mut self) -> Result<(), ErroHandle> {
        self.retro_av
            .set_full_screen(self.current_full_screen_mode.clone())
    }

    fn toggle_can_request_new_frames(&mut self) {
        if self.can_request_new_frames {
            self.controller.resume_thread_events();
            self.can_request_new_frames = false;
        } else {
            self.controller.stop_thread_events();
            self.can_request_new_frames = true;
        }
    }

    fn connect_controller(&self, device: Device) -> Result<(), ErroHandle> {
        self.retro_core
            .connect_controller(device.retro_port, device.retro_type)
    }
}
