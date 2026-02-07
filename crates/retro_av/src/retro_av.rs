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
    av_info: Option<Arc<AvInfo>>,
}

impl RetroAv {
    #[doc = "cria uma nova instancia de RetroAv. sempre mantenha a instancia dentro da thread onde foi criada!"]
    pub fn new() -> Result<Self, ErrorHandle> {
        let sync = RetroSync::new(0.005);
        let video = RetroVideo::new(sync.sync_data.clone());
        let audio = RetroAudio::new(sync.sync_data.clone())?;

        Ok(Self {
            video,
            audio,
            sync,
            av_info: None,
        })
    }

    pub fn build_window(
        &mut self,
        av_info: &Arc<AvInfo>,
        event_loop: &ActiveEventLoop,
    ) -> Result<(), ErrorHandle> {
        self.video.init(av_info, event_loop)?;
        self.av_info.replace(av_info.clone());
        self.audio.set_av_info(av_info.clone());

        Ok(())
    }

    pub fn destroy_window(&mut self) {
        self.av_info.take();
        self.video.destroy_window();
    }

    pub fn redraw_request(&self) -> Result<(), ErrorHandle> {
        self.video.request_redraw()
    }

    pub fn prepare_to_sync(&mut self) -> Result<(), ErrorHandle> {
        if let Some(av_info) = &self.av_info {
            self.sync.prepare_sync_data(av_info)?
        }

        Ok(())
    }

    pub fn sync_now(&mut self) -> Result<(), ErrorHandle> {
        self.sync.sync_now()
    }

    pub fn print_screen(&self, out_path: &Path) -> Result<(), ErrorHandle> {
        if let Some(av_info) = &self.av_info {
            self.video.print_screen(out_path, av_info)
        } else {
            Ok(())
        }
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
