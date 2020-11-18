/// Take an audio media and decode it
use std::fs::File;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::core::units::Duration;
use synthesizer_io_core::module::N_SAMPLES_PER_CHUNK;

pub fn decode_source(file: File, extension: Option<&str>) -> Vec<[f32; N_SAMPLES_PER_CHUNK]> {
    let mut hint = Hint::new();
    if let Some(extension) = extension {
        hint.with_extension(extension);
    }

    let mss = MediaSourceStream::new(Box::new(file));

    let format_opts: FormatOptions = Default::default();
    let metadata_opts: MetadataOptions = Default::default();

    let mut track = vec![];
    match symphonia::default::get_probe().format(&hint, mss, &format_opts, &metadata_opts) {
        Ok(probed) => {
            // Get the default stream.
            // TODO: Allow stream selection.
            let mut reader = probed.format;
            let stream = reader.default_stream().unwrap();
            let options = DecoderOptions {
                ..Default::default()
            };

            // Create a decoder for the stream.
            let mut decoder = symphonia::default::get_codecs()
                .make(&stream.codec_params, &options)
                .unwrap();

            loop {
                match &reader.next_packet() {
                    Err(_) => break,
                    Ok(packet) => match decoder.decode(packet) {
                        Err(Error::DecodeError(_)) => {
                            // Decode errors are not fatal.
                            continue;
                        }
                        Err(_) => {
                            panic!();
                        }
                        Ok(decoded) => {
                            let mut buffer: [f32; N_SAMPLES_PER_CHUNK] = [0.0; N_SAMPLES_PER_CHUNK];
                            let mut i = 0;

                            let spec = *decoded.spec();
                            let duration = Duration::from(decoded.capacity() as u64);

                            let mut sample_buf =
                                symphonia::core::audio::SampleBuffer::<f32>::new(duration, spec);
                            sample_buf.copy_interleaved_ref(decoded);
                            let samples: Vec<f32> =
                                sample_buf.samples().into_iter().map(|x| *x).collect();

                            for sample in samples {
                                buffer[i] = sample;
                                if i == N_SAMPLES_PER_CHUNK - 1 {
                                    track.push(buffer);
                                    i = 0
                                } else {
                                    i = i + 1;
                                }
                            }
                        }
                    },
                }
            }
        }
        Err(e) => panic!(format!("{}", e)),
    }
    track
}
