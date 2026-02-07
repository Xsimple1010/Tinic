use crate::core_env::RetroEnvCallbacks;
use crate::graphic_api::GraphicApi;
use crate::retro_core::RetroCore;
use crate::test_tools::constants::CORE_TEST_RELATIVE_PATH;
use crate::test_tools::paths::get_paths;
use crate::{RetroAudioEnvCallbacks, RetroCoreIns, RetroVideoEnvCallbacks};
use libretro_sys::binding_libretro::retro_rumble_effect;
use std::ptr;

fn input_poll_callback() {}

fn input_state_callback(_port: i16, _device: i16, _index: i16, _id: i16) -> i16 {
    println!("input_state_callback -> _port:{_port} device:{_device} index:{_index} id:{_id}");
    0
}

fn rumble_callback(port: std::os::raw::c_uint, effect: retro_rumble_effect, strength: u16) -> bool {
    println!(
        "rumble_callback -> port:{:?} effect:{:?} strength:{:?}",
        port, effect, strength
    );

    true
}

pub fn get_callbacks() -> RetroEnvCallbacks {
    RetroEnvCallbacks {
        input_poll_callback,
        input_state_callback,
        video: Box::new(Video {}),
        audio: Box::new(Audio {}),
        rumble_callback,
    }
}

struct Video;

impl RetroVideoEnvCallbacks for Video {
    fn video_refresh_callback(
        &self,
        _data: *const std::os::raw::c_void,
        _width: u32,
        _height: u32,
        _pitch: usize,
    ) {
        println!("video_refresh_callback -> width:{_width} height:{_height} pitch:{_pitch}")
    }

    fn context_destroy(&self) {
        println!("context_destroy");
    }

    fn context_reset(&self) {
        println!("context_reset");
    }

    fn get_proc_address(&self, name: &str) -> *const () {
        println!("video api request: {:?}", name);

        ptr::null()
    }
}

struct Audio;

impl RetroAudioEnvCallbacks for Audio {
    fn audio_sample_callback(&self, _left: i16, _right: i16) {}

    fn audio_sample_batch_callback(&self, _data: *const i16, _frames: usize) -> usize {
        println!("audio_sample_batch_callback -> {_frames}");
        0
    }
}

pub fn get_core_wrapper() -> RetroCoreIns {
    RetroCore::new(
        CORE_TEST_RELATIVE_PATH,
        get_paths().unwrap(),
        get_callbacks(),
        GraphicApi::default(),
    )
    .unwrap()
}
