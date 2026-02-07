use crate::app::GameInstance;
use crate::app_dispatcher::GameInstanceActions;
use winit::event_loop::ActiveEventLoop;

impl GameInstance {
    pub(crate) fn process_user_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        event: GameInstanceActions,
    ) {
        let result = match event {
            GameInstanceActions::ConnectDevice(device) => self.ctx.connect_controller(device),
            GameInstanceActions::LoadState(slot) => self.ctx.load_state(slot),
            GameInstanceActions::SaveState(slot) => self.ctx.save_state(slot),
            GameInstanceActions::ChangeDefaultSlot(slot) => Ok(self.default_slot = slot),
            GameInstanceActions::EnableKeyboard => self.ctx.active_keyboard(),
            GameInstanceActions::DisableKeyboard => Ok(self.ctx.disable_keyboard()),
            GameInstanceActions::Pause => self.ctx.pause(),
            GameInstanceActions::Resume => self.ctx.resume(),
            GameInstanceActions::Exit => {
                Ok(self.destroy_window_and_render_context(event_loop, &self.ctx))
            }
        };

        if let Err(e) = result {
            self.destroy_window_and_render_context(event_loop, &self.ctx);
            println!("Error: {e:?}");
        }
    }
}
