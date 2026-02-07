extern crate generics;
extern crate gilrs;

mod gamepad;
mod keyboard;
mod retro_controller;
mod state_thread;

pub use gamepad::gamepad_key_map::GamepadKeyMap;
pub mod devices_manager;
pub use retro_controller::{RetroController, RetroControllerCb};
