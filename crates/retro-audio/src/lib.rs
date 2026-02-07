extern crate cpal;
extern crate libretro_sys;
extern crate rubato;

mod audio_driver;
mod audio_resample;
mod audios;

pub use audios::{AudioMetadata, RetroAudio};
