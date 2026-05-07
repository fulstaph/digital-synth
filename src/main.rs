use std::process::ExitCode;
use std::thread;
use std::time::Duration;

mod cli;

use clap::Parser;
use cli::CliConfig;
use digital_synth::prototype::PrototypePlaybackConfig;

fn main() -> ExitCode {
    let config: PrototypePlaybackConfig = CliConfig::parse().into();

    let player = match config.play() {
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

    match config.duration {
        Some(duration) => thread::sleep(duration),
        None => loop {
            thread::sleep(Duration::from_secs(1));
        },
    }

    ExitCode::SUCCESS
}
