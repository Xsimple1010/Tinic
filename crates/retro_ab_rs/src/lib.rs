extern crate libloading;
extern crate uuid;

mod av_info;
mod binding;
mod constants;
mod controller_info;
mod environment;
mod managers;
mod retro_context;
mod retro_perf;
mod tools;

//arquivo principal!
pub mod retro_ab;

pub mod core;
pub mod erro_handle;
pub mod graphic_api;
pub mod paths;
pub mod system;
pub mod test_tools;

pub use managers::option_manager;

pub use managers::args_manager;

pub use binding::binding_libretro as retro_sys;
