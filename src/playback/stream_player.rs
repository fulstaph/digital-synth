use std::error::Error;
use std::io;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, SampleFormat, SizedSample};

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

        let stream = match sample_format {
            SampleFormat::F32 => build_output_stream::<f32>(&device, &stream_config, generator)?,
            SampleFormat::F64 => build_output_stream::<f64>(&device, &stream_config, generator)?,
            SampleFormat::I16 => build_output_stream::<i16>(&device, &stream_config, generator)?,
            SampleFormat::U16 => build_output_stream::<u16>(&device, &stream_config, generator)?,
            unsupported => {
                return Err(io::Error::other(format!(
                    "unsupported output sample format: {unsupported}"
                ))
                .into());
            }
        };

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
    if channels == 0 {
        let silence = T::from_sample(0.0);
        output.fill(silence);
        return;
    }

    for frame in output.chunks_mut(channels) {
        let mut sample = [0.0];
        generator.fill_mono(&mut sample);
        let converted_sample = T::from_sample(sample[0]);

        for channel_sample in frame {
            *channel_sample = converted_sample;
        }
    }
}

pub fn write_mono_to_channels<I>(output: &mut [f32], channels: usize, mono_samples: I)
where
    I: IntoIterator<Item = f32>,
{
    if channels == 0 {
        output.fill(0.0);
        return;
    }

    for (frame, mono_sample) in output.chunks_mut(channels).zip(mono_samples) {
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
        let mut output = [0.0; 4];

        write_mono_to_channels(&mut output, 2, mono_samples);

        assert_eq!(output, [0.1, 0.1, -0.2, -0.2]);
    }

    #[test]
    fn treats_zero_channels_as_silence() {
        let mono_samples = [0.1, -0.2];
        let mut output = [1.0; 4];

        write_mono_to_channels(&mut output, 0, mono_samples);

        assert_eq!(output, [0.0; 4]);
    }
}
