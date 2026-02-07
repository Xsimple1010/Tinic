use super::render::Render;
use crate::raw_texture::RawTextureData;
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
use retro_core::av_info::AvInfo;
use std::num::NonZeroU32;
use std::ptr::null;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use winit::dpi::PhysicalSize;
use winit::window::Fullscreen;

pub struct RetroGlWindow {
    window_mode: RetroWindowMode,
    renderer: Option<Render>,
    gl_context: Option<PossiblyCurrentContext>,
    gl_surface: Option<Surface<WindowSurface>>,
    gl_config: Config,
    window: Window,
    av_info: Arc<AvInfo>,
}

use crate::retro_window::{RetroWindowContext, RetroWindowMode};
use libretro_sys::binding_libretro::retro_hw_context_type;
use retro_core::graphic_api::GraphicApi;

fn create_gl_context(window: &Window, gl_config: &Config, api: &GraphicApi) -> NotCurrentContext {
    let raw_window_handle = window.window_handle().ok().map(|wh| wh.as_raw());
    let display = gl_config.display();

    let debug = api.debug_context.load(Ordering::SeqCst);

    // === 1. Decide API and version (RetroArch logic) ===
    let (primary, fallback) = match api.context_type {
        retro_hw_context_type::RETRO_HW_CONTEXT_OPENGL => {
            // Desktop GL: ignore major/minor
            (
                ContextApi::OpenGl(Some(Version::new(3, 3))),
                ContextApi::OpenGl(Some(Version::new(2, 1))),
            )
        }
        retro_hw_context_type::RETRO_HW_CONTEXT_OPENGLES2 => (
            ContextApi::Gles(Some(Version::new(2, 0))),
            ContextApi::Gles(Some(Version::new(2, 0))),
        ),
        retro_hw_context_type::RETRO_HW_CONTEXT_OPENGLES3 => {
            let major = api.major.load(Ordering::SeqCst);
            let minor = api.minor.load(Ordering::SeqCst);

            let version = if major >= 3 {
                Version::new(major, minor)
            } else {
                Version::new(3, 0)
            };

            (
                ContextApi::Gles(Some(version)),
                ContextApi::Gles(Some(Version::new(3, 0))),
            )
        }
        retro_hw_context_type::RETRO_HW_CONTEXT_OPENGL_CORE => (
            ContextApi::OpenGl(Some(Version::new(3, 3))),
            ContextApi::OpenGl(Some(Version::new(2, 1))),
        ),
        _ => panic!("Unsupported HW context type"),
    };

    // === 2. Context attributes ===
    let primary_attrs = ContextAttributesBuilder::new()
        .with_context_api(primary)
        .with_debug(debug)
        .build(raw_window_handle);

    let fallback_attrs = ContextAttributesBuilder::new()
        .with_context_api(fallback)
        .with_debug(debug)
        .build(raw_window_handle);

    // === 3. Create context (primary → fallback) ===
    unsafe {
        display
            .create_context(gl_config, &primary_attrs)
            .unwrap_or_else(|_| {
                display
                    .create_context(gl_config, &fallback_attrs)
                    .expect("Failed to create any GL context")
            })
    }
}

impl RetroWindowContext for RetroGlWindow {
    fn request_redraw(&self) {
        self.window.request_redraw();
    }

    fn draw_new_frame(&self, texture: &RawTextureData) {
        let size = self.window.inner_size();

        let renderer = match &self.renderer {
            Some(renderer) => renderer,
            None => return,
        };
        let gl_surface = match &self.gl_surface {
            Some(gl_surface) => gl_surface,
            None => return,
        };
        let gl_context = match &self.gl_context {
            Some(gl_context) => gl_context,
            None => return,
        };

        renderer.draw_new_frame(
            texture,
            &self.av_info.video.geometry,
            size.width as i32,
            size.height as i32,
        );
        gl_surface.swap_buffers(gl_context).unwrap();
    }

    fn get_proc_address(&self, proc_name: &str) -> *const () {
        println!("get_proc_address({:?})", proc_name);
        let cstr = std::ffi::CString::new(proc_name).unwrap();

        match &self.gl_context {
            Some(gl_context) => gl_context.display().get_proc_address(cstr.as_c_str()) as *const (),
            None => null(),
        }
    }

    fn set_window_mode(&mut self, mode: RetroWindowMode) {
        self.window_mode = mode;

        match self.window_mode {
            RetroWindowMode::FullScreen => self
                .window
                .set_fullscreen(Some(Fullscreen::Borderless(None))),
            RetroWindowMode::Windowed => self.window.set_fullscreen(None),
        }
    }

    fn toggle_window_model(&mut self) {
        match self.window_mode { 
            RetroWindowMode::FullScreen => self.set_window_mode(RetroWindowMode::Windowed),
            RetroWindowMode::Windowed => self.set_window_mode(RetroWindowMode::FullScreen),
        }
    }

    fn context_destroy(&mut self) {
        self.renderer = None;
        self.gl_context = None;
        self.gl_surface = None;
    }

    fn context_reset(&mut self) {
        // Create gl context.
        let gl_context = create_gl_context(
            &self.window,
            &self.gl_config,
            &self.av_info.video.graphic_api,
        )
        .treat_as_possibly_current();

        let attrs = self
            .window
            .build_surface_attributes(Default::default())
            .expect("Failed to build surface attributes");

        let gl_surface = unsafe {
            self.gl_config
                .display()
                .create_window_surface(&self.gl_config, &attrs)
                .unwrap()
        };

        let size = self.window.inner_size();

        gl_surface.resize(
            &gl_context,
            NonZeroU32::new(size.width).unwrap(),
            NonZeroU32::new(size.height).unwrap(),
        );

        gl_context.make_current(&gl_surface).unwrap();

        let render = Render::new(&self.av_info, self.gl_config.display()).unwrap();

        self.renderer = Some(render);
        self.gl_context = Some(gl_context);
        self.gl_surface = Some(gl_surface);
    }

    fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        let gl_surface = match &self.gl_surface {
            Some(gl_surface) => gl_surface,
            None => return,
        };
        let gl_context = match &self.gl_context {
            Some(gl_context) => gl_context,
            None => return,
        };

        gl_surface.resize(
            gl_context,
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );
    }

    fn draw_context_as_initialized(&self) -> bool {
        self.gl_context.is_some()
    }
}

impl RetroGlWindow {
    pub fn new(event_loop: &ActiveEventLoop, av_info: &Arc<AvInfo>) -> Self {
        let window_size = PhysicalSize::new(800, 480);
        let attributes = Window::default_attributes()
            .with_title("Tinic")
            .with_inner_size(window_size)
            .with_transparent(false);

        let display_builder = DisplayBuilder::new().with_window_attributes(Some(attributes));
        let template = ConfigTemplateBuilder::new().with_transparency(false);

        let (window, gl_config) = display_builder
            .build(event_loop, template, |configs| {
                configs.reduce(|_, config| config).unwrap()
            })
            .unwrap();

        let window = window.unwrap();
        window.set_min_inner_size(Some(window_size));

        Self {
            gl_context: None,
            gl_surface: None,
            renderer: None,
            window,
            gl_config,
            av_info: av_info.clone(),
            window_mode: RetroWindowMode::Windowed,
        }
    }
}
