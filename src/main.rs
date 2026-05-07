use std::process::ExitCode;
use std::thread;
use std::time::Duration;

use clap::Parser;
use digital_synth::cli::CliConfig;
use digital_synth::playback::stream_player::{SineGeneratorSettings, StreamPlayer};

fn main() -> ExitCode {
    let config = CliConfig::parse();

    let player = match StreamPlayer::play(SineGeneratorSettings {
        frequency_hz: config.frequency_hz,
        amplitude: config.amplitude,
    }) {
        Ok(player) => player,
        Err(error) => {
            eprintln!("failed to start audio stream: {error}");
            return ExitCode::from(1);
        }
    };

    eprintln!(
        "playing {} Hz sine wave at amplitude {} on {} at {} Hz, {} channel(s)",
        config.frequency_hz,
        config.amplitude,
        player.device_name(),
        player.sample_rate_hz(),
        player.channels()
    );

    match config.duration_seconds {
        Some(duration_seconds) => {
            thread::sleep(Duration::from_secs_f64(duration_seconds));
        }
        None => loop {
            thread::sleep(Duration::from_secs(1));
        },
    }

    ExitCode::SUCCESS
}
