use crate::{audio_driver::AudioDriver, SyncData};
use generics::{
    error_handle::ErrorHandle,
    types::{ArcTMutex, TMutex},
};
use retro_core::{av_info::AvInfo, RetroAudioEnvCallbacks};
use std::{ptr::slice_from_raw_parts, sync::Arc};

pub struct RetroAudio {
    drive: ArcTMutex<AudioDriver>,
}

impl RetroAudio {
    pub fn new(sync_data: ArcTMutex<SyncData>) -> Result<Self, ErrorHandle> {
        Ok(Self {
            drive: TMutex::new(AudioDriver::new(sync_data)?),
        })
    }

    pub fn get_core_cb(&self) -> RetroAudioCb {
        RetroAudioCb {
            driver: self.drive.clone(),
        }
    }
}

pub struct RetroAudioCb {
    driver: ArcTMutex<AudioDriver>,
}

impl RetroAudioEnvCallbacks for RetroAudioCb {
    fn audio_sample_callback(
        &self,
        left: i16,
        right: i16,
        av_info: Arc<AvInfo>,
    ) -> Result<(), ErrorHandle> {
        self.driver
            .try_load()?
            .add_sample(&[left, right], 1, av_info)
    }

    fn audio_sample_batch_callback(
        &self,
        data: *const i16,
        frames: usize,
        av_info: Arc<AvInfo>,
    ) -> Result<usize, ErrorHandle> {
        if data.is_null() {
            return Ok(0);
        }

        let new_data = unsafe { &*slice_from_raw_parts(data, frames * 2) };
        self.driver
            .try_load()?
            .add_sample(new_data, frames, av_info)?;

        Ok(frames)
    }
}
