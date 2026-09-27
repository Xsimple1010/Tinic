use crate::audio_driver::AudioDriver;
use retro_core::{RetroAudioEnvCallbacks, av_info::AvInfo};
use ringbuf::{CachingCons, CachingProd, SharedRb, storage::Heap};
use std::{ptr::slice_from_raw_parts, sync::Arc};
use tinic_generics::error_handle::{ErrorHandle, TinicResult};

pub type BufferProd = CachingProd<Arc<SharedRb<Heap<f32>>>>;
pub type BufferCons = CachingCons<Arc<SharedRb<Heap<f32>>>>;

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

    pub fn init(&mut self, av_info: &Arc<AvInfo>) -> TinicResult<()> {
        self.drive.init(av_info)
    }

    pub fn play(&self) -> TinicResult<()> {
        self.drive.play()
    }

    pub fn pause(&self) -> TinicResult<()> {
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
    ) -> TinicResult<()> {
        let metadata = AudioMetadata {
            channels: 2,
            sample_rate: *av_info
                .timing
                .sample_rate
                .try_read()
                .map_err(|_| ErrorHandle::new("Failed to read sample rate"))?,
        };

        // Converte i16 → f32 (normalizado entre -1.0 e 1.0)
        let left_f32 = left as f32 / i16::MAX as f32;
        let right_f32 = right as f32 / i16::MAX as f32;

        self.drive.add_sample(&[left_f32, right_f32], metadata)
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

        let samples_i16 = unsafe { &*slice_from_raw_parts(data, frames * 2) };

        let samples_f32: Vec<f32> = samples_i16
            .iter()
            .map(|&s| s as f32 / i16::MAX as f32)
            .collect();

        let metadata = AudioMetadata {
            channels: 2,
            sample_rate: *av_info
                .timing
                .sample_rate
                .try_read()
                .map_err(|_| ErrorHandle::new("Failed to read sample rate"))?,
        };

        self.drive.add_sample(&samples_f32, metadata)?;
        Ok(frames)
    }
}
