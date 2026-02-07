use std::sync::atomic::Ordering;

use crate::common::setup::get_core_test;
use generics::{
    error_handle::ErrorHandle,
    test_workdir::{get_rom_test_path, remove_test_work_dir_path},
};
use libretro_sys::binding_libretro::{retro_hw_context_type, retro_pixel_format};

mod common;

#[test]
fn test_core_initial_state_and_after_load() -> Result<(), ErrorHandle> {
    let dir = "retro_core.test_core_initial_state_and_after_load";
    let core = get_core_test(dir)?;

    // =========================
    // 🔹 ANTES DO LOAD_GAME
    // =========================
    assert!(!core.game_loaded.load(Ordering::SeqCst));

    // ---------- SYSTEM INFO ----------
    let system = core.system.clone();
    assert_eq!(system.ports.read().unwrap().len(), 28);
    assert_eq!(system.subsystem.read().unwrap().len(), 0);
    assert_eq!(system.performance_level.load(Ordering::SeqCst), 0);

    let info = system.info.clone();
    assert_eq!(info.library_name.to_string(), "Mesen");
    assert_eq!(info.library_version.to_string(), "0.9.9");
    assert_eq!(info.valid_extensions.to_string(), "nes|fds|unf|unif");
    assert!(*info.need_full_path);
    assert!(!*info.block_extract);

    // ---------- VIDEO & TIMING ----------

    let video = &core.av_info.video;

    // ---------- GEOMETRY ----------
    assert_eq!(video.geometry.base_width.load(Ordering::SeqCst), 0);
    assert_eq!(video.geometry.base_height.load(Ordering::SeqCst), 0);
    assert_eq!(video.geometry.max_width.load(Ordering::SeqCst), 0);
    assert_eq!(video.geometry.max_height.load(Ordering::SeqCst), 0);

    // ---------- GRAPH_API ----------
    assert_eq!(
        video.graphic_api.bottom_left_origin.load(Ordering::SeqCst),
        false
    );

    // ---------- TIMING ----------
    let timing = &core.av_info.timing;
    assert_eq!(*timing.fps.read().unwrap(), 0.0);
    assert_eq!(*timing.sample_rate.read().unwrap(), 0);

    // =========================
    // 🔹  DEPOIS DO LOAD_GAME
    // =========================

    core.load_game(&get_rom_test_path().display().to_string())?;
    assert!(core.game_loaded.load(Ordering::SeqCst));

    // ---------- GEOMETRY ----------
    let geo = &core.av_info.video.geometry;

    assert_eq!(geo.base_width.load(Ordering::SeqCst), 256);
    assert_eq!(geo.base_height.load(Ordering::SeqCst), 240);
    assert_eq!(geo.max_width.load(Ordering::SeqCst), 602);
    assert_eq!(geo.max_height.load(Ordering::SeqCst), 240);

    let aspect = *geo.aspect_ratio.read().unwrap();
    assert!((aspect - 1.0666667).abs() < 0.0001);

    // ---------- TIMING ----------
    let timing = &core.av_info.timing;

    let fps = *timing.fps.read().unwrap();
    assert!((fps - 60.0998265).abs() < 0.01);

    assert_eq!(*timing.sample_rate.read().unwrap(), 48_000);

    // ---------- PIXEL FORMAT ----------
    let pixel_format = core
        .av_info
        .video
        .pixel_format
        .load_or(retro_pixel_format::RETRO_PIXEL_FORMAT_UNKNOWN)
        .clone();

    assert_eq!(
        pixel_format,
        retro_pixel_format::RETRO_PIXEL_FORMAT_XRGB8888
    );

    // ---------- GRAPHIC API (software rendering esperado) ----------
    let gfx = &core.av_info.video.graphic_api;

    assert_eq!(
        gfx.context_type,
        retro_hw_context_type::RETRO_HW_CONTEXT_OPENGL
    );
    assert!(gfx.fbo.read().unwrap().is_none());
    assert!(!gfx.depth.load(Ordering::SeqCst));
    assert!(!gfx.stencil.load(Ordering::SeqCst));
    assert!(!gfx.bottom_left_origin.load(Ordering::SeqCst));
    assert_eq!(gfx.major.load(Ordering::SeqCst), 0);
    assert_eq!(gfx.minor.load(Ordering::SeqCst), 0);
    assert!(!gfx.cache_context.load(Ordering::SeqCst));
    assert!(!gfx.debug_context.load(Ordering::SeqCst));

    remove_test_work_dir_path(dir)?;
    Ok(())
}
