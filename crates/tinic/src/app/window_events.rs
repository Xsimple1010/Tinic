use crate::app::GameInstance;
use generics::error_handle::ErrorHandle;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};

impl GameInstance {
    pub(crate) fn process_window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        event: WindowEvent,
    ) {
        let result: Result<(), ErrorHandle> = match event {
            WindowEvent::CloseRequested => {
                self.destroy_window_and_render_context(event_loop, &self.ctx);
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
            self.destroy_window_and_render_context(event_loop, &self.ctx);
        }
    }
}
