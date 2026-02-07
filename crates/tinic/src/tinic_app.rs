use crate::tinic_app_ctx::TinicGameCtx;
use generics::error_handle::ErrorHandle;
use retro_controllers::RetroGamePad;
use winit::event_loop::EventLoopClosed;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    keyboard::{KeyCode, PhysicalKey},
    window::WindowId,
};

pub enum GameInstanceActions {
    ConnectDevice(RetroGamePad),
    ChangeDefaultSlot(usize),
    Pause,
    Resume,
    SaveState(usize),
    LoadState(usize),
    Exit,
}

pub struct GameInstance {
    ctx: TinicGameCtx,
    proxy: EventLoopProxy<GameInstanceActions>,
    pub default_slot: usize,
}
pub struct GameInstanceDispatchers {
    proxy: EventLoopProxy<GameInstanceActions>,
}

impl GameInstanceDispatchers {
    pub fn exit(&self) -> Result<(), EventLoopClosed<GameInstanceActions>> {
        self.proxy.send_event(GameInstanceActions::Exit)
    }

    pub fn pause(&self) -> Result<(), EventLoopClosed<GameInstanceActions>> {
        self.proxy.send_event(GameInstanceActions::Pause)
    }

    pub fn resume(&self) -> Result<(), EventLoopClosed<GameInstanceActions>> {
        self.proxy.send_event(GameInstanceActions::Resume)
    }

    pub fn load_state(&self, slot: usize) -> Result<(), EventLoopClosed<GameInstanceActions>> {
        self.proxy.send_event(GameInstanceActions::LoadState(slot))
    }

    pub fn save_state(&self, slot: usize) -> Result<(), EventLoopClosed<GameInstanceActions>> {
        self.proxy.send_event(GameInstanceActions::SaveState(slot))
    }

    pub fn change_default_slot(
        &self,
        slot: usize,
    ) -> Result<(), EventLoopClosed<GameInstanceActions>> {
        self.proxy
            .send_event(GameInstanceActions::ChangeDefaultSlot(slot))
    }

    pub fn connect_device(
        &self,
        device: RetroGamePad,
    ) -> Result<(), EventLoopClosed<GameInstanceActions>> {
        self.proxy
            .send_event(GameInstanceActions::ConnectDevice(device))
    }
}

impl GameInstance {
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

    pub fn create_dispatcher(&self) -> GameInstanceDispatchers {
        GameInstanceDispatchers {
            proxy: self.proxy.clone(),
        }
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

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Err(e) = self.ctx.redraw_request() {
            println!("{:?}", e);
            event_loop.exit();
        }
    }

    fn suspended(&mut self, _: &ActiveEventLoop) {
        self.ctx.suspend_window();
    }

    fn exiting(&mut self, _: &ActiveEventLoop) {
        if let Err(e) = self.ctx.close_retro_ctx() {
            println!("{:?}", e);
        }
    }
}
