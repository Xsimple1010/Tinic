extern crate gilrs;
#[macro_use]
extern crate lazy_static;

mod constants;
mod gamepad;
mod keyboard;
mod retro_ab_controller;
mod state_thread;

pub mod devices_manager;
pub use retro_ab_controller::{
    input_poll_callback, input_state_callback, rumble_callback, RetroAbController,
};
