use crate::audios::RetroAudioCb;
use crate::sync::RetroSync;
use crate::video::RetroVideo;
use crate::{audios::RetroAudio, video::RetroVideoCb};
use generics::error_handle::ErrorHandle;
use retro_core::av_info::AvInfo;
use std::path::Path;
use std::sync::Arc;
use winit::event_loop::ActiveEventLoop;
use winit::window::Fullscreen;

pub struct RetroAv {
    video: RetroVideo,
    audio: RetroAudio,
    sync: RetroSync,
}

impl RetroAv {
    #[doc = "cria uma nova instancia de RetroAv. sempre mantenha a instancia dentro da thread onde foi criada!"]
    pub fn new() -> Result<Self, ErrorHandle> {
        let sync = RetroSync::new(0.0002);
        let video = RetroVideo::new();
        let audio = RetroAudio::new()?;

        Ok(Self { video, audio, sync })
    }

    pub fn build_window(
        &mut self,
        av_info: &Arc<AvInfo>,
        event_loop: &ActiveEventLoop,
    ) -> Result<(), ErrorHandle> {
        self.video.init(av_info, event_loop)?;
        self.audio.init(av_info)?;
        Ok(())
    }

    pub fn suspend_window(&mut self) {
        self.video.destroy_window();
        self.audio.pause().unwrap();
    }

    pub fn destroy(&self) {
        self.video.destroy_window();
        self.audio.stop();
    }

    pub fn redraw_request(&self) -> Result<(), ErrorHandle> {
        self.audio.play()?;
        self.video.request_redraw()
    }

    pub fn prepare_to_sync(&mut self, av_info: &Arc<AvInfo>) -> Result<(), ErrorHandle> {
        self.sync.prepare_sync_data(av_info)
    }

    pub fn sync_now(&mut self) -> Result<(), ErrorHandle> {
        self.sync.sync_now()
    }

    pub fn print_screen(&self, out_path: &Path, av_info: &Arc<AvInfo>) -> Result<(), ErrorHandle> {
        self.video.print_screen(out_path, av_info)
    }

    pub fn set_full_screen(&mut self, mode: Fullscreen) -> Result<(), ErrorHandle> {
        self.video.set_full_screen(mode)
    }

    pub fn get_core_cb(&self) -> (RetroVideoCb, RetroAudioCb) {
        let video_cb = self.video.get_core_cb();
        let audio_cb = self.audio.get_core_cb();

        (video_cb, audio_cb)
    }
}
