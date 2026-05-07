# AGENTS.md

## Project Purpose

This repository is for designing a programmable digital synthesizer from first principles. The project remains documentation-first, but it now also contains a small Rust implementation spike that proves basic audio output and supports future design work.

Agents working in this repository should treat the project as a music-technology design project, not as a generic software scaffold. The important work at this stage is to clarify synthesis concepts, define behavior, document musical goals, and preserve architectural choices that will survive future implementation decisions.

## Current Phase

The project is in a documentation-first phase with one narrow implementation prototype.

The current Rust code is a learning and validation spike, not the final product architecture. It opens the default audio output device with CPAL and plays a quiet 440 Hz sine wave. This is equivalent to the first stream-player step in the referenced synthesizer tutorial path.

Allowed work:

- Create and improve conceptual documentation.
- Expand the synthesis knowledge base.
- Define terminology.
- Compare architectural options.
- Clarify product goals and development stages.
- Improve project guidance files.
- Add diagrams or explanatory prose when useful.
- Improve the current Rust prototype when the work directly supports the documented implementation spike.
- Add focused tests and CI checks for existing Rust behavior.

Avoid for now:

- Treating Rust, CPAL, or the current source layout as final product decisions.
- Choosing a plugin format.
- Choosing a UI toolkit.
- Adding broad implementation features before they are documented.
- Adding dependencies speculatively.
- Tying the conceptual architecture to the current prototype source tree.

## Current Implementation Snapshot

The implementation currently consists of:

- `Cargo.toml` and `Cargo.lock` for a Rust 2024 crate named `digital-synth`.
- `cpal` for audio output and `clap` for the prototype command-line interface.
- `src/main.rs`, which parses the CLI and starts audio playback.
- `src/playback/stream_player.rs`, which owns CPAL stream setup and duplicates mono generated samples across output channels.
- `src/synthesis/sine_generator.rs`, which owns amplitude, phase increment, and phase for a temporary sine-wave source.
- `.github/workflows/rust.yml`, which runs Rust formatting, clippy, and tests in CI.

The current prototype can be manually checked with:

```bash
cargo run -- --frequency-hz 220 --amplitude 0.1 --duration-seconds 2
```

This command requires access to a local audio output device. CI should not require live audio playback.

## Documentation Standards

Documentation should be detailed, precise, and practical. The reader should be able to understand what a synthesizer component does, why it matters musically, and what design tradeoffs are involved.

Good documentation in this project:

- Defines terms before relying on them.
- Explains musical importance, not only technical mechanics.
- Separates concepts from implementation details.
- Names tradeoffs clearly.
- Avoids vague claims such as "make it better" or "add warmth" unless the behavior is explained.
- Describes expected behavior in enough detail that it can later become implementation criteria.
- Uses examples as prose descriptions, not code.

## Project Architecture Principles

The synthesizer should be designed as a set of conceptual subsystems with clear responsibilities.

Important subsystems:

- Input and performance control.
- Voice allocation.
- Per-voice sound generation.
- Modulation.
- Filtering and tone shaping.
- Mixing and gain staging.
- Effects.
- Presets and patch metadata.
- Visualization and learning aids.
- Non-real-time editing and management.

The architecture should preserve the distinction between real-time audio behavior and slower control, editing, browsing, or visualization behavior. Even before choosing a stack, this distinction matters because real-time audio systems fail audibly when deadlines are missed.

## Writing Style

Use plain technical prose. Prefer clear definitions, short sections, and concrete examples. Avoid marketing language. Avoid implementation snippets. Avoid assuming the reader already knows synthesis jargon.

When adding a new synthesis topic, include:

- Definition.
- Why it matters.
- How it affects sound.
- Common controls.
- Common mistakes.
- Design implications for this project.

## Stack Neutrality

Do not present the current Rust prototype as the final technical stack. Rust and CPAL are acceptable to discuss as the current implementation spike. Plugin format, UI framework, distribution target, preset format, and final audio architecture remain future decisions.

Examples of acceptable phrasing:

- "The future implementation should provide..."
- "A later stack decision must account for..."
- "The architecture should allow..."
- "The current Rust prototype demonstrates..."
- "The CPAL stream player is a temporary standalone validation step..."

Examples of unacceptable phrasing during the current phase:

- "The synth will be built with..."
- "Use this framework for..."
- "The plugin format is..."
- "The current prototype source tree is the final engine architecture..."

## Rust Prototype Standards

When changing Rust code:

- Keep the audio callback small and realtime-conscious: no logging, allocation, blocking I/O, or lock acquisition in the render path.
- Add tests for deterministic DSP behavior before or with behavior changes.
- Keep runtime dependencies minimal and justify any new crate in documentation.
- Run `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --all --locked` before completion.
- Preserve the distinction between prototype mechanics and conceptual architecture.

## Completion Criteria For Documentation Changes

A documentation change is useful when it improves one of these areas:

- The project vision is clearer.
- A synthesis term is easier to understand.
- A subsystem boundary is better defined.
- A future implementation choice becomes less ambiguous.
- A musical feature has clearer expected behavior.
- A tradeoff is documented instead of hidden.
