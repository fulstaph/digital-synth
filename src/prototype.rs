use std::time::Duration;

use crate::playback::stream_player::{AudioResult, SineGeneratorSettings, StreamPlayer};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PrototypePlaybackConfig {
    pub duration: Option<Duration>,
    pub frequency_hz: f32,
    pub amplitude: f32,
}

impl PrototypePlaybackConfig {
    pub fn stream_settings(self) -> SineGeneratorSettings {
        SineGeneratorSettings {
            frequency_hz: self.frequency_hz,
            amplitude: self.amplitude,
        }
    }

    pub fn play(self) -> AudioResult<StreamPlayer> {
        StreamPlayer::play(self.stream_settings())
    }
}

impl Default for PrototypePlaybackConfig {
    fn default() -> Self {
        let settings = SineGeneratorSettings::default();

        Self {
            duration: None,
            frequency_hz: settings.frequency_hz,
            amplitude: settings.amplitude,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_matches_current_temporary_sine_tone() {
        let config = PrototypePlaybackConfig::default();

        assert_eq!(
            config,
            PrototypePlaybackConfig {
                duration: None,
                frequency_hz: 440.0,
                amplitude: 0.2,
            }
        );
    }

    #[test]
    fn converts_to_stream_player_settings() {
        let config = PrototypePlaybackConfig {
            duration: Some(Duration::from_secs(2)),
            frequency_hz: 220.0,
            amplitude: 0.1,
        };

        assert_eq!(
            config.stream_settings(),
            SineGeneratorSettings {
                frequency_hz: 220.0,
                amplitude: 0.1,
            }
        );
    }
}
