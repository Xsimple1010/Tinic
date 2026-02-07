use generics::{
    error_handle::ErrorHandle,
    retro_paths::RetroPaths,
    test_workdir::{create_test_work_dir_path, get_test_core_path},
};
use libretro_sys::binding_libretro::retro_rumble_effect;
use retro_core::{
    RetroAudioEnvCallbacks, RetroControllerEnvCallbacks, RetroCore, RetroCoreIns,
    RetroEnvCallbacks, RetroVideoEnvCallbacks, av_info::AvInfo, graphic_api::GraphicApi,
};
use std::{ptr, sync::Arc};

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
    ) -> Result<(), ErrorHandle> {
        println!("video_refresh_callback -> width:{_width} height:{_height} pitch:{_pitch}");
        Ok(())
    }

    fn context_reset(&self) -> Result<(), ErrorHandle> {
        println!("context_reset");
        Ok(())
    }

    fn get_proc_address(&self, name: &str) -> Result<*const (), ErrorHandle> {
        println!("video api request: {:?}", name);

        Ok(ptr::null())
    }

    fn context_destroy(&self) -> Result<(), ErrorHandle> {
        println!("context_destroy");

        Ok(())
    }
}

struct Audio;

impl RetroAudioEnvCallbacks for Audio {
    fn audio_sample_callback(
        &self,
        _left: i16,
        _right: i16,
        _retro_av: Arc<AvInfo>,
    ) -> Result<(), ErrorHandle> {
        Ok(())
    }

    fn audio_sample_batch_callback(
        &self,
        _data: *const i16,
        _frames: usize,
        _retro_av: Arc<AvInfo>,
    ) -> Result<usize, ErrorHandle> {
        println!("audio_sample_batch_callback -> {_frames}");
        Ok(0)
    }
}

struct Controller;

impl RetroControllerEnvCallbacks for Controller {
    fn input_poll_callback(&self) -> Result<(), ErrorHandle> {
        Ok(())
    }

    fn input_state_callback(
        &self,
        _port: i16,
        _device: i16,
        _index: i16,
        _id: i16,
    ) -> Result<i16, ErrorHandle> {
        println!("input_state_callback -> _port:{_port} device:{_device} index:{_index} id:{_id}");
        Ok(0)
    }

    fn rumble_callback(
        &self,
        port: std::os::raw::c_uint,
        effect: retro_rumble_effect,
        strength: u16,
    ) -> Result<bool, ErrorHandle> {
        println!(
            "rumble_callback -> port:{:?} effect:{:?} strength:{:?}",
            port, effect, strength
        );

        Ok(true)
    }
}

pub fn get_core_test(project_name: &str) -> Result<RetroCoreIns, ErrorHandle> {
    let test_dir = create_test_work_dir_path(project_name)
        .display()
        .to_string();

    RetroCore::new(
        &get_test_core_path(),
        RetroPaths::from_base(&test_dir)?,
        get_callbacks(),
        GraphicApi::default(),
    )
}
