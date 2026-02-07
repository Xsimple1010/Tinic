extern crate reqwest;
extern crate zip;

mod download;
mod extract_files;

pub mod core_info;
pub use download::FileProgress;
