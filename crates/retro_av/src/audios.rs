use generics::{
    error_handle::ErrorHandle,
    types::{ArcTMutex, TMutex},
};
use retro_core::{av_info::AvInfo, RetroAudioEnvCallbacks};
use rodio::{buffer::SamplesBuffer, OutputStream, OutputStreamHandle, Sink};
use std::{
    cell::UnsafeCell,
    ptr::{null, slice_from_raw_parts},
    sync::Arc,
};

pub struct AudioNewFrame {
    pub data: *const i16,
    pub frames: usize,
    pub channel: u16,
}

pub struct RetroAudio {
    _stream_handle: OutputStreamHandle,
    _stream: OutputStream,
    sink: Sink,
    buffer: ArcTMutex<UnsafeCell<AudioNewFrame>>,
}

impl RetroAudio {
    pub fn new() -> Result<Self, ErrorHandle> {
        let (stream, stream_handle) = match OutputStream::try_default() {
            Ok(out) => out,
            Err(e) => {
                return Err(ErrorHandle {
                    message: e.to_string(),
                })
            }
        };

        let sink: Sink = match Sink::try_new(&stream_handle) {
            Ok(sink) => sink,
            Err(e) => {
                return Err(ErrorHandle {
                    message: e.to_string(),
                })
            }
        };

        Ok(Self {
            _stream: stream,
            _stream_handle: stream_handle,
            sink,
            buffer: TMutex::new(UnsafeCell::new(AudioNewFrame {
                data: null(),
                frames: 0,
                channel: 2,
            })),
        })
    }

    pub fn resume_new_frame(&mut self, av_info: &Arc<AvInfo>) -> Result<(), ErrorHandle> {
        if let Ok(sample_rate) = av_info.timing.sample_rate.read() {
            let buffer = unsafe { self.buffer.try_load()?.get().read() };

            if buffer.data.is_null() {
                Ok(())
            } else {
                let data = unsafe { &*slice_from_raw_parts(buffer.data, buffer.frames * 2) };

                let sample_buffer = SamplesBuffer::new(buffer.channel, *sample_rate as u32, data);

                self.sink.append(sample_buffer);

                Ok(())
            }
        } else {
            Ok(())
        }
    }

    pub fn get_core_cb(&self) -> RetroAudioCb {
        RetroAudioCb {
            buffer: self.buffer.clone(),
        }
    }
}

pub struct RetroAudioCb {
    buffer: ArcTMutex<UnsafeCell<AudioNewFrame>>,
}

impl RetroAudioEnvCallbacks for RetroAudioCb {
    fn audio_sample_callback(&self, left: i16, right: i16) -> Result<(), ErrorHandle> {
        let mut buffer = self.buffer.try_load()?;
        let buffer = buffer.get_mut();

        buffer.data = [left, right].as_ptr();
        buffer.frames = 1;
        buffer.channel = 2;

        Ok(())
    }

    fn audio_sample_batch_callback(
        &self,
        data: *const i16,
        frames: usize,
    ) -> Result<usize, ErrorHandle> {
        let mut buffer = self.buffer.try_load()?;
        let buffer = buffer.get_mut();

        buffer.data = data;
        buffer.frames = frames;
        buffer.channel = 2;

        Ok(frames)
    }
}
