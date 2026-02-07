mod env_directory;
mod env_gamepads_io;
mod env_option;
mod env_video;
mod environment;

// // New safe context modules
// pub mod compatible_context;
// pub mod environment_safe;
// pub mod migration_example;
// pub mod safe_context;

pub use env_gamepads_io::{input_poll_callback, input_state_callback};
pub use env_video::{audio_sample_batch_callback, audio_sample_callback, video_refresh_callback};
pub use environment::*;

// // Export new safe interfaces
// pub use compatible_context::{legacy, CompatibleContext};
// pub use environment_safe::{
//     audio_sample_batch_callback as safe_audio_sample_batch_callback,
//     audio_sample_callback as safe_audio_sample_callback, configure as safe_configure,
//     core_environment as safe_core_environment, delete_local_core_ctx as safe_delete_context,
//     input_poll_callback as safe_input_poll_callback,
//     input_state_callback as safe_input_state_callback, migration,
//     video_refresh_callback as safe_video_refresh_callback,
// };
// pub use safe_context::SafeCoreContext;

// // Re-export migration utilities
// pub use migration_example::{
//     examples as migration_examples, testing as migration_testing, troubleshooting, MigrationGuide,
// };
