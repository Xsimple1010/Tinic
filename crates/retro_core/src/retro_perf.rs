use crate::libretro_sys::binding_libretro::{
    retro_perf_counter, retro_perf_tick_t, retro_time_t, RETRO_SIMD_AVX, RETRO_SIMD_AVX2,
    RETRO_SIMD_MMX, RETRO_SIMD_SSE, RETRO_SIMD_SSE2, RETRO_SIMD_SSE3, RETRO_SIMD_SSE4,
    RETRO_SIMD_SSE42,
};

use raw_cpuid::CpuId;
use std::ptr;
use std::sync::{
    atomic::{AtomicPtr, Ordering},
    OnceLock,
};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

//
// ─────────────────────────────────────────────────────────
// GLOBALS (THREAD-SAFE)
// ─────────────────────────────────────────────────────────
//

static PERF_EPOCH: OnceLock<Instant> = OnceLock::new();
static LAST_COUNTER: AtomicPtr<retro_perf_counter> = AtomicPtr::new(ptr::null_mut());

fn perf_epoch() -> &'static Instant {
    PERF_EPOCH.get_or_init(Instant::now)
}

//
// ─────────────────────────────────────────────────────────
// PERF COUNTERS
// ─────────────────────────────────────────────────────────
//

pub extern "C" fn core_get_perf_counter() -> retro_perf_tick_t {
    perf_epoch().elapsed().as_nanos() as retro_perf_tick_t
}

pub unsafe extern "C" fn core_perf_register(counter_raw: *mut retro_perf_counter) {
    if counter_raw.is_null() {
        return;
    }

    let counter = unsafe { &mut *counter_raw };
    counter.registered = true;
    counter.total = 0;
    counter.start = 0;

    LAST_COUNTER.store(counter_raw, Ordering::Release);
}

pub unsafe extern "C" fn core_perf_start(counter_raw: *mut retro_perf_counter) {
    unsafe {
        if let Some(counter) = counter_raw.as_mut()
            && counter.registered
        {
            counter.start = core_get_perf_counter();
        }
    }
}

pub unsafe extern "C" fn core_perf_stop(counter_raw: *mut retro_perf_counter) {
    unsafe {
        if let Some(counter) = counter_raw.as_mut() {
            let end = core_get_perf_counter();
            counter.total = counter
                .total
                .saturating_add(end.saturating_sub(counter.start));
        }
    }
}

pub extern "C" fn core_perf_log() {
    let counter_ptr = LAST_COUNTER.load(Ordering::Acquire);

    if let Some(counter) = unsafe { counter_ptr.as_ref() } {
        println!(
            "[perf] ident={:?} total={} ticks",
            counter.ident, counter.total
        );
    }
}

//
// ─────────────────────────────────────────────────────────
// CPU FEATURES
// ─────────────────────────────────────────────────────────
//

pub extern "C" fn get_cpu_features() -> u64 {
    let mut cpu: u64 = 0;
    let cpuid = CpuId::new();

    if let Some(feature_info) = cpuid.get_feature_info() {
        if feature_info.has_mmx() {
            cpu |= RETRO_SIMD_MMX as u64;
        }
        if feature_info.has_sse() {
            cpu |= RETRO_SIMD_SSE as u64;
        }
        if feature_info.has_sse2() {
            cpu |= RETRO_SIMD_SSE2 as u64;
        }
        if feature_info.has_sse3() {
            cpu |= RETRO_SIMD_SSE3 as u64;
        }
        if feature_info.has_sse41() {
            cpu |= RETRO_SIMD_SSE4 as u64;
        }
        if feature_info.has_sse42() {
            cpu |= RETRO_SIMD_SSE42 as u64;
        }
        if feature_info.has_avx() {
            cpu |= RETRO_SIMD_AVX as u64;
        }
    }

    if let Some(extended_info) = cpuid.get_extended_feature_info()
        && extended_info.has_avx2()
    {
        cpu |= RETRO_SIMD_AVX2 as u64;
    }

    cpu
}

//
// ─────────────────────────────────────────────────────────
// TIME (MICROSECONDS, THREAD-SAFE)
// ─────────────────────────────────────────────────────────
//

pub extern "C" fn get_features_get_time_usec() -> retro_time_t {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_micros() as retro_time_t
}
