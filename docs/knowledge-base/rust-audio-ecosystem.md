# Rust Audio Ecosystem

This document records Rust audio libraries that are relevant to the first implementation spike. It is not a final stack decision for Digital Synth. The purpose is to explain which crates are useful for learning, prototyping, and later product decisions.

The first Rust implementation intentionally uses only CPAL. Other libraries remain candidates for later stages when the project has a concrete need for their scope.

## Selection Principles

A Rust audio library should be evaluated by the role it plays in the synthesizer.

- Audio device access: opening an output device and supplying samples on time.
- DSP building blocks: oscillators, filters, sample conversion, graphs, and analysis.
- MIDI and control input: receiving note and controller messages outside the audio callback.
- Realtime communication: moving control events into the audio thread without locks or allocation.
- File and analysis tools: rendering WAV files, loading reference material, and computing spectra.
- Plugin packaging: exposing the engine to DAWs after the standalone engine is musically useful.

The first implementation step needs audio output and a simple sample callback. It does not need MIDI, a plugin wrapper, a DSP graph framework, a GUI, or a preset format.

## CPAL

[CPAL](https://docs.rs/cpal/latest/cpal/) is a low-level cross-platform audio I/O library. It exposes hosts, devices, stream configurations, and callback-driven input or output streams.

Why it fits Part 1:

- It is the closest Rust equivalent to the PyAudio role in Michael Skiles' Part 1 article.
- It makes the audio callback boundary explicit, which is useful for learning realtime constraints.
- It allows the prototype to use the system default output device without choosing a plugin format.

Tradeoffs:

- CPAL is intentionally low-level. The project must handle sample format selection, channel layout, and stream lifetime.
- Live playback depends on host audio configuration, so CI should compile and test logic but should not require an audio device.

Current role: use CPAL for the standalone Part 1 stream player.

## Rodio

[Rodio](https://docs.rs/rodio/latest/rodio/) is a higher-level audio playback library. Its main concept is a `Source` that can be added to an output stream or mixer.

Why it matters:

- It is useful for playing decoded audio files or simple application sounds.
- It re-exports CPAL for playback and hides some stream setup details.

Why it is deferred:

- Digital Synth needs to understand and control the audio callback path directly.
- Rodio's higher-level playback model is less useful for specifying a synthesizer engine from first principles.

Current role: defer unless the project later needs simple file playback or auditioning.

## DASP

[DASP](https://docs.rs/dasp/latest/dasp/) provides low-level digital audio signal processing traits and utilities, including sample, frame, and signal abstractions.

Why it matters:

- Its `Sample`, `Frame`, and `Signal` ideas are relevant once Digital Synth defines reusable oscillator and processing traits.
- It can help avoid ad hoc sample conversion and channel-layout code.

Why it is deferred:

- The first component model should be small and project-specific so the team understands the signal flow before adopting broader abstractions.

Current role: study before the Part 2 signal-component rewrite, but do not add it yet.

## FunDSP

[FunDSP](https://docs.rs/fundsp/latest/fundsp/) is an audio processing and synthesis library with composable graph notation and DSP components.

Why it matters:

- It is a useful reference for graph-shaped DSP systems.
- It can help evaluate how Rust types can represent signal flow.

Why it is deferred:

- Digital Synth is trying to document and build the first signal model deliberately, not import a full graph vocabulary at the start.
- Adopting FunDSP too early could hide design questions about voices, modulation, gain staging, and parameter ownership.

Current role: reference for later graph design, not a dependency for Part 1.

## MIDI: Midir

[Midir](https://docs.rs/midir/latest/midir/) provides MIDI input and output access. It exposes input and output ports and connections.

Why it matters:

- It is the likely Rust equivalent for the MIDI listener stage inspired by Skiles Part 3.
- It can support note input before the project commits to a plugin host.

Why it is deferred:

- MIDI introduces threading, command translation, note state, and realtime communication.
- The Part 1 goal is only a deterministic tone through an audio stream.

Current role: likely candidate for the first MIDI prototype.

## Realtime Messaging: RTRB

[RTRB](https://docs.rs/rtrb/latest/rtrb/) is a realtime-safe single-producer single-consumer ring buffer. It allocates a fixed-capacity buffer at construction, then provides lock-free and wait-free read/write operations.

Why it matters:

- MIDI and UI/control events must reach the audio callback without blocking it.
- A fixed-capacity queue makes backpressure explicit instead of hiding allocations or locks.

Why it is deferred:

- Part 1 has no control thread and no event stream.

Current role: evaluate before MIDI or parameter automation enters the prototype.

## WAV Rendering: Hound

[Hound](https://docs.rs/hound/latest/hound/) reads and writes WAV files.

Why it matters:

- Offline WAV rendering is useful for tests, examples, and documentation audio snippets.
- It can make oscillator and envelope behavior easier to inspect without relying on a live output device.

Why it is deferred:

- The requested first step is live stream playback, not offline rendering.

Current role: good candidate for a future test and example-rendering tool.

## FFT And Analysis: RustFFT And RealFFT

[RustFFT](https://docs.rs/rustfft/latest/rustfft/) is a high-performance FFT library. [RealFFT](https://docs.rs/realfft/latest/realfft/) wraps RustFFT for real-valued input and real-to-complex transforms.

Why they matter:

- Spectrum analysis is part of the project's visualization and learning goals.
- Real-valued audio is the common input shape for oscillator and output analysis.

Why they are deferred:

- Part 1 does not need spectral analysis.
- FFT infrastructure belongs with visualization, oscillator verification, or later spectral processing work.

Current role: future candidates for spectrum analysis and audio verification.

## Plugin Frameworks: NIH-Plug And Clack

[NIH-plug](https://github.com/robbert-vdh/nih-plug) is a Rust VST3 and CLAP plugin framework. [Clack](https://docs.rs/clack-plugin/latest/clack_plugin/) provides low-level safe Rust wrappers for the CLAP plugin API.

Why they matter:

- Digital Synth will eventually need to decide whether it is a standalone app, plugin, or both.
- CLAP is especially relevant because of its modulation and note-expression direction.
- Kwarf's [Writing a CLAP synthesizer in Rust](https://kwarf.com/2024/07/writing-a-clap-synthesizer-in-rust-part-1/) is a useful implementation reference once plugin work begins.

Why they are deferred:

- Plugin wrappers introduce host lifecycle, plugin descriptors, audio/note ports, state handling, packaging, and DAW testing.
- The standalone engine should first prove that it can generate, mix, and respond to notes without plugin-specific constraints.

Current role: revisit after the standalone engine reaches note-driven mono synthesis.

## Recommendation

Use CPAL only for the first Rust implementation spike. The next dependency decision should be made only when the prototype reaches a documented need:

- Add `midir` when MIDI input begins.
- Add `rtrb` or an equivalent when events must cross into the audio callback.
- Add `hound` when offline audio rendering becomes part of testing or documentation.
- Add `dasp` or `fundsp` only after the project-specific component model is understood.
- Add `nih-plug` or `clack` only after the standalone synth can generate, mix, and respond to note events.
