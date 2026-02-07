use generics::erro_handle::ErroHandle;
use libretro_sys::binding_libretro::retro_log_level::RETRO_LOG_ERROR;
use retro_core::{av_info::AvInfo, RetroAudioEnvCallbacks};
use rodio::{buffer::SamplesBuffer, OutputStream, OutputStreamHandle, Sink};
use std::{
    ptr::{null, slice_from_raw_parts},
    sync::Arc,
};

struct AudioNewFrame {
    data: *const i16,
    frames: usize,
    channel: u16,
}

static mut NEW_FRAME: AudioNewFrame = AudioNewFrame {
    data: null(),
    frames: 0,
    channel: 2,
};

pub struct RetroAudio {
    _stream_handle: OutputStreamHandle,
    _stream: OutputStream,
    sink: Sink,
}

impl RetroAudio {
    pub fn new() -> Result<Self, ErroHandle> {
        let (stream, stream_handle) = match OutputStream::try_default() {
            Ok(out) => out,
            Err(e) => {
                return Err(ErroHandle {
                    level: RETRO_LOG_ERROR,
                    message: e.to_string(),
                })
            }
        };

        let sink: Sink = match Sink::try_new(&stream_handle) {
            Ok(sink) => sink,
            Err(e) => {
                return Err(ErroHandle {
                    level: RETRO_LOG_ERROR,
                    message: e.to_string(),
                })
            }
        };

        Ok(Self {
            _stream: stream,
            _stream_handle: stream_handle,
            sink,
        })
    }

    pub fn resume_new_frame(&mut self, av_info: &Arc<AvInfo>) {
        if let Ok(sample_rate) = av_info.timing.sample_rate.read() {
            let data = unsafe { &*slice_from_raw_parts(NEW_FRAME.data, NEW_FRAME.frames * 2) };

            let channel = unsafe { NEW_FRAME.channel };

            let sample_buffer = SamplesBuffer::new(channel, *sample_rate as u32, data);

            self.sink.append(sample_buffer);
        }
    }

    pub fn get_core_cb(&self) -> RetroAudioCb {
        RetroAudioCb {}
    }
}

pub struct RetroAudioCb;

impl RetroAudioEnvCallbacks for RetroAudioCb {
    fn audio_sample_batch_callback(&self, data: *const i16, frames: usize) -> usize {
        unsafe {
            NEW_FRAME = AudioNewFrame {
                data,
                frames,
                channel: 2,
            };
        }

        frames
    }

    fn audio_sample_callback(&self, left: i16, right: i16) {
        println!("audio_sample_callback");

        unsafe {
            NEW_FRAME = AudioNewFrame {
                data: [left, right].as_ptr(),
                frames: 1,
                channel: 1,
            };
        }
    }
}
