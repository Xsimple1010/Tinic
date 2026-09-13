use crate::raw_texture::RawTextureData;
use crate::retro_env_callback::RetroVideoCb;
use crate::retro_gl::proc_resolver::GlProcResolver;
use crate::retro_window::{RetroWindowContext, RetroWindowMode};
use crate::sync::RetroSync;
use crate::window_ctx::WindowCtx;
use crate::{print_scree::PrintScree, retro_gl::window::RetroGlWindow};
use retro_core::av_info::AvInfo;
use retro_core::graphic_api::HwContextType;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tinic_generics::{
    error_handle::{ErrorHandle, TinicResult},
    types::{ArcTMutex, TMutex},
};
use winit::event_loop::ActiveEventLoop;

pub struct RetroVideo {
    window_ctx: Option<WindowCtx>,
    texture: ArcTMutex<RawTextureData>,
    proc_resolve: Arc<GlProcResolver>,
    pub sync: RetroSync,
}

impl Default for RetroVideo {
    fn default() -> Self {
        Self {
            window_ctx: None,
            texture: TMutex::new(RawTextureData::new()),
            proc_resolve: Arc::new(GlProcResolver::new()),
            sync: RetroSync::new(0.0002),
        }
    }
}

impl RetroVideo {
    pub fn create_window(
        &mut self,
        av_info: &Arc<AvInfo>,
        event_loop: &ActiveEventLoop,
    ) -> TinicResult<()> {
        match *av_info.video.graphic_api.context_type.read()? {
            HwContextType::OpenGl | HwContextType::OpenGlCore => {
                self.window_ctx
                    .replace(WindowCtx::OpenGl(RetroGlWindow::new(
                        event_loop,
                        av_info,
                        &self.texture,
                        self.proc_resolve.clone(),
                    )));
            }
            _ => {
                return Err(ErrorHandle::new(
                    "[create_window]: unsupported graphics API",
                ));
            }
        }

        Ok(())
    }

    pub fn create_draw_context(&mut self) -> TinicResult<()> {
        let ctx = self.window_ctx.as_mut().ok_or_else(|| {
            ErrorHandle::new("[create_draw_context]: window context not initialized")
        })?;

        ctx.init_context()?;
        Ok(())
    }

    pub fn draw_context_as_initialized(&self) -> bool {
        self.window_ctx
            .as_ref()
            .map(|ctx| ctx.draw_context_as_initialized())
            .unwrap_or(false)
    }

    pub fn teardown_graphics(&mut self) -> TinicResult<()> {
        let ctx = self.window_ctx.as_mut().ok_or_else(|| {
            ErrorHandle::new("[teardown_graphics]: window context not initialized")
        })?;

        ctx.destroy()?;

        self.texture.store(RawTextureData::new());
        self.window_ctx = None;

        Ok(())
    }

    pub fn request_redraw(&self) -> TinicResult<()> {
        let ctx = self
            .window_ctx
            .as_ref()
            .ok_or_else(|| ErrorHandle::new("[request_redraw]: window context not initialized"))?;

        ctx.request_redraw();
        Ok(())
    }

    pub fn draw_new_frame(&self) -> TinicResult<()> {
        let ctx = self
            .window_ctx
            .as_ref()
            .ok_or_else(|| ErrorHandle::new("[draw_new_frame]: window context not initialized"))?;

        ctx.draw_new_frame()
            .map_err(|e| ErrorHandle::new(&format!("[draw_new_frame]: {e:?}")))
    }

    pub fn print_screen(&self, out_path: &Path, av_info: &Arc<AvInfo>) -> TinicResult<()> {
        PrintScree::take(
            &*self.texture.try_load()?,
            av_info,
            &mut PathBuf::from(out_path),
        )
    }

    pub fn toggle_window_mode(&mut self) -> TinicResult<()> {
        let ctx = self.window_ctx.as_mut().ok_or_else(|| {
            ErrorHandle::new("[toggle_window_mode]: window context not initialized")
        })?;

        ctx.toggle_window_model();
        Ok(())
    }

    pub fn set_window_mode(&mut self, mode: RetroWindowMode) -> TinicResult<()> {
        let ctx = self
            .window_ctx
            .as_mut()
            .ok_or_else(|| ErrorHandle::new("[set_window_mode]: window context not initialized"))?;

        ctx.set_window_mode(mode);
        Ok(())
    }

    pub fn resize_window(&mut self, width: u32, height: u32) -> TinicResult<()> {
        let ctx = self
            .window_ctx
            .as_mut()
            .ok_or_else(|| ErrorHandle::new("[resize_window]: window context not initialized"))?;

        ctx.resize(width, height);
        Ok(())
    }

    pub fn get_core_cb(&self) -> TinicResult<RetroVideoCb> {
        Ok(RetroVideoCb::new(
            self.texture.clone(),
            self.proc_resolve.clone(),
        ))
    }

    pub fn prepare_to_core(&mut self) -> TinicResult<()> {
        let ctx = self
            .window_ctx
            .as_mut()
            .ok_or_else(|| ErrorHandle::new("[prepare_to_core]: window context not initialized"))?;

        ctx.prepare_for_core();
        Ok(())
    }

    pub fn init_frame_buffer(&mut self, av_info: &Arc<AvInfo>) -> TinicResult<()> {
        let ctx = self.window_ctx.as_mut().ok_or_else(|| {
            ErrorHandle::new("[init_frame_buffer]: window context not initialized")
        })?;

        ctx.init_frame_buffer(av_info)
    }
}
