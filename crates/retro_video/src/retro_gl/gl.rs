#![deny(clippy::all)]
#[allow(clippy::all)]
#[allow(warnings)]
pub mod gl {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}
