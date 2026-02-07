use crate::core::retro_pixel_format::{self, RETRO_PIXEL_FORMAT_UNKNOWN};
use crate::graphic_api::GraphicApi;
use libretro_sys::binding_libretro::{
    retro_game_geometry, retro_system_av_info, retro_system_timing, LibretroRaw,
};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, RwLock};

#[derive(Default, Debug)]
pub struct Timing {
    #[doc = "FPS of video content."]
    pub fps: RwLock<f64>,
    #[doc = "Sampling rate of audio."]
    pub sample_rate: RwLock<f64>,
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
    pub pixel_format: RwLock<retro_pixel_format>,
    pub geometry: Geometry,
    pub graphic_api: GraphicApi,
}

impl Default for Video {
    fn default() -> Self {
        Video {
            can_dupe: RwLock::new(false),
            pixel_format: RwLock::new(RETRO_PIXEL_FORMAT_UNKNOWN),
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

    /// # Safety
    ///
    /// Garanta que o ponteiro *raw geometry ptr* é valido antes de envia para essa função.
    pub unsafe fn try_set_new_geometry(&self, raw_geometry_ptr: *const retro_game_geometry) {
        if raw_geometry_ptr.is_null() {
            return;
        }

        let raw_geometry = unsafe { *raw_geometry_ptr };
        let geometry_ctx = &self.video.geometry;

        *geometry_ctx.aspect_ratio.write().unwrap() = raw_geometry.aspect_ratio;
        geometry_ctx
            .base_height
            .store(raw_geometry.base_height, Ordering::SeqCst);
        geometry_ctx
            .base_width
            .store(raw_geometry.base_width, Ordering::SeqCst);
        geometry_ctx
            .max_height
            .store(raw_geometry.max_height, Ordering::SeqCst);
        geometry_ctx
            .max_width
            .store(raw_geometry.max_width, Ordering::SeqCst);
    }

    fn _set_timing(&self, raw_system_timing: *const retro_system_timing) {
        if raw_system_timing.is_null() {
            return;
        }

        let timing = unsafe { *raw_system_timing };

        *self
            .timing
            .fps
            .write()
            .expect("Nao foi possível definir um novo valor para timing.fps") = timing.fps;

        *self
            .timing
            .sample_rate
            .write()
            .expect("Nao foi possível definir um novo valor para timing.sample_rate") =
            timing.sample_rate;
    }

    pub fn update_av_info(&self, core_raw: &Arc<LibretroRaw>) {
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
            self.try_set_new_geometry(&raw_av_info.geometry);
            self._set_timing(&raw_av_info.timing);
        }
    }
}
