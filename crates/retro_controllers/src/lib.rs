extern crate generics;
extern crate gilrs;

mod gamepad;
mod retro_controller;
mod state_thread;

pub use gamepad::retro_gamepad::RetroGamePad;
pub mod devices_manager;

pub use retro_controller::{RetroController, RetroControllerCb};
