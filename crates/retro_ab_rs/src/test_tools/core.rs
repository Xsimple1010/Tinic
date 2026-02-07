use crate::core::CoreWrapper;
use crate::environment::RetroEnvCallbacks;
use crate::graphic_api::GraphicApi;
use crate::retro_sys::retro_rumble_effect;
use crate::test_tools::constants::CORE_TEST_RELATIVE_PATH;
use crate::test_tools::paths::get_paths;
use std::ptr;
use std::sync::Arc;
use uuid::Uuid;

fn audio_sample_callback(_left: i16, _right: i16) {}

fn audio_sample_batch_callback(_data: *const i16, _frames: usize) -> usize {
    println!("audio_sample_batch_callback -> {_frames}");
    0
}

fn input_poll_callback() {}

fn input_state_callback(_port: i16, _device: i16, _index: i16, _id: i16) -> i16 {
    println!("input_state_callback -> _port:{_port} device:{_device} index:{_index} id:{_id}");
    0
}

fn video_refresh_callback(
    _data: *const std::os::raw::c_void,
    _width: u32,
    _height: u32,
    _pitch: usize,
) {
    println!("video_refresh_callback -> width:{_width} height:{_height} pitch:{_pitch}")
}

fn rumble_callback(port: std::os::raw::c_uint, effect: retro_rumble_effect, strength: u16) -> bool {
    println!(
        "rumble_callback -> port:{:?} effect:{:?} strength:{:?}",
        port, effect, strength
    );

    true
}

fn context_destroy() {
    println!("context_destroy");
}

fn context_reset() {
    println!("context_reset");
}

fn get_proc_address(name: &str) -> *const () {
    println!("video api request: {:?}", name);

    ptr::null()
}

pub fn get_callbacks() -> RetroEnvCallbacks {
    RetroEnvCallbacks {
        audio_sample_batch_callback,
        audio_sample_callback,
        input_poll_callback,
        input_state_callback,
        video_refresh_callback,
        rumble_callback,
        context_destroy,
        context_reset,
        get_proc_address,
    }
}

pub fn get_core_wrapper() -> Arc<CoreWrapper> {
    CoreWrapper::new(
        Uuid::new_v4(),
        CORE_TEST_RELATIVE_PATH,
        get_paths().unwrap(),
        get_callbacks(),
        GraphicApi::default(),
    )
    .unwrap()
}
