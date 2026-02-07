use std::env;
use std::fs::File;
use std::path::PathBuf;

use cfg_aliases::cfg_aliases;
use gl_generator::{Api, Fallbacks, Profile, Registry, StructGenerator};

fn main() {
    // XXX this is taken from glutin/build.rs.

    // Setup alias to reduce `cfg` boilerplate.
    cfg_aliases! {
        // Systems.
        android_platform: { target_os = "android" },
        macos_platform: { target_os = "macos" },
        free_unix: { all(unix, not(apple), not(android_platform)) },

        // Native displays.
        x11_platform: { all(feature = "x11", free_unix) },
        wayland_platform: { all(feature = "wayland", free_unix) },

        // Backends.
        egl_backend: { all(feature = "egl", any(windows, unix), not(apple)) },
        glx_backend: { all(feature = "glx", x11_platform) },
        wgl_backend: { all(feature = "wgl", windows) },
    }

    let dest = PathBuf::from(&env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-changed=build.rs");

    let mut file = File::create(dest.join("gl_bindings.rs")).unwrap();
    Registry::new(Api::Gl, (3, 1), Profile::Core, Fallbacks::All, [])
        .write_bindings(StructGenerator, &mut file)
        .unwrap();
}
