use std::env;
use std::process::ExitCode;
use std::thread;
use std::time::Duration;

use digital_synth::cli::CliConfig;
use digital_synth::playback::stream_player::StreamPlayer;

fn main() -> ExitCode {
    let config = match CliConfig::parse(env::args()) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("{error}");
            eprintln!("usage: digital-synth [--duration-seconds N]");
            return ExitCode::from(2);
        }
    };

    let player = match StreamPlayer::play_default_sine() {
        Ok(player) => player,
        Err(error) => {
            eprintln!("failed to start audio stream: {error}");
            return ExitCode::from(1);
        }
    };

    eprintln!(
        "playing 440 Hz sine wave on {} at {} Hz, {} channel(s)",
        player.device_name(),
        player.sample_rate_hz(),
        player.channels()
    );

    match config.duration_seconds {
        Some(duration_seconds) if duration_seconds > 0.0 => {
            thread::sleep(Duration::from_secs_f64(duration_seconds));
        }
        Some(_) => {}
        None => loop {
            thread::sleep(Duration::from_secs(1));
        },
    }

    ExitCode::SUCCESS
}
