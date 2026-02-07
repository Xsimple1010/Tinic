extern crate libretro_sys;
extern crate retro_audio;
extern crate retro_controllers;
extern crate retro_core;
extern crate retro_video;
extern crate tinic_generics;

mod app;
mod app_dispatcher;
mod device_listener;
mod tinic;

pub use app::{GameInstance, listener::*};
pub use app_dispatcher::GameInstanceDispatchers;
pub use retro_controllers::{
    RetroController, RetroGamePad,
    devices_manager::{DeviceListener, DeviceStateListener},
};
pub use retro_core::args_manager;
pub use tinic::*;
pub use tinic_generics::error_handle::ErrorHandle;
pub use tinic_generics::retro_paths::RetroPaths;
