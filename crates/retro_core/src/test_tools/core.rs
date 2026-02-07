use crate::core_env::{RetroControllerEnvCallbacks, RetroEnvCallbacks};
use crate::graphic_api::GraphicApi;
use crate::retro_core::RetroCore;
use crate::test_tools::constants::CORE_TEST_RELATIVE_PATH;
use crate::test_tools::paths::get_paths;
use crate::{RetroAudioEnvCallbacks, RetroCoreIns, RetroVideoEnvCallbacks};
use generics::erro_handle::ErroHandle;
use libretro_sys::binding_libretro::retro_rumble_effect;
use std::ptr;

pub fn get_callbacks() -> RetroEnvCallbacks {
    RetroEnvCallbacks {
        video: Box::new(Video {}),
        audio: Box::new(Audio {}),
        controller: Box::new(Controller {}),
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
    ) -> Result<(), ErroHandle> {
        println!("video_refresh_callback -> width:{_width} height:{_height} pitch:{_pitch}");
        Ok(())
    }

    fn context_destroy(&self) -> Result<(), ErroHandle> {
        println!("context_destroy");

        Ok(())
    }

    fn context_reset(&self) -> Result<(), ErroHandle> {
        println!("context_reset");
        Ok(())
    }

    fn get_proc_address(&self, name: &str) -> Result<*const (), ErroHandle> {
        println!("video api request: {:?}", name);

        Ok(ptr::null())
    }
}

struct Audio;

impl RetroAudioEnvCallbacks for Audio {
    fn audio_sample_callback(&self, _left: i16, _right: i16) -> Result<(), ErroHandle> {
        Ok(())
    }

    fn audio_sample_batch_callback(
        &self,
        _data: *const i16,
        _frames: usize,
    ) -> Result<usize, ErroHandle> {
        println!("audio_sample_batch_callback -> {_frames}");
        Ok(0)
    }
}

struct Controller;

impl RetroControllerEnvCallbacks for Controller {
    fn input_poll_callback(&self) -> Result<(), ErroHandle> {
        Ok(())
    }

    fn input_state_callback(
        &self,
        _port: i16,
        _device: i16,
        _index: i16,
        _id: i16,
    ) -> Result<i16, ErroHandle> {
        println!("input_state_callback -> _port:{_port} device:{_device} index:{_index} id:{_id}");
        Ok(0)
    }

    fn rumble_callback(
        &self,
        port: std::os::raw::c_uint,
        effect: retro_rumble_effect,
        strength: u16,
    ) -> Result<bool, ErroHandle> {
        println!(
            "rumble_callback -> port:{:?} effect:{:?} strength:{:?}",
            port, effect, strength
        );

        Ok(true)
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
