use std::error::Error;
use std::io;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, I24, SampleFormat, SizedSample, U24};

use crate::synthesis::sine_generator::SineGenerator;

pub type AudioResult<T> = Result<T, Box<dyn Error + Send + Sync + 'static>>;

const DEFAULT_FREQUENCY_HZ: f32 = 440.0;
const DEFAULT_AMPLITUDE: f32 = 0.2;

pub struct StreamPlayer {
    _stream: cpal::Stream,
    device_name: String,
    sample_rate_hz: u32,
    channels: u16,
}

impl StreamPlayer {
    pub fn play_default_sine() -> AudioResult<Self> {
        Self::play(SineGeneratorSettings {
            frequency_hz: DEFAULT_FREQUENCY_HZ,
            amplitude: DEFAULT_AMPLITUDE,
        })
    }

    pub fn play(settings: SineGeneratorSettings) -> AudioResult<Self> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "no default output device"))?;
        let device_name = device
            .description()
            .map(|description| description.to_string())
            .unwrap_or_else(|_| "unknown device".to_string());
        let supported_config = device.default_output_config()?;
        let sample_format = supported_config.sample_format();
        let stream_config = supported_config.config();
        let sample_rate_hz = stream_config.sample_rate;
        let channels = stream_config.channels;
        let generator = SineGenerator::new(
            settings.frequency_hz,
            settings.amplitude,
            sample_rate_hz as f32,
        );

        if !is_supported_output_sample_format(sample_format) {
            return Err(io::Error::other(format!(
                "unsupported output sample format: {sample_format}"
            ))
            .into());
        }

        let stream = match sample_format {
            SampleFormat::I8 => build_output_stream::<i8>(&device, &stream_config, generator),
            SampleFormat::I16 => build_output_stream::<i16>(&device, &stream_config, generator),
            SampleFormat::I24 => build_output_stream::<I24>(&device, &stream_config, generator),
            SampleFormat::I32 => build_output_stream::<i32>(&device, &stream_config, generator),
            SampleFormat::I64 => build_output_stream::<i64>(&device, &stream_config, generator),
            SampleFormat::U8 => build_output_stream::<u8>(&device, &stream_config, generator),
            SampleFormat::U16 => build_output_stream::<u16>(&device, &stream_config, generator),
            SampleFormat::U24 => build_output_stream::<U24>(&device, &stream_config, generator),
            SampleFormat::U32 => build_output_stream::<u32>(&device, &stream_config, generator),
            SampleFormat::U64 => build_output_stream::<u64>(&device, &stream_config, generator),
            SampleFormat::F32 => build_output_stream::<f32>(&device, &stream_config, generator),
            SampleFormat::F64 => build_output_stream::<f64>(&device, &stream_config, generator),
            _ => unreachable!("unsupported sample formats are rejected before stream creation"),
        };
        let stream = stream?;

        stream.play()?;

        Ok(Self {
            _stream: stream,
            device_name,
            sample_rate_hz,
            channels,
        })
    }

    pub fn device_name(&self) -> &str {
        &self.device_name
    }

    pub fn sample_rate_hz(&self) -> u32 {
        self.sample_rate_hz
    }

    pub fn channels(&self) -> u16 {
        self.channels
    }
}

fn is_supported_output_sample_format(sample_format: SampleFormat) -> bool {
    matches!(
        sample_format,
        SampleFormat::I8
            | SampleFormat::I16
            | SampleFormat::I24
            | SampleFormat::I32
            | SampleFormat::I64
            | SampleFormat::U8
            | SampleFormat::U16
            | SampleFormat::U24
            | SampleFormat::U32
            | SampleFormat::U64
            | SampleFormat::F32
            | SampleFormat::F64
    )
}

#[derive(Debug, Clone, Copy)]
pub struct SineGeneratorSettings {
    pub frequency_hz: f32,
    pub amplitude: f32,
}

fn build_output_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    mut generator: SineGenerator,
) -> Result<cpal::Stream, cpal::BuildStreamError>
where
    T: SizedSample + FromSample<f32>,
{
    let channels = usize::from(config.channels);

    device.build_output_stream(
        config,
        move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_sine_to_stream(output, channels, &mut generator);
        },
        move |error| {
            eprintln!("audio stream error: {error}");
        },
        None,
    )
}

fn write_sine_to_stream<T>(output: &mut [T], channels: usize, generator: &mut SineGenerator)
where
    T: SizedSample + FromSample<f32>,
{
    write_mono_to_channels(output, channels, || generator.next_sample());
}

pub fn write_mono_to_channels<T, F>(output: &mut [T], channels: usize, mut next_mono_sample: F)
where
    T: SizedSample + FromSample<f32>,
    F: FnMut() -> f32,
{
    if channels == 0 {
        output.fill(T::from_sample(0.0));
        return;
    }

    for frame in output.chunks_mut(channels) {
        let mono_sample = T::from_sample(next_mono_sample());

        for sample in frame {
            *sample = mono_sample;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_mono_samples_to_each_output_channel() {
        let mono_samples = [0.1, -0.2];
        let mut mono_samples = mono_samples.into_iter();
        let mut output = [0.0_f32; 4];

        write_mono_to_channels(&mut output, 2, || mono_samples.next().unwrap_or(0.0));

        assert_eq!(output, [0.1, 0.1, -0.2, -0.2]);
    }

    #[test]
    fn treats_zero_channels_as_silence() {
        let mono_samples = [0.1, -0.2];
        let mut mono_samples = mono_samples.into_iter();
        let mut output = [1.0_f32; 4];

        write_mono_to_channels(&mut output, 0, || mono_samples.next().unwrap_or(0.0));

        assert_eq!(output, [0.0; 4]);
    }

    #[test]
    fn supports_all_non_dsd_pcm_sample_formats() {
        let supported_formats = [
            SampleFormat::I8,
            SampleFormat::I16,
            SampleFormat::I24,
            SampleFormat::I32,
            SampleFormat::I64,
            SampleFormat::U8,
            SampleFormat::U16,
            SampleFormat::U24,
            SampleFormat::U32,
            SampleFormat::U64,
            SampleFormat::F32,
            SampleFormat::F64,
        ];

        for sample_format in supported_formats {
            assert!(
                is_supported_output_sample_format(sample_format),
                "{sample_format} should be supported"
            );
        }
    }

    #[test]
    fn rejects_dsd_sample_formats() {
        let unsupported_formats = [
            SampleFormat::DsdU8,
            SampleFormat::DsdU16,
            SampleFormat::DsdU32,
        ];

        for sample_format in unsupported_formats {
            assert!(
                !is_supported_output_sample_format(sample_format),
                "{sample_format} should be unsupported"
            );
        }
    }
}
