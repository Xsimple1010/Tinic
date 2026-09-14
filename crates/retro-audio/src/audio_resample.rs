use crate::audios::{AudioMetadata, BufferCons, BufferProd};
use arc_swap::ArcSwapOption;
use ringbuf::{
    SharedRb,
    storage::Heap,
    traits::{Consumer, Observer, Producer, Split},
};
use rubato::{
    Adjustable, Async, FixedAsync, PolynomialDegree, Resampler,
    audioadapter_buffers::direct::SequentialSliceOfVecs,
};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::thread::{self, sleep};
use std::time::Duration;
use tinic_generics::types::{ArcTMutex, TMutex};

const CHANNELS: usize = 2;

#[derive(Clone)]
pub struct AudioResample {
    // BufferProd precisa de &mut para push_slice -> continua atrás de um
    // Mutex, que dá exclusividade de verdade. ArcSwap não serve aqui.
    back_buffer_prod: ArcTMutex<Option<BufferProd>>,

    // AudioMetadata é só lida (nunca mutada por dentro) — troca inteira,
    // consumida com frequência pela thread de resample. Handoff lock-free.
    in_metadata: Arc<ArcSwapOption<AudioMetadata>>,

    can_run_thread: Arc<AtomicBool>,
}

impl Drop for AudioResample {
    fn drop(&mut self) {
        self.stop();
    }
}

