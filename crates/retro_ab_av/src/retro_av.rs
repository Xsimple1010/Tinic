use std::sync::Arc;

use crate::audios::RetroAudio;
use crate::sync::RetroSync;
use crate::video::RetroVideo;
use retro_ab::{core::AvInfo, erro_handle::ErroHandle, retro_sys::retro_log_level};
use sdl2::{EventPump, Sdl};

pub struct RetroAvCtx {
    pub video: RetroVideo,
    pub audio: RetroAudio,
    sync: RetroSync,
    av_info: Arc<AvInfo>,
    _sdl: Sdl,
}

impl RetroAvCtx {
    #[doc = "cria uma nova instancia de RetroAvCtx. sempre mantenha a instancia dentro da thread onde foi criada!"]
    pub fn new(av_info: Arc<AvInfo>) -> Result<(RetroAvCtx, EventPump), ErroHandle> {
        let _sdl = {
            match sdl2::init() {
                Ok(sdl) => sdl,
                Err(message) => {
                    return Err(ErroHandle {
                        level: retro_log_level::RETRO_LOG_ERROR,
                        message,
                    })
                }
            }
        };

        let event_pump = match _sdl.event_pump() {
            Ok(event_pump) => event_pump,
            Err(message) => {
                return Err(ErroHandle {
                    level: retro_log_level::RETRO_LOG_ERROR,
                    message,
                })
            }
        };

        let video = RetroVideo::new(&_sdl, &av_info)?;
        let audio = RetroAudio::init(&av_info)?;

        Ok((
            RetroAvCtx {
                video,
                audio,
                _sdl,
                sync: RetroSync::default(),
                av_info: av_info.clone(),
            },
            event_pump,
        ))
    }

    pub fn get_new_frame(&mut self) {
        self.audio.resume_new_frame();
        self.video.draw_new_frame();
    }

    pub fn sync(&mut self) -> bool {
        let fps = self.av_info.timing.fps.read().unwrap().abs();
        self.sync.sync(fps)
    }
}
