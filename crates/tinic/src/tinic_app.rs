use crate::tinic_app_ctx::TinicGameCtx;
use generics::error_handle::ErrorHandle;
use retro_controllers::devices_manager::Device;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    keyboard::{KeyCode, PhysicalKey},
    window::WindowId,
};

pub struct TinicGameInstance {
    ctx: TinicGameCtx,
    proxy: EventLoopProxy<GameInstanceActions>,
    pub default_slot: usize,
}

pub enum GameInstanceActions {
    ConnectDevice(Device),
    Pause,
    Resume,
    SaveState(usize),
    LoadState(usize),
    Exit,
}

impl TinicGameInstance {
    pub fn new(ctx: TinicGameCtx) -> (Self, EventLoop<GameInstanceActions>) {
        let event_loop = EventLoop::<GameInstanceActions>::with_user_event()
            .build()
            .unwrap();

        let proxy = event_loop.create_proxy();

        (
            Self {
                ctx,
                default_slot: 1,
                proxy: proxy.clone(),
            },
            event_loop,
        )
    }

    pub fn exit(&self) {
        let _ = self.proxy.send_event(GameInstanceActions::Exit);
    }

    pub fn pause(&self) {
        let _ = self.proxy.send_event(GameInstanceActions::Pause);
    }

    pub fn resume(&self) {
        let _ = self.proxy.send_event(GameInstanceActions::Resume);
    }

    pub fn load_state(&self, slot: usize) {
        let _ = self.proxy.send_event(GameInstanceActions::LoadState(slot));
    }

    pub fn save_state(&self, slot: usize) {
        let _ = self.proxy.send_event(GameInstanceActions::SaveState(slot));
    }

    pub fn change_default_slot(&mut self, slot: usize) {
        self.default_slot = slot;
    }

    pub fn connect_device(&self, device: Device) {
        let _ = self
            .proxy
            .send_event(GameInstanceActions::ConnectDevice(device));
    }
}

impl ApplicationHandler<GameInstanceActions> for TinicGameInstance {
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

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: GameInstanceActions) {
        let result = match event {
            GameInstanceActions::ConnectDevice(device) => self.ctx.connect_controller(device),
            GameInstanceActions::LoadState(slot) => self.ctx.load_state(slot),
            GameInstanceActions::SaveState(slot) => self.ctx.save_state(slot),
            GameInstanceActions::Pause => {
                self.ctx.pause();
                Ok(())
            }
            GameInstanceActions::Resume => {
                self.ctx.resume();
                Ok(())
            }
            GameInstanceActions::Exit => {
                event_loop.exit();
                Ok(())
            }
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
        let result: Result<(), ErrorHandle> = match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
                Ok(())
            }
            WindowEvent::RedrawRequested => self.ctx.draw_new_frame(),
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                if event.repeat || !event.state.is_pressed() {
                    return;
                }

                match event.physical_key {
                    PhysicalKey::Code(KeyCode::F1) => self.ctx.save_state(self.default_slot),
                    PhysicalKey::Code(KeyCode::F2) => self.ctx.load_state(self.default_slot),
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
