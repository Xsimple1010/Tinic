extern crate generics;
extern crate gilrs;
#[macro_use]
extern crate lazy_static;

mod gamepad;
mod keyboard;
mod retro_controller;
mod state_thread;

pub use gamepad::gamepad_key_map::GamepadKeyMap;
pub mod devices_manager;
pub use retro_controller::{
    input_poll_callback, input_state_callback, rumble_callback, RetroController,
};
