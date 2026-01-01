use crate::graphic_api::GraphicApi;
use crate::tools::validation::InputValidator;
use generics::error_handle::ErrorHandle;
use generics::types::{ArcTMutex, TMutex};
use libretro_sys::binding_libretro::{
    retro_game_geometry, retro_pixel_format::{self, RETRO_PIXEL_FORMAT_UNKNOWN},
    retro_system_av_info,
    retro_system_timing, LibretroRaw,
};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, RwLock};

#[derive(Default, Debug)]
pub struct Timing {
    #[doc = "FPS of video content."]
    pub fps: RwLock<f64>,
    #[doc = "Sampling rate of audio."]
    pub sample_rate: RwLock<u32>,
}

#[derive(Debug, Default)]
pub struct Geometry {
    #[doc = "Nominal video width of game."]
    pub base_width: AtomicU32,

    #[doc = "Nominal video height of game."]
    pub base_height: AtomicU32,

    #[doc = "Maximum possible width of game."]
    pub max_width: AtomicU32,

    #[doc = "Maximum possible height of game."]
    pub max_height: AtomicU32,

    #[doc = "Nominal aspect ratio of game. If
    aspect_ratio is <= 0.0, an aspect ratio
    of base_width / base_height is assumed.
    A frontend could override this setting,
    if desired."]
    pub aspect_ratio: RwLock<f32>,
}

#[derive(Debug)]
pub struct Video {
    pub can_dupe: RwLock<bool>,
    pub pixel_format: ArcTMutex<retro_pixel_format>,
    pub geometry: Geometry,
    pub graphic_api: GraphicApi,
}

impl Default for Video {
    fn default() -> Self {
        Video {
            can_dupe: RwLock::new(false),
            pixel_format: TMutex::new(RETRO_PIXEL_FORMAT_UNKNOWN),
            geometry: Geometry::default(),
            graphic_api: GraphicApi::default(),
        }
    }
}

#[derive(Debug)]
pub struct AvInfo {
    pub video: Video,
    pub timing: Timing,
}

impl AvInfo {
    pub fn new(graphic_api: GraphicApi) -> Self {
        Self {
            video: Video {
                graphic_api,
                ..Default::default()
            },
            timing: Timing::default(),
        }
    }

    pub fn try_set_new_geometry(
        &self,
        raw_geometry: &retro_game_geometry,
    ) -> Result<(), ErrorHandle> {
        let geometry = &self.video.geometry;

        match geometry.aspect_ratio.write() {
            Ok(mut aspect_ratio) => {
                *aspect_ratio = raw_geometry.aspect_ratio;
            }
            Err(_) => {
                return Err(ErrorHandle::new(
                    "nao foi possível atualiza o aspect_ratio da textura",
                ));
            }
        }

        geometry
            .base_height
            .store(raw_geometry.base_height, Ordering::SeqCst);
        geometry
            .base_width
            .store(raw_geometry.base_width, Ordering::SeqCst);
        geometry
            .max_height
            .store(raw_geometry.max_height, Ordering::SeqCst);
        geometry
            .max_width
            .store(raw_geometry.max_width, Ordering::SeqCst);

        Ok(())
    }

    fn _set_timing(
        &self,
        raw_system_timing: *const retro_system_timing,
    ) -> Result<(), ErrorHandle> {
        InputValidator::validate_non_null_ptr(raw_system_timing, "raw_system_timing")?;

        let timing = unsafe { *raw_system_timing };

        *self.timing.fps.write()? = timing.fps;
        InputValidator::validate_sample_rate(timing.sample_rate as u32)?;
        *self.timing.sample_rate.write()? = timing.sample_rate as u32;

        Ok(())
    }

    pub fn update_av_info(&self, core_raw: &Arc<LibretroRaw>) -> Result<(), ErrorHandle> {
        let mut raw_av_info = retro_system_av_info {
            geometry: retro_game_geometry {
                aspect_ratio: 0.0,
                base_height: 0,
                base_width: 0,
                max_height: 0,
                max_width: 0,
            },
            timing: retro_system_timing {
                fps: 0.0,
                sample_rate: 0.0,
            },
        };

        unsafe {
            core_raw.retro_get_system_av_info(&mut raw_av_info);
            self.try_set_new_geometry(&raw_av_info.geometry)?;
        }

        self._set_timing(&raw_av_info.timing)?;

        Ok(())
    }
}
