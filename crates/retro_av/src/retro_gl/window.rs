use super::{gl::gl, render::Render};
use crate::video::RetroVideoAPi;
use generics::erro_handle::ErroHandle;
use libretro_sys::binding_libretro::{
    retro_hw_context_type::{
        RETRO_HW_CONTEXT_NONE, RETRO_HW_CONTEXT_OPENGL, RETRO_HW_CONTEXT_OPENGL_CORE,
    },
    retro_log_level::RETRO_LOG_ERROR,
};
use retro_core::core::AvInfo;
use sdl2::video::FullScreenType;
use sdl2::{
    video::{GLContext, GLProfile, Window},
    Sdl, VideoSubsystem,
};
use std::sync::atomic::Ordering;
use std::{rc::Rc, sync::Arc};

pub struct GlWindow {
    video: VideoSubsystem,
    window: Window,
    gl_ctx: Option<GLContext>,
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
        {
            self.gl_ctx.take();
        }

        self.video.gl_unload_library();
    }
}

impl RetroVideoAPi for GlWindow {
    fn get_window_id(&self) -> u32 {
        self.window.id()
    }

    fn draw_new_frame(&self, texture: &crate::video::RawTextureData) {
        let (width, height) = self.window.size();

        self.render.draw_new_frame(
            texture,
            &self.av_info.video.geometry,
            width as i32,
            height as i32,
        );

        self.window.gl_swap_window();
    }

    fn resize(&mut self, (width, height): (u32, u32)) {
        self.window.set_size(width, height).unwrap();
    }

    fn get_proc_address(&self, proc_name: &str) -> *const () {
        self.video.gl_get_proc_address(proc_name)
    }

    fn context_destroy(&mut self) {
        println!("context_destroy");
    }

    fn context_reset(&mut self) {
        println!("context_reset");
    }

    fn enable_full_screen(&mut self) {
        self.window.set_fullscreen(FullScreenType::True).unwrap()
    }

    fn disable_full_screen(&mut self) {
        self.window.set_fullscreen(FullScreenType::Off).unwrap()
    }
}

impl GlWindow {
    pub fn new(sdl: &Sdl, av_info: &Arc<AvInfo>) -> Result<GlWindow, ErroHandle> {
        let video = match sdl.video() {
            Ok(sdl) => sdl,
            Err(message) => {
                return Err(ErroHandle {
                    level: RETRO_LOG_ERROR,
                    message,
                })
            }
        };

        let graphic_api = &av_info.video.graphic_api;
        let gl_attr = video.gl_attr();

        match graphic_api.context_type {
            RETRO_HW_CONTEXT_OPENGL_CORE | RETRO_HW_CONTEXT_NONE => {
                gl_attr.set_context_profile(GLProfile::Core);
            }
            RETRO_HW_CONTEXT_OPENGL => {
                gl_attr.set_context_profile(GLProfile::Compatibility);
            }
            _ => {
                return Err(ErroHandle {
                    level: RETRO_LOG_ERROR,
                    message: "api selecionado nao e compativel".to_string(),
                })
            }
        }

        let mut major = graphic_api.major.load(Ordering::SeqCst) as u8;
        let mut minor = graphic_api.minor.load(Ordering::SeqCst) as u8;

        if major == 0 {
            major = 3;
            minor = 2;
        }

        gl_attr.set_context_version(major, minor);

        let geo = &av_info.video.geometry;

        let win_result = video
            .window(
                "retro_av",
                geo.base_width.load(Ordering::SeqCst),
                geo.base_height.load(Ordering::SeqCst),
            )
            .opengl()
            .maximized()
            .resizable()
            .position_centered()
            .build();

        match win_result {
            Ok(mut window) => {
                let gl_ctx = window.gl_create_context().unwrap();
                let gl = Rc::new(gl::Gl::load_with(|name| {
                    video.gl_get_proc_address(name) as *const _
                }));

                let _ = video.gl_set_swap_interval(1);

                let result = window.set_minimum_size(
                    geo.base_width.load(Ordering::SeqCst),
                    geo.base_height.load(Ordering::SeqCst),
                );

                if let Err(e) = result {
                    return Err(ErroHandle {
                        level: RETRO_LOG_ERROR,
                        message: e.to_string(),
                    });
                }

                let render = Render::new(av_info, gl.clone())?;

                Ok(GlWindow {
                    video,
                    window,
                    gl_ctx: Some(gl_ctx),
                    render,
                    av_info: av_info.clone(),
                })
            }
            Err(e) => Err(ErroHandle {
                level: RETRO_LOG_ERROR,
                message: e.to_string(),
            }),
        }
    }
}
