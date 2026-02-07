use crate::audios::{AudioMetadata, BufferCons, BufferProd};
use generics::{
    error_handle::ErrorHandle,
    types::{ArcTMutex, TMutex},
};
use ringbuf::{
    storage::Heap,
    traits::{Consumer, Observer, Producer, Split},
    SharedRb,
};
use rubato::{
    Resampler, SincFixedIn, SincInterpolationParameters, SincInterpolationType, WindowFunction,
};
use std::thread::{self, JoinHandle};

#[derive(Clone)]
pub struct AudioResample {
    back_buffer_prod: ArcTMutex<Option<BufferProd>>,
    in_metadata: ArcTMutex<Option<AudioMetadata>>,
    // saída (lida pelo CPAL)
    thread_handler: ArcTMutex<Option<JoinHandle<()>>>,
}

impl AudioResample {
    pub fn new() -> Self {
        Self {
            back_buffer_prod: TMutex::new(None),
            in_metadata: TMutex::new(None),
            thread_handler: TMutex::new(None),
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
        self.in_metadata.store(Some(in_metadata));

        self.resample_process_thread(back_buffer_cons, front_buffer_prod, front_metadata)
    }

    pub fn stop(&self) {
        self.thread_handler.store(None);
        self.back_buffer_prod.store(None);
        self.in_metadata.store(None);
    }

    fn set_up_resampler(out_channels: u16) -> SincFixedIn<f64> {
        let params = SincInterpolationParameters {
            sinc_len: (520.0 * 0.10) as usize,
            f_cutoff: 0.95,
            interpolation: SincInterpolationType::Linear,
            oversampling_factor: 128,
            window: WindowFunction::BlackmanHarris2,
        };

        SincFixedIn::<f64>::new(1.5, 4.0, params, 520, out_channels as usize).unwrap()
    }

    pub fn add_sample(&self, data: &[i16], metadata: AudioMetadata) -> Result<(), ErrorHandle> {
        let mut res = self.back_buffer_prod.load_or_spaw_err(
            "Não foi possivel adicionar amostras de audio ao buffer de entrada",
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

        let join = thread::spawn(move || {
            let mut resampler = Self::set_up_resampler(front_metadata.channels);

            loop {
                let back_metadata = {
                    match back_metadata.try_load() {
                        Ok(metadata) => match metadata.clone() {
                            Some(metadata) => metadata,
                            None => continue,
                        },
                        _ => continue,
                    }
                };

                if back_metadata.sample_rate == front_metadata.sample_rate {
                    let mut temps: Vec<i16> = vec![0i16; back_buffer_cons.occupied_len()];
                    back_buffer_cons.pop_slice(&mut temps);
                    front_buffer_prod.push_slice(&temps);
                } else {
                    AudioResample::make_resample(
                        &mut resampler,
                        &mut back_buffer_cons,
                        &back_metadata,
                        &mut front_buffer_prod,
                        &front_metadata,
                    );
                }
            }
        });

        self.thread_handler.store(Some(join));
    }

    fn make_resample(
        resampler: &mut SincFixedIn<f64>,
        back_buffer_cons: &mut BufferCons,
        back_metadata: &AudioMetadata,
        front_buffer_prod: &mut BufferProd,
        front_metadata: &AudioMetadata,
    ) {
        // println!("back_buffer size: {}", back_buffer_cons.occupied_len());
        if back_buffer_cons.is_empty() {
            return;
        }

        let ratio = front_metadata.sample_rate as f64 / back_metadata.sample_rate as f64;
        resampler.set_resample_ratio(ratio, false).unwrap();

        let frames_needed = resampler.input_frames_next();
        let samples_needed = frames_needed * back_metadata.channels as usize;

        if back_buffer_cons.occupied_len() < samples_needed {
            return;
        }

        let mut input_raw = vec![0i16; samples_needed];
        back_buffer_cons.pop_slice(&mut input_raw);

        let waves_in = Self::samples_to_waves(&input_raw, back_metadata.channels);

        let waves_out = resampler.process(&waves_in, None).unwrap();

        Self::waves_to_frontbuffer(&waves_out, front_buffer_prod);
    }

    fn samples_to_waves(samples: &[i16], channels: u16) -> Vec<Vec<f64>> {
        let frames = samples.len() / channels as usize;

        let mut left = Vec::with_capacity(frames);
        let mut right = Vec::with_capacity(frames);

        for i in 0..frames {
            let l = samples[i * channels as usize] as f64 / i16::MAX as f64;

            if channels == 1 {
                left.push(l);
                right.push(l);
            } else {
                let r = samples[i * channels as usize + 1] as f64 / i16::MAX as f64;
                left.push(l);
                right.push(r);
            }
        }

        vec![left, right]
    }

    fn waves_to_frontbuffer(waves: &Vec<Vec<f64>>, front_buffer: &mut BufferProd) {
        let left = &waves[0];
        let right = &waves[1];

        let frames = left.len().min(right.len());

        let mut out: Vec<i16> = vec![];

        for i in 0..frames {
            let l = (left[i].clamp(-1.0, 1.0) * i16::MAX as f64) as i16;
            let r = (right[i].clamp(-1.0, 1.0) * i16::MAX as f64) as i16;

            // stereo interleaved
            out.push(l);
            out.push(r);
        }

        front_buffer.push_slice(&out);
    }
}
