//! Low level interface

//use symphonia::core::audio::SignalSpec;
use symphonia::core::sample::Sample;
use synthesizer_io_core::module::N_SAMPLES_PER_CHUNK;
use synthesizer_io_core::worker::Worker;

use cpal::traits::DeviceTrait;
use cpal::traits::HostTrait;
use cpal::traits::StreamTrait;
use cpal::StreamConfig;
use time;

use rb::*;

pub const BYTES_PER_SAMPLE: u16 = 32;

pub struct AudioOutput {
    ring_buf_producer: rb::Producer<f32>,
    #[allow(dead_code)]
    stream: cpal::Stream,
}

impl AudioOutput {
    pub fn new(device: &cpal::Device, config: StreamConfig) -> Result<AudioOutput> {
        // Instantiate a ring buffer capable of buffering 8K (arbitrarily chosen) samples.
        let ring_buf = SpscRb::new(8 * 1024);
        let (ring_buf_producer, ring_buf_consumer) = (ring_buf.producer(), ring_buf.consumer());

        let stream_result = device.build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                // Write out as many samples as possible from the ring buffer to the audio
                // output.
                let written = ring_buf_consumer.read(data).unwrap_or(0);
                // Mute any remaining samples.
                data[written..].iter_mut().for_each(|s| *s = f32::MID);
            },
            move |err| todo!("audio output error: {}", err),
        );

        if let Err(_) = stream_result {
            todo!()
        }

        let stream = stream_result.unwrap();

        // Start the output stream.
        if let Err(_) = stream.play() {
            todo!()
        }

        Ok(AudioOutput {
            ring_buf_producer,
            stream,
        })
    }
    pub fn write_sample(&mut self, sample_buf: &[f32]) {
        // Audio samples must be interleaved for cpal. Interleave the samples in the audio
        // buffer into the sample buffer.

        let mut i = 0;

        // Write out all samples in the sample buffer to the ring buffer.
        while i < sample_buf.len() {
            // Write as many samples as possible to the ring buffer. This blocks until some
            // samples are written or the consumer has been destroyed (None is returned).
            if let Some(written) = self.ring_buf_producer.write_blocking(sample_buf) {
                i += written;
            } else {
                todo!()
            }
        }
    }
}

pub fn run_cpal(mut worker: Worker) {
    let mut audio_output = try_open().unwrap();
    loop {
        let timestamp = time::precise_time_ns();
        let next_buf = worker.work(timestamp)[0].get();
        let next_bufs = next_buf.chunks(2);
        for chunk in next_bufs {
            audio_output.write_sample(chunk);
        }
    }
}

pub fn try_open() -> Result<AudioOutput> {
    // Get default host.
    let host = cpal::default_host();

    // Get the default audio output device.
    let device = match host.default_output_device() {
        Some(device) => device,
        _ => todo!(),
    };

    let config = match device.default_output_config() {
        Ok(config) => config,
        Err(_) => todo!(),
    };

    // Output audio stream config. TODO
    let stream_config = cpal::StreamConfig {
        channels: 2 as cpal::ChannelCount,
        sample_rate: cpal::SampleRate(44100),
        buffer_size: cpal::BufferSize::Fixed(BYTES_PER_SAMPLE as u32 * N_SAMPLES_PER_CHUNK as u32),
    };

    // Select proper playback routine based on sample format.
    match config.sample_format() {
        cpal::SampleFormat::F32 => AudioOutput::new(&device, stream_config),
        _ => todo!(),
    }
}
