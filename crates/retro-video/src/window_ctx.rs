use std::sync::Arc;

use retro_core::av_info::AvInfo;
use tinic_generics::error_handle::TinicResult;

use crate::{RetroWindowMode, retro_gl::window::RetroGlWindow, retro_window::RetroWindowContext};

pub enum WindowCtx {
    OpenGl(RetroGlWindow),
}

impl RetroWindowContext for WindowCtx {
    fn request_redraw(&self) {
        match self {
            WindowCtx::OpenGl(w) => w.request_redraw(),
        }
    }

    fn draw_new_frame(&self) -> TinicResult<()> {
        match self {
            WindowCtx::OpenGl(w) => w.draw_new_frame(),
        }
    }

    fn set_window_mode(&mut self, mode: RetroWindowMode) {
        match self {
            WindowCtx::OpenGl(w) => w.set_window_mode(mode),
        }
    }

    fn toggle_window_model(&mut self) {
        match self {
            WindowCtx::OpenGl(w) => w.toggle_window_model(),
        }
    }

    fn destroy(&mut self) -> TinicResult<()> {
        match self {
            WindowCtx::OpenGl(w) => w.destroy(),
        }
    }

    fn init_context(&mut self) -> TinicResult<()> {
        match self {
            WindowCtx::OpenGl(w) => w.init_context(),
        }
    }

    fn resize(&mut self, width: u32, height: u32) {
        match self {
            WindowCtx::OpenGl(w) => w.resize(width, height),
        }
    }

    fn draw_context_as_initialized(&self) -> bool {
        match self {
            WindowCtx::OpenGl(w) => w.draw_context_as_initialized(),
        }
    }

    fn init_frame_buffer(&mut self, av_info: &Arc<AvInfo>) -> TinicResult<()> {
        match self {
            WindowCtx::OpenGl(w) => w.init_frame_buffer(av_info),
        }
    }

    fn prepare_for_core(&self) {
        match self {
            WindowCtx::OpenGl(w) => w.prepare_for_core(),
        }
    }
}
