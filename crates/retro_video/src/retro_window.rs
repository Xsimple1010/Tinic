use crate::raw_texture::RawTextureData;

pub enum RetroWindowMode {
    Windowed,
    FullScreen,
}

pub trait RetroWindowContext {
    fn request_redraw(&self);

    fn draw_new_frame(&self, texture: &RawTextureData);

    fn get_proc_address(&self, proc_name: &str) -> *const ();

    fn set_window_mode(&mut self, mode: RetroWindowMode);

    fn toggle_window_model(&mut self);

    fn context_destroy(&mut self);

    fn context_reset(&mut self);

    fn resize(&mut self, width: u32, height: u32);

    fn draw_context_as_initialized(&self) -> bool;
}
