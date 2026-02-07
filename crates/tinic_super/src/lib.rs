extern crate reqwest;
extern crate zip;

pub mod art;
pub mod cores;
pub mod event;
pub mod infos;
pub mod rdb_manager;
pub mod tinic_super;
mod tools;

pub use generics::{error_handle::ErrorHandle, retro_paths::RetroPaths};
pub use tools::{
    download::DownloadProgress, extract_files::ExtractProgress, game_identifier::GameIdentifier,
};
