use crate::SyncData;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Stream,
};
use generics::{
    error_handle::ErrorHandle,
    types::{ArcTMutex, TMutex},
};
use retro_core::av_info::AvInfo;
use rubato::SincFixedIn;
use std::{collections::VecDeque, result::Result, sync::Arc, time::Duration};

pub struct AudioNewFrame {
    pub data: VecDeque<i16>,
    pub frames: usize,
    pub channel: u16,
}

pub struct AudioDriver {
    stream: Stream,
    buffer: ArcTMutex<AudioNewFrame>,
    sync_data: ArcTMutex<SyncData>,
    sample_rate: u32,
    resampler: ArcTMutex<Option<SincFixedIn<f64>>>,
    temp_buffer: ArcTMutex<Vec<i16>>, // Buffer temporário
}

impl AudioDriver {
    pub fn new(sync_data: ArcTMutex<SyncData>) -> Result<Self, ErrorHandle> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| ErrorHandle::new("No output device"))?;
        let config = device
            .default_output_config()
            .map_err(|e| ErrorHandle::new(&e.to_string()))?;
        let sample_rate = config.sample_rate().0;
        let channels = config.channels();
        eprintln!("cpal sample_rate -> {}", sample_rate);

        let buffer = TMutex::new(AudioNewFrame {
            data: VecDeque::new(),
            frames: 0,
            channel: channels,
        });

        let stream = {
            let buffer_clone = buffer.clone();
            let this = &device;
            let config = &config.into();
            let error_callback = |err| eprintln!("erro no stream {}", err);
            let timeout = Some(Duration::ZERO); // Reduzido para 16ms
            let data_callback = move |data: &mut [i16], _: &cpal::OutputCallbackInfo| {
                let mut buffer = buffer_clone.try_load().unwrap();
                // eprintln!(
                //     "cpal requested: {}, buffer size: {}",
                //     data.len(),
                //     buffer.data.len()
                // );
                if buffer.data.is_empty() || buffer.frames == 0 {
                    data.fill(0);
                    return;
                }
                let len = data.len().min(buffer.data.len());
                for (i, sample) in buffer.data.drain(..len).enumerate() {
                    data[i] = sample;
                }
                buffer.frames = buffer.data.len() / buffer.channel as usize;
                if len < data.len() {
                    data[len..].fill(0);
                }
            };

            this.build_output_stream(config, data_callback, error_callback, timeout)
        }
        .map_err(|e| ErrorHandle::new(&e.to_string()))?;

        stream
            .play()
            .map_err(|e| ErrorHandle::new(&e.to_string()))?;

        Ok(Self {
            stream,
            buffer,
            sync_data,
            sample_rate,
            resampler: TMutex::new(None),
            temp_buffer: TMutex::new(Vec::new()),
        })
    }

    pub fn add_sample(
        &self,
        data: &[i16],
        frames: usize,
        av_info: Arc<AvInfo>,
    ) -> Result<(), ErrorHandle> {
        println!(
            "retro sample_rate -> {}",
            av_info.timing.sample_rate.read()?
        );

        let mut buffer = self.buffer.try_load()?;
        buffer.data.extend(data);
        buffer.frames = frames;
        Ok(())
    }
}
