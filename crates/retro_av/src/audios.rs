use crate::SyncData;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use generics::{
    error_handle::ErrorHandle,
    types::{ArcTMutex, TMutex},
};
use retro_core::{av_info::AvInfo, RetroAudioEnvCallbacks};
use std::{collections::VecDeque, time::Duration};
use std::{ptr::slice_from_raw_parts, sync::Arc};

pub struct AudioNewFrame {
    pub data: VecDeque<i16>,
    pub frames: usize,
    pub channel: u16,
}

pub struct RetroAudio {
    stream: cpal::Stream,
    buffer: ArcTMutex<AudioNewFrame>,
    sync_data: ArcTMutex<SyncData>,
    av_info: ArcTMutex<Option<Arc<AvInfo>>>,
}

impl RetroAudio {
    pub fn new(sync_data: ArcTMutex<SyncData>) -> Result<Self, ErrorHandle> {

        //start device config
        let host = cpal::default_host();
        let device = host.default_output_device().ok_or_else(|| ErrorHandle::new("No output device"))?;
        let config = device.default_output_config().map_err(|e| ErrorHandle::new(&e.to_string()))?;
        let sample_rate  = config.sample_rate().0;
        let channels: u16 = config.channels();
        eprintln!("cpal sample_rate -> {}", sample_rate);
        //#######################################



        let buffer = TMutex::new(AudioNewFrame {
            data: VecDeque::new(),
            frames: 0,
            channel: channels,
        });

        let stream = {
            let buffer_clone = buffer.clone();
            let this = &device;
            let config = &config.into();
            let error_callback = |err| {eprintln!("erro no stream {}", err)};
            let timeout = Some(Duration::from_millis(16));
            let data_callback = move|data: &mut [i16], _: &cpal::OutputCallbackInfo| {
                println!("audio");
                let mut buffer = buffer_clone.try_load().unwrap();
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

            this.build_output_stream(
                config,
                data_callback,
                error_callback,
                timeout,
            )
        }.map_err(|e| ErrorHandle::new(&e.to_string()))?;

        stream.play().map_err(|e| {ErrorHandle::new(&e.to_string())})?;

        Ok(Self {
            buffer,
            sync_data,
            stream,
            av_info: TMutex::new(None),
        })
    }

    pub fn set_av_info(&self, av_info: Arc<AvInfo>) {
        self.av_info.store(Some(av_info.clone()));
    }

    pub fn get_core_cb(&self) -> RetroAudioCb {
        RetroAudioCb {
            buffer: self.buffer.clone(),
            sync_data: self.sync_data.clone(),
            av_info: self.av_info.clone(),
        }
    }
}

pub struct RetroAudioCb {
    buffer: ArcTMutex<AudioNewFrame>,
    sync_data: ArcTMutex<SyncData>,
    av_info: ArcTMutex<Option<Arc<AvInfo>>>,
}
//impl RetroAudioCb {
//    fn cubic_interpolate(&self, y0: i16, y1: i16, y2: i16, y3: i16, t: f64) -> i16 {
//        let t2 = t * t;
//        let a0 = y3 as f64 - y2 as f64 - y0 as f64 + y1 as f64;
//        let a1 = y0 as f64 - y1 as f64 - a0;
//        let a2 = y2 as f64 - y0 as f64;
//        let a3 = y1 as f64;
//        (a0 * t * t2 + a1 * t2 + a2 * t + a3).round() as i16
//    }
//}

impl RetroAudioEnvCallbacks for RetroAudioCb {
    fn audio_sample_callback(&self, left: i16, right: i16) -> Result<(), ErrorHandle> {
        let mut buffer = self.buffer.try_load()?;
        buffer.data.extend([left, right]);
        buffer.frames = buffer.data.len() / 2;
        Ok(())
    }

    fn audio_sample_batch_callback(
        &self,
        data: *const i16,
        frames: usize,
    ) -> Result<usize, ErrorHandle> {
        if data.is_null() {
            return Ok(0);
        }

        let new_data = unsafe { &*slice_from_raw_parts(data, frames * 2) };
        let mut buffer = self.buffer.try_load()?;

        buffer.data.extend(new_data);
        buffer.frames = frames;

        Ok(frames)
    }
}
