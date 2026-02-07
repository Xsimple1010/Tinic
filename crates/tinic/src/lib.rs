extern crate generics;
extern crate libretro_sys;
extern crate retro_audio;
extern crate retro_controllers;
extern crate retro_core;
extern crate retro_video;

mod app;
mod app_dispatcher;
mod device_listener;
mod tinic;

pub use app::{GameInstance, listener::*};
pub use app_dispatcher::GameInstanceDispatchers;
pub use generics::error_handle::ErrorHandle;
pub use generics::retro_paths::RetroPaths;
pub use retro_controllers::{
    RetroController, RetroGamePad,
    devices_manager::{DeviceListener, DeviceStateListener},
};
pub use retro_core::args_manager;
pub use tinic::*;
