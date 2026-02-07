pub mod listener;
mod user_events;
mod window_events;

use crate::app::listener::WindowListener;
use crate::app_dispatcher::{GameInstanceActions, GameInstanceDispatchers};
use crate::tinic_app_ctx::TinicGameCtx;
use crate::TinicGameInfo;
use generics::error_handle::ErrorHandle;
use retro_controllers::RetroController;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
    window::WindowId,
};

pub struct GameInstance {
    ctx: TinicGameCtx,
    game_dispatchers: GameInstanceDispatchers,
    pub default_slot: usize,
    window_listener: Arc<Box<dyn WindowListener>>,
}

impl GameInstance {
    pub fn new(
        game_info: TinicGameInfo,
        retro_controle: Arc<RetroController>,
        window_listener: Arc<Box<dyn WindowListener>>,
        game_dispatchers: GameInstanceDispatchers,
    ) -> Result<Self, ErrorHandle> {
        let ctx = TinicGameCtx::new(game_info, retro_controle, window_listener.clone())?;

        Ok(Self {
            ctx,
            default_slot: 1,
            game_dispatchers,
            window_listener,
        })
    }

    pub fn create_dispatcher(&self) -> GameInstanceDispatchers {
        self.game_dispatchers.clone()
    }

    fn destroy_window_and_render_context(&self, event_loop: &ActiveEventLoop, ctx: &TinicGameCtx) {
        let _ = ctx.destroy_retro_ctx();
        event_loop.exit();
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
        self.process_user_event(event_loop, event);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        self.process_window_event(event_loop, event);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Err(e) = self.ctx.redraw_request() {
            self.destroy_window_and_render_context(event_loop, &self.ctx);
        }
    }

    fn suspended(&mut self, _: &ActiveEventLoop) {
        self.ctx.suspend_window();
    }
}
