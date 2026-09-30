use super::render::Render;
use crate::raw_texture::RawTextureData;
use crate::retro_gl::proc_resolver::GlProcResolver;
use crate::retro_window::{RetroWindowContext, RetroWindowMode};
use crate::winit::{event_loop::ActiveEventLoop, window::Window};
use glutin::context::GlProfile;
use glutin::prelude::{NotCurrentGlContext, PossiblyCurrentGlContext};
use glutin::{
    config::{Config, ConfigTemplateBuilder},
    context::{
        ContextApi, ContextAttributesBuilder, NotCurrentContext, PossiblyCurrentContext, Version,
    },
    display::{GetGlDisplay, GlDisplay},
    surface::{GlSurface, Surface, WindowSurface},
};
use glutin_winit::{DisplayBuilder, GlWindow};
use raw_window_handle::HasWindowHandle;
use retro_core::av_info::AvInfo;
use retro_core::graphic_api::{GraphicApi, HwContextType};
use std::ffi::CString;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use tinic_generics::error_handle::{ErrorHandle, TinicResult};
use tinic_generics::types::{ArcTMutex, TMutex};
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
    texture: ArcTMutex<RawTextureData>,
    pub proc_resolve: Arc<GlProcResolver>,
}

fn create_gl_context(
    window: &Window,
    gl_config: &Config,
    api: &GraphicApi,
) -> TinicResult<NotCurrentContext> {
    let raw_window_handle = window.window_handle().ok().map(|wh| wh.as_raw());
    let display = gl_config.display();
    let debug = api.debug_context.load(Ordering::SeqCst);

    let (primary, fallback) = match *api.context_type.read()? {
        HwContextType::OpenGl | HwContextType::OpenGlCore | HwContextType::None => (
            ContextApi::OpenGl(Some(Version::new(3, 3))),
            ContextApi::OpenGl(Some(Version::new(2, 1))),
        ),
        HwContextType::OpenGlEs2 => (
            ContextApi::Gles(Some(Version::new(2, 0))),
            ContextApi::Gles(Some(Version::new(2, 0))),
        ),
        HwContextType::OpenGlEs3 => {
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
        _ => return Err(ErrorHandle::new("Unsupported HW context type")),
    };

    let primary_attrs = ContextAttributesBuilder::new()
        .with_context_api(primary)
        .with_profile(GlProfile::Compatibility)
        .with_debug(debug)
        .build(raw_window_handle);

    let fallback_attrs = ContextAttributesBuilder::new()
        .with_context_api(fallback)
        .with_profile(GlProfile::Compatibility)
        .with_debug(debug)
        .build(raw_window_handle);

    unsafe {
        let ctx = display
            .create_context(gl_config, &primary_attrs)
            .unwrap_or_else(|_| {
                display
                    .create_context(gl_config, &fallback_attrs)
                    .expect("Failed to create any GL context")
            });

        Ok(ctx)
    }
}

impl RetroWindowContext for RetroGlWindow {
    fn request_redraw(&self) {
        self.window.request_redraw();
    }

    fn draw_new_frame(&self) -> TinicResult<()> {
        let size = self.window.inner_size();

        let renderer = match &self.renderer {
            Some(renderer) => renderer,
            None => {
                return Err(ErrorHandle::new(
                    "RetroGlWindow: renderer não inicializado ao tentar desenhar frame",
                ));
            }
        };

        let gl_surface = match &self.gl_surface {
            Some(gl_surface) => gl_surface,
            None => {
                return Err(ErrorHandle::new(
                    "RetroGlWindow: gl_surface não inicializada ao tentar swap_buffers",
                ));
            }
        };

        let gl_context = match &self.gl_context {
            Some(gl_context) => gl_context,
            None => {
                return Err(ErrorHandle::new(
                    "RetroGlWindow: gl_context não inicializado ao tentar swap_buffers",
                ));
            }
        };

        renderer.draw_new_frame(&self.av_info, size.width as i32, size.height as i32)?;

        gl_surface.swap_buffers(gl_context).unwrap();

        Ok(())
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

    fn destroy(&mut self) -> TinicResult<()> {
        let ctx = match self.gl_context.take() {
            Some(ctx) => ctx,
            None => return Ok(()),
        };

        let surface = match self.gl_surface.take() {
            Some(gl_surface) => gl_surface,
            None => return Ok(()),
        };

        if let Err(e) = ctx.make_current(&surface) {
            println!("{e:?}");
        }

        if let Err(r) = self.av_info.video.graphic_api.try_destroy_ctx() {
            println!("{r:?}");
        }

        if let Some(mut renderer) = self.renderer.take() {
            renderer.de_init(&self.av_info);
        }

        let ctx = match ctx.make_not_current() {
            Ok(ctx) => ctx,
            Err(e) => {
                println!("{e:?}");
                return Err(ErrorHandle::new("window context not dropped"));
            }
        };

        drop(surface);
        drop(ctx);

        // limpa resolver (evita ponteiro velho)
        self.proc_resolve.set_loader(|_| std::ptr::null());

        Ok(())
    }

    fn init_context(&mut self) -> TinicResult<()> {
        let gl_context = create_gl_context(
            &self.window,
            &self.gl_config,
            &self.av_info.video.graphic_api,
        )?
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

        let display = self.gl_config.display();
        let resolver = self.proc_resolve.clone();

        resolver.set_loader(move |name: &str| {
            let cstr = CString::new(name).unwrap();
            display.get_proc_address(cstr.as_c_str()) as *const ()
        });

        let render = Render::new(self.gl_config.display(), &self.texture)?;

        self.renderer = Some(render);
        self.gl_context = Some(gl_context);
        self.gl_surface = Some(gl_surface);

        Ok(())
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

    fn init_frame_buffer(&mut self, av_info: &Arc<AvInfo>) -> TinicResult<()> {
        let renderer = match &mut self.renderer {
            Some(renderer) => renderer,
            None => return Ok(()),
        };

        renderer.init_frame_buffer(av_info)?;
        av_info.video.graphic_api.try_reset_ctx()
    }

    fn prepare_for_core(&self) {
        if let Some(renderer) = &self.renderer {
            renderer.prepare_for_core();
        }
    }
}

impl RetroGlWindow {
    pub fn new(
        event_loop: &ActiveEventLoop,
        av_info: &Arc<AvInfo>,
        texture: &Arc<TMutex<RawTextureData>>,
        proc_resolve: Arc<GlProcResolver>,
    ) -> Self {
        let window_size = PhysicalSize::new(800, 480);

        let attributes = Window::default_attributes()
            .with_title("Tinic")
            .with_inner_size(window_size)
            .with_transparent(false);

        let display_builder = DisplayBuilder::new().with_window_attributes(Some(attributes));
        let template = ConfigTemplateBuilder::new().with_transparency(false);

        let (window, gl_config) = display_builder
            .build(event_loop, template, |configs| {
                configs.reduce(|_, c| c).unwrap()
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
            proc_resolve,
            texture: texture.clone(),
        }
    }
}
