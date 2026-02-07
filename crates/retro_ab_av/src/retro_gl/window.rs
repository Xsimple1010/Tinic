use super::{gl::gl, render::Render};
use crate::video::RetroVideoAPi;
use generics::erro_handle::ErroHandle;
use libretro_sys::binding_libretro::retro_log_level::RETRO_LOG_ERROR;
use retro_ab::core::AvInfo;
use sdl2::video::FullscreenType;
use sdl2::{
    video::{GLContext, GLProfile, Window},
    Sdl, VideoSubsystem,
};
use std::{rc::Rc, sync::Arc};

pub struct GlWIndow {
    video: VideoSubsystem,
    window: Window,
    gl_ctx: Option<GLContext>,
    render: Render,
    av_info: Arc<AvInfo>,
}

impl Drop for GlWIndow {
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

impl RetroVideoAPi for GlWIndow {
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

    fn enable_full_screen(&mut self) {
        self.window.set_fullscreen(FullscreenType::True).unwrap()
    }

    fn disable_full_screen(&mut self) {
        self.window.set_fullscreen(FullscreenType::Off).unwrap()
    }
}

impl GlWIndow {
    pub fn new(sdl: &Sdl, av_info: &Arc<AvInfo>) -> Result<GlWIndow, ErroHandle> {
        let video = match sdl.video() {
            Ok(sdl) => sdl,
            Err(message) => {
                return Err(ErroHandle {
                    level: RETRO_LOG_ERROR,
                    message,
                })
            }
        };

        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 2);

        let geo = &av_info.video.geometry;

        let win_result = video
            .window(
                "retro_ab_av",
                *geo.base_width.read().unwrap(),
                *geo.base_height.read().unwrap(),
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
                    *av_info.video.geometry.base_width.read().unwrap(),
                    *av_info.video.geometry.base_height.read().unwrap(),
                );

                if let Err(e) = result {
                    return Err(ErroHandle {
                        level: RETRO_LOG_ERROR,
                        message: e.to_string(),
                    });
                }

                let render = Render::new(av_info, gl.clone())?;

                Ok(GlWIndow {
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
