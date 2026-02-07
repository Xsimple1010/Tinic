use crate::audios::RetroAudio;
use crate::sync::RetroSync;
use crate::video::RetroVideo;
use generics::erro_handle::ErroHandle;
use libretro_sys::binding_libretro::retro_log_level::RETRO_LOG_ERROR;
use retro_ab::core::AvInfo;
use sdl2::{EventPump, Sdl};
use std::sync::Arc;

pub struct RetroAv {
    pub video: RetroVideo,
    pub audio: RetroAudio,
    sync: RetroSync,
    av_info: Arc<AvInfo>,
    _sdl: Sdl,
}

impl RetroAv {
    #[doc = "cria uma nova instancia de RetroAv. sempre mantenha a instancia dentro da thread onde foi criada!"]
    pub fn new(av_info: Arc<AvInfo>) -> Result<(RetroAv, EventPump), ErroHandle> {
        let _sdl = {
            match sdl2::init() {
                Ok(sdl) => sdl,
                Err(message) => {
                    return Err(ErroHandle {
                        level: RETRO_LOG_ERROR,
                        message,
                    })
                }
            }
        };

        let event_pump = match _sdl.event_pump() {
            Ok(event_pump) => event_pump,
            Err(message) => {
                return Err(ErroHandle {
                    level: RETRO_LOG_ERROR,
                    message,
                })
            }
        };

        let video = RetroVideo::new(&_sdl, &av_info)?;
        let audio = RetroAudio::new(&av_info)?;

        Ok((
            RetroAv {
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
