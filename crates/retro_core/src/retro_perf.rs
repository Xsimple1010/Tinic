use crate::libretro_sys::binding_libretro::{
    RETRO_SIMD_AVX, RETRO_SIMD_AVX2, RETRO_SIMD_MMX, RETRO_SIMD_SSE, RETRO_SIMD_SSE2,
    RETRO_SIMD_SSE3, RETRO_SIMD_SSE4, RETRO_SIMD_SSE42, retro_perf_counter, retro_perf_tick_t,
    retro_time_t,
};
use raw_cpuid::CpuId;
use std::time::{Instant, SystemTime};

static mut LAST_COUNTER: Option<*mut retro_perf_counter> = None;

pub unsafe extern "C" fn core_get_perf_counter() -> retro_perf_tick_t {
    Instant::now().elapsed().as_nanos() as retro_perf_tick_t
}

pub unsafe extern "C" fn core_perf_register(counter_raw: *mut retro_perf_counter) {
    unsafe {
        let mut counter = *counter_raw;
        counter.registered = true;
        LAST_COUNTER = Some(counter_raw);
    }
}

pub unsafe extern "C" fn core_perf_start(counter_raw: *mut retro_perf_counter) {
    unsafe {
        let mut counter = *counter_raw;
        if counter.registered {
            counter.start = core_get_perf_counter();
        }
    }
}

pub unsafe extern "C" fn core_perf_stop(counter_raw: *mut retro_perf_counter) {
    unsafe {
        let mut counter = *counter_raw;
        counter.total = core_get_perf_counter() - counter.start;
    }
}

pub unsafe extern "C" fn core_perf_log() {
    unsafe {
        if let Some(counter_raw) = LAST_COUNTER {
            let counter = *counter_raw;
            println!("[timer] {:?}", counter);
        }
    }
}

pub unsafe extern "C" fn get_cpu_features() -> u64 {
    let mut cpu: u64 = 0;
    let cpuid = CpuId::new();

    if let Some(feature_info) = cpuid.get_feature_info() {
        if feature_info.has_avx() {
            cpu |= RETRO_SIMD_AVX as u64;
        }
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
    }

    if let Some(extended_info) = cpuid.get_extended_feature_info() {
        if extended_info.has_avx2() {
            cpu |= RETRO_SIMD_AVX2 as u64;
        }
    }

    cpu
}

pub unsafe extern "C" fn get_features_get_time_usec() -> retro_time_t {
    // Captura o tempo de início
    let start = SystemTime::now();

    // Calcula o tempo decorrido em milissegundos
    if let Ok(elapsed) = start.elapsed() {
        println!(
            "get_features_get_time_usec -> Tempo decorrido: {} milissegundos",
            elapsed.as_secs()
        );

        elapsed.as_secs() as retro_time_t
    } else {
        println!("Erro ao calcular o tempo decorrido");

        0
    }
}
