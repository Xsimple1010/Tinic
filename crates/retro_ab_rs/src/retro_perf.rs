use sdl2::sys::{
    SDL_GetPerformanceCounter, SDL_GetTicks, SDL_HasAVX, SDL_HasAVX2, SDL_HasMMX, SDL_HasSSE,
    SDL_HasSSE2, SDL_HasSSE3, SDL_HasSSE41, SDL_HasSSE42, SDL_bool,
};

use crate::retro_sys::{
    retro_perf_counter, retro_perf_tick_t, retro_time_t, RETRO_SIMD_AVX, RETRO_SIMD_AVX2,
    RETRO_SIMD_MMX, RETRO_SIMD_SSE, RETRO_SIMD_SSE2, RETRO_SIMD_SSE3, RETRO_SIMD_SSE4,
    RETRO_SIMD_SSE42,
};

static mut LAST_COUNTER: Option<*mut retro_perf_counter> = None;

pub unsafe extern "C" fn core_get_perf_counter() -> retro_perf_tick_t {
    SDL_GetPerformanceCounter() as retro_perf_tick_t
}

pub unsafe extern "C" fn core_perf_register(counter_raw: *mut retro_perf_counter) {
    let mut counter = *counter_raw;
    counter.registered = true;
    LAST_COUNTER = Some(counter_raw);
}

pub unsafe extern "C" fn core_perf_start(counter_raw: *mut retro_perf_counter) {
    let mut counter = *counter_raw;
    if counter.registered {
        counter.start = core_get_perf_counter();
    }
}

pub unsafe extern "C" fn core_perf_stop(counter_raw: *mut retro_perf_counter) {
    let mut counter = *counter_raw;
    counter.total = core_get_perf_counter() - counter.start;
}

pub unsafe extern "C" fn core_perf_log() {
    if let Some(counter_raw) = LAST_COUNTER {
        let counter = *counter_raw;
        println!("[timer] {:?}", counter);
    }
}

pub unsafe extern "C" fn get_cpu_features() -> u64 {
    let mut cpu: u64 = 0;

    if SDL_bool::SDL_TRUE == SDL_HasAVX() {
        cpu |= RETRO_SIMD_AVX as u64;
    }
    if SDL_bool::SDL_TRUE == SDL_HasAVX2() {
        cpu |= RETRO_SIMD_AVX2 as u64;
    }
    if SDL_bool::SDL_TRUE == SDL_HasMMX() {
        cpu |= RETRO_SIMD_MMX as u64;
    }
    if SDL_bool::SDL_TRUE == SDL_HasSSE() {
        cpu |= RETRO_SIMD_SSE as u64;
    }
    if SDL_bool::SDL_TRUE == SDL_HasSSE2() {
        cpu |= RETRO_SIMD_SSE2 as u64;
    }
    if SDL_bool::SDL_TRUE == SDL_HasSSE3() {
        cpu |= RETRO_SIMD_SSE3 as u64;
    }
    if SDL_bool::SDL_TRUE == SDL_HasSSE41() {
        cpu |= RETRO_SIMD_SSE4 as u64;
    }
    if SDL_bool::SDL_TRUE == SDL_HasSSE42() {
        cpu |= RETRO_SIMD_SSE42 as u64;
    }
    cpu
}

pub unsafe extern "C" fn get_features_get_time_usec() -> retro_time_t {
    (SDL_GetTicks() * 1000) as retro_time_t
}
