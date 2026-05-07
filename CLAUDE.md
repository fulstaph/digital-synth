# CLAUDE.md

## Project Context

Digital Synth is currently a documentation-first project for designing a digital synthesizer. The repository should develop a strong conceptual foundation while maintaining a small Rust implementation spike that validates basic audio output.

The project should not be narrowed to a final technical stack yet. Rust, CPAL, and clap are present as the current prototype path, but plugin format, user-interface framework, deployment target, preset format, and final engine architecture remain open decisions.

## What This Project Is Trying To Become

The long-term goal is an expressive programmable synthesizer that can serve as:

- A playable musical instrument.
- A sound-design laboratory.
- A learning environment for synthesis concepts.
- A platform for experimenting with multiple synthesis methods.

The likely first mature instrument is a subtractive synthesizer with a strong modulation system, careful voice behavior, useful effects, preset management, and visual explanations of what the sound engine is doing.

## Current Documentation Priorities

Prioritize:

- Clear synthesis definitions.
- Conceptual architecture.
- Musical goals and user-facing behavior.
- Real-time audio constraints explained at a design level.
- Development stages.
- Tradeoffs among synthesis methods.
- Preset, modulation, and performance-control concepts.

Do not prioritize:

- Broad implementation code outside the documented Rust prototype.
- Treating language-specific details as product architecture.
- Framework-specific APIs beyond the current CPAL stream-player spike.
- Dependency selection before a documented need exists.
- Build tooling.
- Platform packaging.

## Current Implementation

The repository now contains a small Rust 2024 crate named `digital-synth`.

Current behavior:

- `src/main.rs` adapts the clap-based CLI into library playback configuration and manages process lifetime.
- `src/cli.rs` owns command-line syntax, generated usage text, and argument validation.
- `src/prototype.rs` owns the current prototype playback configuration and starts playback without depending on CLI concepts.
- `src/playback/stream_player.rs` opens the default output device with CPAL, owns the stream, and writes mono generated samples to each output channel.
- `src/synthesis/sine_generator.rs` generates a temporary sine tone using amplitude, phase increment, and phase state.
- Unit tests cover CLI parsing, CLI-to-library configuration conversion, prototype playback settings, channel writing, sine amplitude bounds, phase continuity, and silence for non-positive frequencies.
- `.github/workflows/rust.yml` runs formatting, clippy, and unit tests.

Useful commands:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
cargo run -- --help
cargo run -- --duration-seconds 2
cargo run -- --frequency-hz 220 --amplitude 0.1 --duration-seconds 2
```

The playback command requires a local audio device and should not be used as a CI requirement.

## Important Design Principles

The synthesizer should be modular, expressive, and understandable.

Modular means each major part has a clear responsibility: input handling, voices, oscillators, modulation, filters, effects, presets, and visualization should be separable at the design level.

Expressive means the instrument should respond musically to performance gestures such as velocity, pitch bend, pressure, modulation controls, macros, legato, glide, and eventually per-note expression.

Understandable means the documentation and future interface should help users connect controls to sound. A user should be able to learn why a patch behaves the way it does.

## Documentation Rules

Keep the writing detailed but grounded. Define terms before using them. When describing a feature, explain both the technical role and the musical reason it matters.

Avoid code examples. Avoid pseudocode. Avoid choosing a stack. Avoid adding implementation-specific recommendations unless the user explicitly asks for that phase.

When expanding the knowledge base, prefer this shape:

- What the term or subsystem means.
- Why it matters.
- How it affects sound.
- Which controls are usually exposed.
- What mistakes or edge cases matter.
- How it should influence this project.

## Project Boundary

The Rust source tree is a prototype, not the product design. The authoritative material for long-term architecture remains the documentation. If prototype behavior and conceptual docs diverge, update the docs or explicitly identify the prototype as temporary.

## Rust Prototype Rules

When modifying Rust code:

- Keep the audio callback realtime-conscious: no logging, blocking work, file I/O, allocation, or locks in the render loop.
- Keep command-line parsing separate from playback behavior. CLI types can convert into library configuration, but playback and synthesis modules should not depend on `clap`.
- Prefer small, deterministic units with tests before expanding musical behavior.
- Keep runtime dependencies minimal and justify new crates in documentation.
- Do not introduce plugin, UI, preset, or broad synthesis architecture decisions as side effects of prototype work.
