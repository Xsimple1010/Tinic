use crate::audio_driver::AudioDriver;
use generics::error_handle::ErrorHandle;
use retro_core::{av_info::AvInfo, RetroAudioEnvCallbacks};
use ringbuf::{storage::Heap, CachingCons, CachingProd, SharedRb};
use std::{ptr::slice_from_raw_parts, sync::Arc};

pub type BufferProd = CachingProd<Arc<SharedRb<Heap<i16>>>>;
pub type BufferCons = CachingCons<Arc<SharedRb<Heap<i16>>>>;

pub struct RetroAudio {
    drive: Arc<AudioDriver>,
}

#[derive(Default, Clone, Debug)]
pub struct AudioMetadata {
    pub channels: u16,
    pub sample_rate: u32,
}

impl RetroAudio {
    pub fn new() -> Result<Self, ErrorHandle> {
        Ok(Self {
            drive: Arc::new(AudioDriver::new()?),
        })
    }

    pub fn init(&mut self, av_info: &Arc<AvInfo>) -> Result<(), ErrorHandle> {
        self.drive.init(av_info)
    }

    pub fn play(&self) -> Result<(), ErrorHandle> {
        self.drive.play()
    }

    pub fn pause(&self) -> Result<(), ErrorHandle> {
        self.drive.pause()
    }

    pub fn stop(&self) {
        self.drive.stop();
    }

    pub fn get_core_cb(&self) -> RetroAudioCb {
        RetroAudioCb {
            drive: Arc::clone(&self.drive),
        }
    }
}

pub struct RetroAudioCb {
    drive: Arc<AudioDriver>,
}

impl RetroAudioEnvCallbacks for RetroAudioCb {
    fn audio_sample_callback(
        &self,
        left: i16,
        right: i16,
        av_info: Arc<AvInfo>,
    ) -> Result<(), ErrorHandle> {
        let metadata = AudioMetadata {
            channels: 1,
            sample_rate: *av_info
                .timing
                .sample_rate
                .try_read()
                .map_err(|_| ErrorHandle::new("Failed to read sample rate"))?,
        };

        self.drive.add_sample(&[left, right], metadata)
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
        let metadata = AudioMetadata {
            channels: 2,
            sample_rate: *av_info
                .timing
                .sample_rate
                .try_read()
                .map_err(|_| ErrorHandle::new("Failed to read sample rate"))?,
        };

        self.drive.add_sample(new_data, metadata)?;
        Ok(frames)
    }
}
