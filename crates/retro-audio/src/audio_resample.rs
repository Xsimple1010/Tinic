use crate::audios::{AudioMetadata, BufferCons, BufferProd};
use ringbuf::{
    SharedRb,
    storage::Heap,
    traits::{Consumer, Observer, Producer, Split},
};
use rubato::audioadapter_buffers::direct::InterleavedSlice;
use rubato::{Adjustable, Async, FixedAsync, Resampler};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::thread::{self};
use tinic_generics::{
    error_handle::TinicResult,
    types::{ArcTMutex, TMutex},
};

#[derive(Clone)]
pub struct AudioResample {
    back_buffer_prod: ArcTMutex<Option<BufferProd>>,
    in_metadata: ArcTMutex<Option<AudioMetadata>>,
    // saída (lida pelo CPAL)
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
            in_metadata: TMutex::new(None),
            can_run_thread: Arc::new(AtomicBool::new(false)),
        }
    }
    pub fn init(
        &self,
        in_metadata: AudioMetadata,
        front_buffer_prod: BufferProd,
        front_metadata: AudioMetadata,
    ) {
        let out_rb = SharedRb::<Heap<f32>>::new(600000);
        let (back_buffer_prod, back_buffer_cons) = out_rb.split();

        self.back_buffer_prod.store(Some(back_buffer_prod));
        self.in_metadata.store(Some(in_metadata));

        self.resample_process_thread(back_buffer_cons, front_buffer_prod, front_metadata)
    }

    pub fn stop(&self) {
        self.can_run_thread.store(false, Ordering::SeqCst);

        self.back_buffer_prod.store(None);
        self.in_metadata.store(None);
    }

    fn set_up_resampler(out_channels: u16) -> Async<f32> {
        use rubato::PolynomialDegree;

        Async::<f32>::new_poly(
            48000.0 / 44100.0,
            4.0,
            PolynomialDegree::Cubic, // ou Linear / Quadratic
            512,                     // chunk menor = menos latência
            out_channels as usize,
            FixedAsync::Input,
        )
        .unwrap()
    }

    pub fn add_sample(&self, data: &[f32], metadata: AudioMetadata) -> TinicResult<()> {
        let mut res = self.back_buffer_prod.load_or_spawn_err(
            "Não foi possível adicionar amostras de audio ao buffer de entrada",
        )?;

        if let Some(back_buffer_prod) = &mut *res {
            back_buffer_prod.push_slice(data);
            self.in_metadata.store(Some(metadata));
        }

        Ok(())
    }

    fn resample_process_thread(
        &self,
        mut back_buffer_cons: BufferCons,
        mut front_buffer_prod: BufferProd,
        front_metadata: AudioMetadata,
    ) {
        let back_metadata = self.in_metadata.clone();
        let can_run_thread = self.can_run_thread.clone();
        can_run_thread.store(true, Ordering::SeqCst);

        thread::spawn(move || {
            let mut resampler = Self::set_up_resampler(front_metadata.channels);
            let mut input_buf = Vec::<f32>::with_capacity(4096);
            let mut output_buf = Vec::<f32>::with_capacity(4096);

            let back_metadata = {
                match back_metadata.try_load() {
                    Ok(metadata) => match metadata.clone() {
                        Some(metadata) => metadata,
                        None => return,
                    },
                    _ => return,
                }
            };
            
            let ratio = front_metadata.sample_rate as f64 / back_metadata.sample_rate as f64;
            resampler.set_resample_ratio(ratio, false).unwrap();

            while can_run_thread.load(Ordering::SeqCst) {
                let back_metadata = AudioMetadata {
                    channels: 2,
                    sample_rate: 44100,
                };

                if back_metadata.sample_rate == front_metadata.sample_rate {
                    let mut temps: Vec<f32> = vec![0f32; back_buffer_cons.occupied_len()];
                    back_buffer_cons.pop_slice(&mut temps);
                    front_buffer_prod.push_slice(&temps);
                } else {
                    AudioResample::make_resample(
                        &mut resampler,
                        &mut back_buffer_cons,
                        &back_metadata,
                        &mut front_buffer_prod,
                        &front_metadata,
                        &mut input_buf,
                        &mut output_buf,
                    );
                }
            }
        });
    }

    fn make_resample(
        resampler: &mut Async<f32>,
        back_buffer_cons: &mut BufferCons,
        back_metadata: &AudioMetadata,
        front_buffer_prod: &mut BufferProd,
        front_metadata: &AudioMetadata,
        input_buf: &mut Vec<f32>,
        output_buf: &mut Vec<f32>,
    ) {
        if back_buffer_cons.is_empty() {
            return;
        }

        let frames_needed = resampler.input_frames_next();
        let channels_in = back_metadata.channels as usize;
        let samples_needed = frames_needed * channels_in;

        if back_buffer_cons.occupied_len() < samples_needed {
            return;
        }

        // Reusa o buffer
        input_buf.resize(samples_needed, 0.0);
        back_buffer_cons.pop_slice(input_buf);

        let input_adapter =
            InterleavedSlice::new(input_buf, channels_in, frames_needed).expect("input size");

        let max_out_frames = resampler.output_frames_next(); // melhor que max()
        let channels_out = front_metadata.channels as usize;
        output_buf.resize(max_out_frames * channels_out, 0.0);

        let mut output_adapter =
            InterleavedSlice::new_mut(output_buf, channels_out, max_out_frames)
                .expect("output size");

        let (_in, frames_out) = resampler
            .process_into_buffer(&input_adapter, &mut output_adapter, None)
            .unwrap();

        let samples_written = frames_out * channels_out;
        front_buffer_prod.push_slice(&output_buf[..samples_written]);
    }
}
