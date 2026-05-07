use std::time::Duration;

use clap::Parser;

/// Play the current Digital Synth Rust prototype.
///
/// This CLI is intentionally small: it controls the temporary sine-wave source
/// used by the standalone CPAL stream-player spike. It is not the final user
/// interface for the instrument.
#[derive(Debug, Clone, Copy, Parser, PartialEq)]
#[command(version, about, long_about)]
pub struct CliConfig {
    /// How long to play before exiting, in seconds.
    ///
    /// Omit this option to keep playing until the process is interrupted.
    #[arg(long, value_name = "SECONDS", allow_hyphen_values = true, value_parser = parse_duration_seconds)]
    pub duration_seconds: Option<f64>,

    /// Oscillator frequency in hertz.
    #[arg(long, value_name = "HZ", default_value_t = 440.0, allow_hyphen_values = true, value_parser = parse_frequency_hz)]
    pub frequency_hz: f32,

    /// Output amplitude from 0.0 to 1.0.
    ///
    /// Values above 1.0 are rejected because they can clip on some devices.
    #[arg(long, value_name = "LEVEL", default_value_t = 0.2, allow_hyphen_values = true, value_parser = parse_amplitude)]
    pub amplitude: f32,
}

fn parse_duration_seconds(value: &str) -> Result<f64, String> {
    let duration_seconds = value
        .parse::<f64>()
        .map_err(|_| format!("invalid duration seconds: {value}"))?;

    validate_duration_seconds(duration_seconds)?;

    Ok(duration_seconds)
}

fn validate_duration_seconds(duration_seconds: f64) -> Result<(), String> {
    if !duration_seconds.is_finite() || duration_seconds <= 0.0 {
        return Err("duration seconds must be finite and greater than 0".to_string());
    }

    if Duration::try_from_secs_f64(duration_seconds).is_err() {
        return Err("duration seconds is too large".to_string());
    }

    Ok(())
}

fn parse_frequency_hz(value: &str) -> Result<f32, String> {
    let frequency_hz = value
        .parse::<f32>()
        .map_err(|_| format!("invalid frequency: {value}"))?;

    if !frequency_hz.is_finite() || frequency_hz <= 0.0 {
        return Err("frequency must be finite and greater than 0".to_string());
    }

    Ok(frequency_hz)
}

fn parse_amplitude(value: &str) -> Result<f32, String> {
    let amplitude = value
        .parse::<f32>()
        .map_err(|_| format!("invalid amplitude: {value}"))?;

    if !amplitude.is_finite() || !(0.0..=1.0).contains(&amplitude) {
        return Err("amplitude must be finite and between 0 and 1".to_string());
    }

    Ok(amplitude)
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    use clap::Parser;

    #[test]
    fn parses_duration_seconds_flag() {
        let args = ["digital-synth", "--duration-seconds", "2.5"];

        let config = CliConfig::try_parse_from(args).expect("duration flag should parse");

        assert_eq!(config.duration_seconds, Some(2.5));
    }

    #[test]
    fn uses_defaults_when_optional_flags_are_absent() {
        let args = ["digital-synth"];

        let config = CliConfig::try_parse_from(args).expect("empty args should parse");

        assert_eq!(config.duration_seconds, None);
        assert_eq!(config.frequency_hz, 440.0);
        assert_eq!(config.amplitude, 0.2);
    }

    #[test]
    fn parses_frequency_and_amplitude_flags() {
        let args = [
            "digital-synth",
            "--frequency-hz",
            "220",
            "--amplitude",
            "0.1",
        ];

        let config = CliConfig::try_parse_from(args).expect("tone flags should parse");

        assert_eq!(config.frequency_hz, 220.0);
        assert_eq!(config.amplitude, 0.1);
    }

    #[test]
    fn rejects_missing_duration_value() {
        let args = ["digital-synth", "--duration-seconds"];

        let error = CliConfig::try_parse_from(args).expect_err("missing duration should fail");

        assert!(error.to_string().contains("a value is required"));
    }

    #[test]
    fn rejects_zero_duration() {
        let args = ["digital-synth", "--duration-seconds", "0"];

        let error = CliConfig::try_parse_from(args).expect_err("zero duration should fail");

        assert!(
            error
                .to_string()
                .contains("duration seconds must be finite and greater than 0")
        );
    }

    #[test]
    fn rejects_negative_duration() {
        let args = ["digital-synth", "--duration-seconds", "-1"];

        let error = CliConfig::try_parse_from(args).expect_err("negative duration should fail");

        assert!(
            error
                .to_string()
                .contains("duration seconds must be finite and greater than 0")
        );
    }

    #[test]
    fn rejects_non_finite_duration() {
        let args = ["digital-synth", "--duration-seconds", "inf"];

        let error = CliConfig::try_parse_from(args).expect_err("infinite duration should fail");

        assert!(
            error
                .to_string()
                .contains("duration seconds must be finite and greater than 0")
        );
    }

    #[test]
    fn rejects_duration_larger_than_std_duration_can_represent() {
        let args = ["digital-synth", "--duration-seconds", "2e19"];

        let error = CliConfig::try_parse_from(args).expect_err("too-large duration should fail");

        assert!(error.to_string().contains("duration seconds is too large"));
    }

    #[test]
    fn rejects_non_positive_frequency() {
        let args = ["digital-synth", "--frequency-hz", "0"];

        let error = CliConfig::try_parse_from(args).expect_err("zero frequency should fail");

        assert!(
            error
                .to_string()
                .contains("frequency must be finite and greater than 0")
        );
    }

    #[test]
    fn rejects_out_of_range_amplitude() {
        let args = ["digital-synth", "--amplitude", "1.2"];

        let error = CliConfig::try_parse_from(args).expect_err("amplitude above 1 should fail");

        assert!(
            error
                .to_string()
                .contains("amplitude must be finite and between 0 and 1")
        );
    }

    #[test]
    fn rejects_unknown_arguments() {
        let args = ["digital-synth", "--frequency", "220"];

        let error = CliConfig::try_parse_from(args).expect_err("unknown argument should fail");

        assert!(error.to_string().contains("unexpected argument"));
    }

    #[test]
    fn generated_help_includes_usage_comments() {
        let help = CliConfig::command().render_long_help().to_string();

        assert!(help.contains("Play the current Digital Synth Rust prototype."));
        assert!(help.contains("How long to play before exiting"));
        assert!(help.contains("Oscillator frequency in hertz"));
        assert!(help.contains("Output amplitude"));
    }
}
