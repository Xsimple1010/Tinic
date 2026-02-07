use generics::erro_handle::ErroHandle;
use retro_controllers::devices_manager::Device;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowId,
};

use crate::tinic_app_ctx::TinicAppCtx;

pub struct TinicApp {
    ctx: TinicAppCtx,
}

pub enum TinicAppActions {
    ConnectDevice(Device),
}

impl TinicApp {
    pub fn new(ctx: TinicAppCtx) -> Self {
        Self { ctx }
    }
}

impl ApplicationHandler<TinicAppActions> for TinicApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Err(e) = self.ctx.create_window(event_loop) {
            println!("{:?}", e);
            event_loop.exit();
        }
    }

    fn suspended(&mut self, _: &ActiveEventLoop) {
        self.ctx.suspend_window();
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Err(e) = self.ctx.redraw_request() {
            println!("{:?}", e);
            event_loop.exit();
        }
    }

    fn exiting(&mut self, _: &ActiveEventLoop) {
        if let Err(e) = self.ctx.close_retro_ctx() {
            println!("{:?}", e);
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: TinicAppActions) {
        let result = match event {
            TinicAppActions::ConnectDevice(device) => self.ctx.connect_controller(device),
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
            WindowEvent::RedrawRequested => self.ctx.redraw_request(),
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                if event.repeat || !event.state.is_pressed() {
                    return;
                }

                match event.physical_key {
                    PhysicalKey::Code(KeyCode::F1) => self.ctx.save_state(1),
                    PhysicalKey::Code(KeyCode::F2) => self.ctx.load_state(1),
                    PhysicalKey::Code(KeyCode::F5) => self.ctx.reset(),
                    PhysicalKey::Code(KeyCode::F8) => {
                        self.ctx.toggle_can_request_new_frames();
                        Ok(())
                    }
                    PhysicalKey::Code(KeyCode::F11) => self.ctx.toggle_full_screen_mode(),
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
