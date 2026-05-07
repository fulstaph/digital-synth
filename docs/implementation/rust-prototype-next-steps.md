# Rust Prototype Next Steps

This document translates the early Python synthesizer tutorial path into a Rust implementation sequence. It is an implementation spike roadmap, not the final product architecture for Digital Synth.

The current Rust prototype corresponds to Michael Skiles' [Part 1: Stream player](https://mskiles.com/blog/how-to-build-a-synthesizer-with-python-part-1/): it creates a project, opens an audio stream, and feeds a temporary 440 Hz sine wave into the stream callback.

## Current Step: Part 1 Equivalent

The Rust Part 1 implementation should remain intentionally small.

Delivered behavior:

- A standalone Rust binary runs from `cargo run`.
- CPAL opens the default output device.
- A `StreamPlayer` owns the stream and keeps it alive.
- A temporary `SineGenerator` produces a continuous A440 tone.
- The binary accepts `--duration-seconds N` for a bounded manual playback check.
- Unit tests cover the sine generator, phase continuity, silence for non-positive frequency, channel duplication, and CLI parsing.
- CI runs formatting, clippy, and tests without requiring an audio device.

Manual check:

```bash
cargo run -- --duration-seconds 2
```

Start with low monitor volume. A continuous sine tone is simple but can still be uncomfortable or unexpectedly loud.

## Next Step 1: Signal Components

Source: Skiles [Part 2](https://mskiles.com/blog/how-to-build-a-synthesizer-with-python-part-2/).

Goal: replace the temporary sine generator with a small signal-component model.

Recommended Rust work:

- Define a `SignalComponent` trait that can fill a mono buffer for the current audio block.
- Move `SineGenerator` into a `SineOscillator` component.
- Add a `SquareOscillator` component.
- Add a `Gain` component that scales one child component.
- Add a `Mixer` component that averages or sums child components with clipping protection.
- Keep all component processing allocation-free after construction.

Acceptance criteria:

- A sine oscillator and square oscillator can both render blocks.
- A mixer can combine both sources into one mono signal.
- Tests verify oscillator bounds, mixer gain behavior, and clipping protection.
- The live stream player consumes a component instead of a special-case sine generator.

## Next Step 2: MIDI Input And Command Translation

Source: Skiles [Part 3](https://mskiles.com/blog/how-to-build-a-synthesizer-with-python-part-3/).

Goal: make the prototype respond to note input while keeping MIDI outside the audio engine.

Recommended Rust work:

- Use `midir` to list and open MIDI input ports.
- Translate raw MIDI note messages into project-owned synth commands.
- Keep MIDI concepts out of oscillator internals.
- Use a realtime-safe queue such as `rtrb` to pass commands from the MIDI thread to the audio callback.
- Start with monophonic note state: note on sets oscillator frequency, note off silences it if it matches the current note.

Acceptance criteria:

- The app can list available MIDI input ports.
- A selected MIDI input port can control the prototype.
- Note-on messages set oscillator frequency from MIDI note number.
- Note-off messages silence the active note predictably.
- The audio callback does not block on MIDI input or allocate while processing.

## Next Step 3: Realtime Control Boundary

Goal: make the audio/control split explicit before adding more musical features.

Recommended Rust work:

- Define an internal command enum for note on, note off, and shutdown.
- Allocate communication buffers before playback starts.
- Decide what happens when the command queue is full.
- Keep logging, MIDI device management, command parsing, and user-facing errors outside the audio callback.

Acceptance criteria:

- The audio callback only reads pending commands, updates audio state, and writes samples.
- Queue overflow behavior is documented and tested.
- Command translation can be tested without an audio device.

## Next Step 4: Musical Voice Design

Goal: move from a tone generator to the beginning of a synthesizer voice.

Recommended Rust work:

- Add amplitude state so note off can stop or release sound instead of only setting frequency to zero.
- Add a minimal ADSR envelope after the signal-component model is stable.
- Define parameter ranges according to the knowledge base's parameter design guidance.
- Keep polyphony out of this step; one monophonic voice is enough.

Acceptance criteria:

- Note-on starts audible output.
- Note-off transitions to silence without clicks.
- Envelope timing is tested with deterministic buffers.
- The init sound remains quiet and predictable.

## Deferred Work

The following are intentionally not part of the next implementation step:

- Plugin packaging with CLAP, VST3, NIH-plug, or Clack.
- GUI or visualization.
- Preset files.
- Polyphonic voice allocation.
- Effects.
- Advanced anti-aliasing beyond documenting the limitation of naive square waves.

Plugin work should be revisited after the standalone engine can generate, mix, and respond to note events. Kwarf's [CLAP synthesizer series](https://kwarf.com/2024/07/writing-a-clap-synthesizer-in-rust-part-1/) is the best current reference for that transition.
