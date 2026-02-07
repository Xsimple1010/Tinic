use crate::app_dispatcher::{GameInstanceActions, GameInstanceDispatchers};
use crate::tinic_app_ctx::TinicGameCtx;
use generics::error_handle::ErrorHandle;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoopProxy},
    keyboard::{KeyCode, PhysicalKey},
    window::WindowId,
};

pub struct GameInstance {
    ctx: TinicGameCtx,
    proxy: EventLoopProxy<GameInstanceActions>,
    pub default_slot: usize,
}

impl GameInstance {
    pub fn new(ctx: TinicGameCtx, proxy: EventLoopProxy<GameInstanceActions>) -> Self {
        Self {
            ctx,
            default_slot: 1,
            proxy,
        }
    }

    pub fn create_dispatcher(&self) -> GameInstanceDispatchers {
        GameInstanceDispatchers::new(self.proxy.clone())
    }

    pub fn change_default_slot(&mut self, slot: usize) {
        self.default_slot = slot;
    }
}

impl ApplicationHandler<GameInstanceActions> for GameInstance {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Err(e) = self.ctx.create_window(event_loop) {
            println!("{:?}", e);
            event_loop.exit();
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: GameInstanceActions) {
        let result = match event {
            GameInstanceActions::ConnectDevice(device) => self.ctx.connect_controller(device),
            GameInstanceActions::LoadState(slot) => self.ctx.load_state(slot),
            GameInstanceActions::SaveState(slot) => self.ctx.save_state(slot),
            GameInstanceActions::ChangeDefaultSlot(slot) => {
                self.default_slot = slot;
                Ok(())
            }
            GameInstanceActions::EnableKeyboard => self.ctx.active_keyboard(),
            GameInstanceActions::DisableKeyboard => {
                self.ctx.disable_keyboard();
                Ok(())
            }
            GameInstanceActions::Pause => self.ctx.pause(),
            GameInstanceActions::Resume => self.ctx.resume(),
            GameInstanceActions::Exit => {
                event_loop.exit();
                Ok(())
            }
        };

        if let Err(e) = result {
            let _ = self.ctx.destroy_retro_ctx();
            event_loop.exit();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let result: Result<(), ErrorHandle> = match event {
            WindowEvent::CloseRequested => {
                let _ = self.ctx.destroy_retro_ctx();
                event_loop.exit();
                Ok(())
            }
            WindowEvent::RedrawRequested => self.ctx.draw_new_frame(),
            WindowEvent::Resized(size) => self.ctx.resize_window(size),
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                self.ctx
                    .update_keyboard_state(event.physical_key, event.state.is_pressed());

                if event.repeat || !event.state.is_pressed() {
                    return;
                }

                match event.physical_key {
                    PhysicalKey::Code(KeyCode::F1) => self.ctx.save_state(self.default_slot),
                    PhysicalKey::Code(KeyCode::F2) => self.ctx.load_state(self.default_slot),
                    PhysicalKey::Code(KeyCode::F3) => self.ctx.toggle_keyboard_usage(),
                    PhysicalKey::Code(KeyCode::F5) => self.ctx.reset(),
                    PhysicalKey::Code(KeyCode::F8) => self.ctx.toggle_can_request_new_frames(),
                    PhysicalKey::Code(KeyCode::F11) => self.ctx.toggle_full_screen_mode(),
                    _ => Ok(()),
                }
            }
            _ => Ok(()),
        };

        if let Err(e) = result {
            let _ = self.ctx.destroy_retro_ctx();
            event_loop.exit();
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Err(e) = self.ctx.redraw_request() {
            let _ = self.ctx.destroy_retro_ctx();
            event_loop.exit();
        }
    }

    fn suspended(&mut self, _: &ActiveEventLoop) {
        self.ctx.suspend_window();
    }
}
