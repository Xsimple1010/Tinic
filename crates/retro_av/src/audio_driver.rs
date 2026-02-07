use crate::{
    audio_resample::AudioResample,
    audios::{AudioMetadata, BufferCons, BufferProd},
};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, Stream,
};
use generics::{
    error_handle::ErrorHandle,
    types::{ArcTMutex, TMutex},
};
use retro_core::av_info::AvInfo;
use ringbuf::{
    storage::Heap,
    traits::{Consumer, Observer, Producer, Split},
    SharedRb,
};
use std::{result::Result, sync::Arc, time::Duration};

#[derive(Clone)]
pub struct AudioDriver {
    stream: ArcTMutex<Option<Stream>>,
    pub resampler: AudioResample,
    // so existe se não for necessario fazer o resample!
    front_prod_buffer: ArcTMutex<Option<BufferProd>>,
}

impl AudioDriver {
    pub fn new() -> Result<Self, ErrorHandle> {
        Ok(Self {
            stream: TMutex::new(None),
            resampler: AudioResample::new(),
            front_prod_buffer: TMutex::new(None),
        })
    }

    pub fn init(&self, av: &Arc<AvInfo>) -> Result<(), ErrorHandle> {
        let (device, front_sample_rate, front_channels) = AudioDriver::get_device_configs()?;
        let back_sample_rate =
            *av.timing.sample_rate.read().map_err(|e| {
                ErrorHandle::new(&format!("erro ao ler o sample rate do core: {e}"))
            })?;

        let front_rb = SharedRb::<Heap<i16>>::new(600000);
        let (front_prod_buffer, front_cons) = front_rb.split();

        // verifica se é necessario fazer resample do audio
        if front_sample_rate != back_sample_rate {
            let back_metadata = AudioMetadata {
                channels: 2, // Será modificada pelo core nas callbacks
                sample_rate: back_sample_rate,
            };

            let front_metadata = AudioMetadata {
                channels: front_channels,
                sample_rate: front_sample_rate,
            };

            self.resampler
                .init(back_metadata, front_prod_buffer, front_metadata)
        } else {
            self.front_prod_buffer.store(Some(front_prod_buffer));
        }

        self.set_up_stream(device, front_cons)
    }

    pub fn play(&self) -> Result<(), ErrorHandle> {
        match &mut *self
            .stream
            .load_or_spaw_err("Não foi possivel pausar o audio")?
        {
            Some(ref mut stream) => stream.play().map_err(|e| ErrorHandle::new(&e.to_string())),
            None => {
                return Err(ErrorHandle::new("Stream not initialized"));
            }
        }
    }

    pub fn pause(&self) -> Result<(), ErrorHandle> {
        match &mut *self.stream.load_or(None) {
            Some(ref mut stream) => stream.pause().map_err(|e| ErrorHandle::new(&e.to_string())),
            None => {
                return Err(ErrorHandle::new("Stream not initialized"));
            }
        }
    }

    pub fn stop(&self) {
        self.stream.store(None);
        self.resampler.stop();
        self.front_prod_buffer.store(None);
    }

    pub fn add_sample(&self, samples: &[i16], metadata: AudioMetadata) -> Result<(), ErrorHandle> {
        if let Some(front_buffer_prod) = &mut *self
            .front_prod_buffer
            .load_or_spaw_err("Front buffer not initialized")?
        {
            front_buffer_prod.push_slice(samples);
        } else {
            self.resampler.add_sample(samples, metadata)?;
        }

        Ok(())
    }

    fn set_up_stream(&self, device: Device, mut cons: BufferCons) -> Result<(), ErrorHandle> {
        let config = device.default_output_config().unwrap();

        let config = &config.into();
        let error_callback = |err| eprintln!("erro no stream {}", err);
        let timeout = Some(Duration::from_millis(2));
        let data_callback = move |front: &mut [i16], _: &cpal::OutputCallbackInfo| {
            if cons.is_empty() {
                front.fill(0);
                return;
            }

            let len = front.len().min(cons.occupied_len());
            let mut buffer = vec![0; len];
            cons.pop_slice(&mut buffer);

            for (i, sample) in buffer.into_iter().enumerate() {
                front[i] = sample;
            }

            if len < front.len() {
                front[len..].fill(0);
            }
        };

        let stream = device
            .build_output_stream(config, data_callback, error_callback, timeout)
            .map_err(|e| ErrorHandle::new(&e.to_string()))?;

        self.stream.store(Some(stream));

        Ok(())
    }

    fn get_device_configs() -> Result<(Device, u32, u16), ErrorHandle> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| ErrorHandle::new("No frontput device"))?;
        let config = device
            .default_output_config()
            .map_err(|e| ErrorHandle::new(&e.to_string()))?;
        let sample_rate = config.sample_rate().0;

        let channels = config.channels();

        Ok((device, sample_rate, channels))
    }
}
