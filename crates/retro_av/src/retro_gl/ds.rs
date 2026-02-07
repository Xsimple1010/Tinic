use std::cell::UnsafeCell;
use std::num::NonZeroU32;
use std::sync::Arc;

use crate::glutin::config::ConfigTemplateBuilder;
use crate::video::RawTextureData;
use crate::winit::window::Window;
use glutin::config::Config;
use glutin::context::ContextApi;
use glutin::context::ContextAttributesBuilder;
use glutin::context::NotCurrentContext;
use glutin::context::PossiblyCurrentContext;
use glutin::context::Version;
use glutin::display::GetGlDisplay;
use glutin::prelude::GlDisplay;
use glutin::prelude::NotCurrentGlContext;
use glutin::prelude::PossiblyCurrentGlContext;
use glutin::surface::GlSurface;
use glutin::surface::Surface;
use glutin::surface::WindowSurface;
use glutin_winit::DisplayBuilder;
use glutin_winit::GlWindow;
use raw_window_handle::HasWindowHandle;
use retro_core::av_info;
use retro_core::av_info::AvInfo;
use retro_core::av_info::Geometry;
use winit::event_loop::ActiveEventLoop;

use super::render::Render;

pub struct RetroGlWindow {
    template: ConfigTemplateBuilder,
    renderer: Option<Render>,
    // // state: Option<WindowState>,
    gl_context: Option<PossiblyCurrentContext>,
    gl_surface: Option<Surface<WindowSurface>>,
    // // NOTE: Window should be dropped after all resources created using its
    // // raw-window-handle.
    window: Option<Window>,
    // exit_state: Result<(), Box<dyn Error>>,
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

impl RetroGlWindow {
    pub fn new() -> Self {
        Self {
            // attributes: ,
            renderer: None,
            template: ConfigTemplateBuilder::new(),
            // vsync: true,
            window: None,
            gl_surface: None,
            gl_context: None,
            // template,
        }
    }

    pub fn request_redraw(&self) {
        if let Some(win) = &self.window {
            win.request_redraw();
        }
    }

    pub fn try_render(&self, next_frame: &UnsafeCell<RawTextureData>, geo: &Geometry) {
        if let Some(render) = &self.renderer {
            if let Some(win) = &self.window {
                let size = win.inner_size();
                render.draw_new_frame(next_frame, geo, size.width as i32, size.height as i32);
                self.gl_surface
                    .as_ref()
                    .unwrap()
                    .swap_buffers(self.gl_context.as_ref().unwrap())
                    .unwrap();
            }
        }
    }

    pub fn build(&mut self, event_loop: &ActiveEventLoop, av_info: &Arc<AvInfo>) {
        let attributes = Window::default_attributes()
            .with_title("Simple Glium Window")
            .with_inner_size(winit::dpi::PhysicalSize::new(800, 480));
        // First we start by opening a new Window
        let display_builder = DisplayBuilder::new().with_window_attributes(Some(attributes));

        let (window, gl_config) = display_builder
            .build(event_loop, self.template.clone(), |configs| {
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

        self.renderer
            .get_or_insert_with(|| Render::new(av_info, gl_config.display()).unwrap());
        self.gl_context.replace(gl_context);
        self.gl_surface.replace(gl_surface);
        self.window.replace(window);
    }
}
