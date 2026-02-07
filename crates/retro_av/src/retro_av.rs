use crate::audios::RetroAudioCb;
use crate::sync::RetroSync;
use crate::video::RetroVideo;
use crate::{audios::RetroAudio, video::RetroVideoCb};
use generics::erro_handle::ErroHandle;
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
    pub fn new() -> Result<Self, ErroHandle> {
        let video = RetroVideo::new();
        let audio = RetroAudio::new()?;

        Ok(Self {
            video,
            audio,
            sync: RetroSync::default(),
            av_info: None,
        })
    }

    pub fn build_window(
        &mut self,
        av_info: &Arc<AvInfo>,
        event_loop: &ActiveEventLoop,
    ) -> Result<(), ErroHandle> {
        self.video.init(av_info, event_loop)?;
        self.av_info.replace(av_info.clone());

        Ok(())
    }

    pub fn destroy_window(&mut self) {
        self.av_info.take();
        self.video.destroy_window();
    }

    pub fn redraw_request(&self) -> Result<(), ErroHandle> {
        self.video.request_redraw()
    }

    pub fn get_new_frame(&mut self) -> Result<(), ErroHandle> {
        if let Some(av_info) = &self.av_info {
            self.audio.resume_new_frame(av_info)?;
            self.video.draw_new_frame(av_info)?;
        }

        Ok(())
    }

    pub fn print_screen(&self, out_path: &Path) -> Result<(), ErroHandle> {
        if let Some(av_info) = &self.av_info {
            self.video.print_screen(out_path, av_info)
        } else {
            Ok(())
        }
    }

    pub fn set_full_screen(&mut self, mode: Fullscreen) -> Result<(), ErroHandle> {
        self.video.set_full_screen(mode)
    }

    pub fn sync(&mut self) -> bool {
        if let Some(av_info) = &self.av_info {
            let fps = av_info.timing.fps.read().unwrap().abs();
            self.sync.sync(fps)
        } else {
            false
        }
    }

    pub fn get_core_cb(&self) -> (RetroVideoCb, RetroAudioCb) {
        let video_cb = self.video.get_core_cb();
        let audio_cb = self.audio.get_core_cb();

        (video_cb, audio_cb)
    }
}
