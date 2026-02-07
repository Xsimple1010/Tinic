use super::{
    frame_buffer::FrameBuffer,
    gl::gl::{
        self,
        types::{GLint, GLuint},
        DEPTH24_STENCIL8, DEPTH_ATTACHMENT, DEPTH_COMPONENT24, DEPTH_STENCIL_ATTACHMENT,
    },
    gl_buffer::GlBuffer,
    render_buffer::RenderBuffer,
    shader::Shader,
    shader_program::ShaderProgram,
    texture::Texture2D,
    vertex::{new_vertex, GlVertex},
    vertex_array::VertexArray,
};
use crate::video::RawTextureData;
use generics::erro_handle::ErroHandle;
use glutin::prelude::GlDisplay;
use retro_core::av_info::{AvInfo, Geometry};
use std::{ffi::CString, mem::size_of, sync::atomic::Ordering};
use std::{rc::Rc, sync::Arc};

pub struct Render {
    _program: ShaderProgram,
    _texture: Texture2D,
    _i_pos: GLint,
    _i_tex_pos: GLint,
    _u_tex: GLint,
    _vao: VertexArray,
    _vbo: GlBuffer,
    _fbo: FrameBuffer,
    _rbo: Option<RenderBuffer>,
    gl: Rc<gl::Gl>,
}

impl Render {
    fn refresh_vertex(
        &self,
        geo: &Geometry,
        origin_w: f32,
        origin_h: f32,
        window_w: i32,
        window_h: i32,
    ) {
        let vertex = new_vertex(geo, window_w as f32, window_h as f32, origin_w, origin_h);

        self._vao.bind();
        self._vbo.bind();

        self._vbo.set_data(vertex);
        self._vao
            .set_attribute::<GlVertex>(self._i_pos as GLuint, 2, 0);

        self._vao.set_attribute::<GlVertex>(
            self._i_tex_pos as GLuint,
            2,
            (size_of::<f32>() * 2) as i32,
        );

        self._vao.un_bind();
        self._vbo.un_bind();
    }

    pub fn draw_new_frame(
        &self,
        texture: &RawTextureData,
        geo: &Geometry,
        win_width: i32,
        win_height: i32,
    ) {
        unsafe {
            self.refresh_vertex(
                geo,
                texture.width as f32,
                texture.height as f32,
                win_width,
                win_height,
            );

            self.gl.Viewport(0, 0, win_width, win_height);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);

            self._texture.push(texture);
            self._program.use_program();
            self._texture.active();

            self._vao.bind();
            self.gl.DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
            self._vao.un_bind();
            self._program.un_use_program();
        }
    }

    pub fn new<D: GlDisplay>(av_info: &Arc<AvInfo>, gl_display: D) -> Result<Render, ErroHandle> {
        let vertex_shader_src = "
        #version 330 core
        in vec2 i_pos;
        in vec2 i_tex_pos;

        out vec2 f_t_pos;

        void main() {
            f_t_pos = i_tex_pos;
            gl_Position = vec4(i_pos, 0.0, 1.0);
        }
        ";

        let fragment_shader_src = "
        #version 330 core
        in vec2 f_t_pos;
        
        out vec4 FragColor;
        
        uniform sampler2D u_tex;

        void main() {
            FragColor = texture2D(u_tex, f_t_pos);
        }
        ";

        let gl = Rc::new(gl::Gl::load_with(|symbol| {
            let symbol = CString::new(symbol).unwrap();
            gl_display.get_proc_address(symbol.as_c_str()).cast()
        }));

        let vertex_shader = Shader::new(gl::VERTEX_SHADER, vertex_shader_src, gl.clone())?;
        let frag_shader = Shader::new(gl::FRAGMENT_SHADER, fragment_shader_src, gl.clone())?;

        let program = ShaderProgram::new(&[vertex_shader, frag_shader], gl.clone())?;

        let i_pos = program.get_attribute("i_pos");
        let i_tex_pos = program.get_attribute("i_tex_pos");
        let u_tex = program.get_uniform("u_tex");

        let texture = Texture2D::new(av_info, gl.clone())?;

        let vao = VertexArray::new(gl.clone());
        let vbo = GlBuffer::new(gl::ARRAY_BUFFER, gl.clone());
        let fbo = FrameBuffer::new(gl.clone());
        let mut rbo: Option<RenderBuffer> = None;

        //configura o frame_buffer e o render_buffer
        fbo.bind();
        fbo.attach_texture(&texture);

        let g_api = &av_info.video.graphic_api;
        let geo = &av_info.video.geometry;

        if g_api.depth.load(Ordering::SeqCst) && g_api.stencil.load(Ordering::SeqCst) {
            let new_rbo = RenderBuffer::new(gl.clone());

            new_rbo.bind();
            new_rbo.storage(
                DEPTH24_STENCIL8,
                geo.max_width.load(Ordering::SeqCst) as i32,
                geo.max_height.load(Ordering::SeqCst) as i32,
            );

            fbo.attach_render_buffer(DEPTH_STENCIL_ATTACHMENT, new_rbo.get_id());

            rbo.replace(new_rbo);
        } else if g_api.depth.load(Ordering::SeqCst) {
            let new_rbo = RenderBuffer::new(gl.clone());

            new_rbo.bind();
            new_rbo.storage(
                DEPTH_COMPONENT24,
                geo.max_width.load(Ordering::SeqCst) as i32,
                geo.max_height.load(Ordering::SeqCst) as i32,
            );

            fbo.attach_render_buffer(DEPTH_ATTACHMENT, new_rbo.get_id());

            rbo.replace(new_rbo);
        }

        if let Some(rbo) = &rbo {
            rbo.un_bind();
            rbo.un_bind();
        }

        fbo.un_bind();

        av_info
            .video
            .graphic_api
            .fbo
            .write()?
            .replace(fbo.get_id() as usize);

        Ok(Render {
            _program: program,
            _texture: texture,
            _i_pos: i_pos,
            _i_tex_pos: i_tex_pos,
            _u_tex: u_tex,
            _vao: vao,
            _vbo: vbo,
            _fbo: fbo,
            _rbo: rbo,
            gl,
        })
    }
}
