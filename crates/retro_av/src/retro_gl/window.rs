use super::{gl::gl, render::Render};
use crate::video::{RawTextureData, RetroVideoAPi};
use generics::erro_handle::ErroHandle;
use libretro_sys::binding_libretro::retro_hw_context_type::{
    RETRO_HW_CONTEXT_NONE, RETRO_HW_CONTEXT_OPENGL, RETRO_HW_CONTEXT_OPENGL_CORE,
};
use retro_core::av_info::AvInfo;
use std::ptr::null;
use std::{cell::UnsafeCell, sync::atomic::Ordering};
use std::{rc::Rc, sync::Arc};
use winit::event_loop::ActiveEventLoop;

use super::*;
use crate::glutin::config::{Config, ConfigTemplateBuilder};
use glutin_winit::DisplayBuilder;
use std::error::Error;
use std::num::NonZeroU32;
use winit::window::Window;
use winit::window::WindowAttributes;

pub struct GlWindow {
    // video: VideoSubsystem,
    window: Window,
    // gl_ctx: Option<GLContext>,
    render: Render,
    av_info: Arc<AvInfo>,
}

impl Drop for GlWindow {
    fn drop(&mut self) {
        //gl_ctx precisa ser deletado antes de tudo!
        /* esse é comportamento ideal aqui
        // Deletar o contexto OpenGL
        SDL_GL_DeleteContext(gl_context);

        // Destruir a janela
        SDL_DestroyWindow(window);
        */
        // {
        //     self.gl_ctx.take();
        // }

        // self.video.gl_unload_library();
    }
}

impl RetroVideoAPi for GlWindow {
    fn get_window_id(&self) -> u32 {
        0
    }

    fn draw_new_frame(&self, texture: &UnsafeCell<RawTextureData>) {
        // let (width, height) = self.window.size();

        // self.render.draw_new_frame(
        //     texture,
        //     &self.av_info.video.geometry,
        //     width as i32,
        //     height as i32,
        // );

        // self.window.gl_swap_window();
    }

    fn resize(&mut self, (width, height): (u32, u32)) {}

    fn get_proc_address(&self, proc_name: &str) -> *const () {
        null()
    }

    fn context_destroy(&mut self) {
        println!("context_destroy");
    }

    fn context_reset(&mut self) {
        println!("context_reset");
    }

    fn enable_full_screen(&mut self) {}

    fn disable_full_screen(&mut self) {}
}

impl GlWindow {
    pub fn new(
        av_info: &Arc<AvInfo>,
        event_loop: &ActiveEventLoop,
    ) -> Result<GlWindow, ErroHandle> {
        use glutin::prelude::*;
        use raw_window_handle::HasWindowHandle;

        let attributes = Window::default_attributes()
            .with_title("Simple Glium Window")
            .with_inner_size(winit::dpi::PhysicalSize::new(800, 480));
        let template = ConfigTemplateBuilder::new();

        // First we start by opening a new Window
        let display_builder = DisplayBuilder::new().with_window_attributes(Some(attributes));
        let config_template_builder = ConfigTemplateBuilder::new();
        let (window, gl_config) = event_loop
            .build(display_builder, config_template_builder, |mut configs| {
                // Just use the first configuration since we don't have any special preferences here
                configs.next().unwrap()
            })
            .unwrap();
        let window = window.unwrap();

        Err(ErroHandle {
            message: "dss".to_string(),
        })
    }
}