impl AudioResample {
    pub fn new() -> Self {
        Self {
            back_buffer_prod: TMutex::new(None),
            in_metadata: Arc::new(ArcSwapOption::from(None)),
            can_run_thread: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn init(
        &self,
        in_metadata: AudioMetadata,
        front_buffer_prod: BufferProd,
        front_metadata: AudioMetadata,
    ) {
        let out_rb = SharedRb::<Heap<i16>>::new(600000);
        let (back_buffer_prod, back_buffer_cons) = out_rb.split();

        self.back_buffer_prod.store(Some(back_buffer_prod));
        self.in_metadata.store(Some(Arc::new(in_metadata)));

        self.resample_process_thread(back_buffer_cons, front_buffer_prod, front_metadata)
    }

    pub fn stop(&self) {
        self.can_run_thread.store(false, Ordering::SeqCst);
        self.back_buffer_prod.store(None);
        self.in_metadata.store(None);
    }

    /// Chamado pelo produtor de amostras (thread do core), com frequência.
    pub fn add_sample(&self, data: &[i16], metadata: AudioMetadata) {
        // continua exigindo lock (curtíssimo: só um push_slice) porque
        // BufferProd precisa de acesso exclusivo pra empurrar dados.
        if let Ok(mut prod) = self.back_buffer_prod.try_load() {
            if let Some(prod) = &mut *prod {
                prod.push_slice(data);
            }
        }

        // troca lock-free: nunca compete com a leitura da thread de resample.
        self.in_metadata.store(Some(Arc::new(metadata)));
    }

    fn set_up_resampler(out_channels: u16) -> Async<f64> {
        Async::<f64>::new_poly(
            1.5,
            2.2,
            PolynomialDegree::Linear,
            2048,
            out_channels as usize,
            FixedAsync::Output,
        )
            .expect("Failed to create Async resampler")
    }

    fn resample_process_thread(
        &self,
        mut back_buffer_cons: BufferCons,
        mut front_buffer_prod: BufferProd,
        front_metadata: AudioMetadata,
    ) {
        let in_metadata = self.in_metadata.clone();
        let can_run_thread = self.can_run_thread.clone();
        can_run_thread.store(true, Ordering::SeqCst);

        thread::spawn(move || {
            let mut resampler = Self::set_up_resampler(front_metadata.channels);
            let mut scratch = ResampleScratch::new(&resampler);

            while can_run_thread.load(Ordering::SeqCst) {
                // load_full() é wait-free: nunca bloqueia, mesmo se
                // add_sample estiver chamando `store` nesse exato instante.
                let Some(back_metadata) = in_metadata.load_full() else {
                    sleep(Duration::from_millis(3));
                    continue;
                };

                if back_metadata.sample_rate == front_metadata.sample_rate {
                    let occupied = back_buffer_cons.occupied_len();
                    // resize só realoca se `occupied` superar o maior valor
                    // já visto — na prática vira um no-op depois do warm-up.
                    if scratch.passthrough.len() < occupied {
                        scratch.passthrough.resize(occupied, 0);
                    }
                    let buf = &mut scratch.passthrough[..occupied];
                    back_buffer_cons.pop_slice(buf);
                    front_buffer_prod.push_slice(buf);
                } else {
                    // Arc<AudioMetadata> -> &AudioMetadata: make_resample não
                    // precisa saber que a metadata veio de um Arc.
                    Self::make_resample(
                        &mut resampler,
                        &mut back_buffer_cons,
                        &back_metadata,
                        &mut front_buffer_prod,
                        &front_metadata,
                        &mut scratch,
                    );
                }

                // sleep(Duration::from_millis(3));
            }
        });
    }

    fn make_resample(
        resampler: &mut Async<f64>,
        back_buffer_cons: &mut BufferCons,
        back_metadata: &AudioMetadata,
        front_buffer_prod: &mut BufferProd,
        front_metadata: &AudioMetadata,
        scratch: &mut ResampleScratch,
    ) {
        if back_buffer_cons.is_empty() {
            return;
        }

        let ratio = front_metadata.sample_rate as f64 / back_metadata.sample_rate as f64;
        if ratio != scratch.last_ratio {
            resampler.set_resample_ratio(ratio, false).unwrap();
            scratch.last_ratio = ratio;
        }

        let frames_needed = resampler.input_frames_next();
        let samples_needed = frames_needed * back_metadata.channels as usize;

        if back_buffer_cons.occupied_len() < samples_needed {
            return;
        }

        debug_assert!(scratch.input_raw.len() >= samples_needed);
        let input_slice = &mut scratch.input_raw[..samples_needed];
        back_buffer_cons.pop_slice(input_slice);

        Self::samples_to_waves_into(input_slice, back_metadata.channels, &mut scratch.waves_in);

        let input_adapter = SequentialSliceOfVecs::new(&scratch.waves_in, CHANNELS, frames_needed)
            .expect("falha ao construir o adapter de entrada");

        let out_frames = scratch.waves_out[0].len();
        let mut output_adapter =
            SequentialSliceOfVecs::new_mut(&mut scratch.waves_out, CHANNELS, out_frames)
                .expect("falha ao construir o adapter de saída");

        resampler
            .process_into_buffer(&input_adapter, &mut output_adapter, None)
            .expect("falha no resample");

        Self::waves_to_front_buffer(&scratch.waves_out, &mut scratch.out_i16, front_buffer_prod);
    }

    fn samples_to_waves_into(samples: &[i16], channels: u16, waves_in: &mut [Vec<f64>]) {
        let frames = samples.len() / channels as usize;

        for i in 0..frames {
            let l = samples[i * channels as usize] as f64 / i16::MAX as f64;
            waves_in[0][i] = l;
            waves_in[1][i] = if channels == 1 {
                l
            } else {
                samples[i * channels as usize + 1] as f64 / i16::MAX as f64
            };
        }
    }

    fn waves_to_front_buffer(waves: &[Vec<f64>], out_i16: &mut [i16], front_buffer: &mut BufferProd) {
        let left = &waves[0];
        let right = &waves[1];
        let frames = left.len().min(right.len());
        debug_assert!(out_i16.len() >= frames * 2);

        for i in 0..frames {
            out_i16[i * 2] = (left[i].clamp(-1.0, 1.0) * i16::MAX as f64) as i16;
            out_i16[i * 2 + 1] = (right[i].clamp(-1.0, 1.0) * i16::MAX as f64) as i16;
        }

        front_buffer.push_slice(&out_i16[..frames * 2]);
    }
}

struct ResampleScratch {
    input_raw: Vec<i16>,
    waves_in: Vec<Vec<f64>>,
    waves_out: Vec<Vec<f64>>,
    out_i16: Vec<i16>,
    passthrough: Vec<i16>,
    last_ratio: f64,
}

impl ResampleScratch {
    fn new(resampler: &Async<f64>) -> Self {
        let max_input_frames = resampler.input_frames_max();
        let out_frames = resampler.output_frames_next();

        Self {
            input_raw: vec![0i16; max_input_frames * CHANNELS],
            waves_in: vec![vec![0.0; max_input_frames]; CHANNELS],
            waves_out: vec![vec![0.0; out_frames]; CHANNELS],
            out_i16: vec![0i16; out_frames * CHANNELS],
            passthrough: Vec::new(),
            last_ratio: resampler.resample_ratio(),
        }
    }
}