use std::num::NonZeroU32;
use std::sync::Arc;

use super::render::Render;
use crate::video::{RawTextureData, RetroVideoAPi};
use crate::winit::{event_loop::ActiveEventLoop, window::Window};
use glutin::{
    config::{Config, ConfigTemplateBuilder},
    context::{
        ContextApi, ContextAttributesBuilder, NotCurrentContext, PossiblyCurrentContext, Version,
    },
    display::{GetGlDisplay, GlDisplay},
    prelude::{NotCurrentGlContext, PossiblyCurrentGlContext},
    surface::{GlSurface, Surface, WindowSurface},
};
use glutin_winit::{DisplayBuilder, GlWindow};
use raw_window_handle::HasWindowHandle;
use retro_core::av_info::{AvInfo, Geometry};
use winit::window::Fullscreen;

pub struct RetroGlWindow {
    renderer: Render,
    gl_context: PossiblyCurrentContext,
    gl_surface: Surface<WindowSurface>,
    window: Window,
}

fn create_gl_context(window: &Window, gl_config: &Config) -> NotCurrentContext {
    let raw_window_handle = window.window_handle().ok().map(|wh| wh.as_raw());

    // The context creation part.
    let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);

    // Since glutin by default tries to create OpenGL core context, which may not be
    // present we should try gles.
    let fallback_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(None))
        .build(raw_window_handle);

    // There are also some old devices that support neither modern OpenGL nor GLES.
    // To support these we can try and create a 2.1 context.
    let legacy_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::OpenGl(Some(Version::new(3, 1))))
        .build(raw_window_handle);

    // Reuse the uncurrented context from a suspended() call if it exists, otherwise
    // this is the first time resumed() is called, where the context still
    // has to be created.
    let gl_display = gl_config.display();

    unsafe {
        gl_display
            .create_context(gl_config, &context_attributes)
            .unwrap_or_else(|_| {
                gl_display
                    .create_context(gl_config, &fallback_context_attributes)
                    .unwrap_or_else(|_| {
                        gl_display
                            .create_context(gl_config, &legacy_context_attributes)
                            .expect("failed to create context")
                    })
            })
    }
}

impl RetroVideoAPi for RetroGlWindow {
    fn request_redraw(&self) {
        self.window.request_redraw();
    }

    fn draw_new_frame(&self, texture: &RawTextureData, geo: &Geometry) {
        let size = self.window.inner_size();
        self.renderer
            .draw_new_frame(texture, geo, size.width as i32, size.height as i32);
        self.gl_surface.swap_buffers(&self.gl_context).unwrap();
    }

    fn get_proc_address(&self, _proc_name: &str) -> *const () {
        todo!("get_proc_address ainda nao foi criado")
    }

    fn set_full_screen(&mut self, mode: Fullscreen) {
        self.window.set_fullscreen(Some(mode));
    }

    fn context_destroy(&mut self) {
        todo!("context_destroy ainda nao foi criado")
    }

    fn context_reset(&mut self) {
        todo!("context_reset ainda nao foi criado")
    }
}

impl RetroGlWindow {
    pub fn new(event_loop: &ActiveEventLoop, av_info: &Arc<AvInfo>) -> Self {
        let attributes = Window::default_attributes()
            .with_title("Simple Glium Window")
            .with_inner_size(winit::dpi::PhysicalSize::new(800, 480));
        // First we start by opening a new Window
        let display_builder = DisplayBuilder::new().with_window_attributes(Some(attributes));

        let template = ConfigTemplateBuilder::new();

        let (window, gl_config) = display_builder
            .build(event_loop, template, |configs| {
                configs.reduce(|_, config| config).unwrap()
            })
            .unwrap();

        let window = window.unwrap();

        // Create gl context.
        let gl_context = create_gl_context(&window, &gl_config).treat_as_possibly_current();

        let attrs = window
            .build_surface_attributes(Default::default())
            .expect("Failed to build surface attributes");

        let gl_surface = unsafe {
            gl_config
                .display()
                .create_window_surface(&gl_config, &attrs)
                .unwrap()
        };

        let size = window.inner_size();

        gl_surface.resize(
            &gl_context,
            NonZeroU32::new(size.width).unwrap(),
            NonZeroU32::new(size.height).unwrap(),
        );

        gl_context.make_current(&gl_surface).unwrap();

        Self {
            gl_context,
            gl_surface,
            renderer: Render::new(av_info, gl_config.display()).unwrap(),
            window,
        }
    }
}
