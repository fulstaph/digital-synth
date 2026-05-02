# Digital Synth

Digital Synth is a documentation-first project for designing a programmable digital synthesizer. The immediate goal is to build a rigorous synthesis knowledge base and a clear conceptual architecture before choosing a technical stack or writing implementation code.

This repository should make the eventual instrument easier to design by separating the musical system from the future implementation details. The docs define what the synthesizer should do, why each part matters, and how the major pieces should relate.

## Project Goals

The project should eventually become a digital instrument that is useful for learning, sound design, composition, and performance. The first goal is not to ship a minimal tone generator. The first goal is to understand the instrument deeply enough that implementation choices can be made deliberately.

Primary goals:

- Build a strong conceptual foundation for digital synthesis.
- Define a modular architecture for sound generation, modulation, voice handling, effects, presets, and performance control.
- Keep synthesis concepts independent from any programming language, framework, operating system, plugin format, or user-interface toolkit until the design is mature.
- Document terms, tradeoffs, and design constraints clearly enough that future implementation work has stable references.
- Design for expressive musical use, not only technical correctness.

Secondary goals:

- Make the synthesizer approachable for learning synthesis.
- Keep the architecture extensible so additional synthesis methods can be added later.
- Preserve real-time audio constraints as a first-class design concern.
- Support modern expressive control concepts such as per-note modulation, macro controls, and performance gestures.
- Make preset design, exploration, and sound discovery part of the core experience.

## Product Vision

The synthesizer should feel like an instrument, a laboratory, and a teaching tool at the same time.

As an instrument, it should respond quickly, sound good across common musical roles, and support expressive playing. As a laboratory, it should make internal signal flow and modulation relationships visible and editable. As a teaching tool, it should use clear organization so that a user can connect what they hear to the underlying synthesis concepts.

The first mature version should probably be a hybrid subtractive synthesizer: a subtractive core with carefully designed modulation, visual feedback, and room to add wavetable, FM, granular, or physical-modeling features later. Subtractive synthesis is a strong first foundation because it is musically useful, historically central, and easy to reason about.

## Candidate Feature Set

Core instrument features:

- Polyphonic voice engine with configurable voice count.
- Monophonic mode for basses and leads.
- Voice stealing behavior that avoids obvious clicks and musical surprises.
- Multiple oscillators per voice.
- Common waveforms such as sine, triangle, sawtooth, square, pulse, and noise.
- Detune, phase, oscillator sync, and unison concepts.
- Mixer stage for balancing oscillators, noise, and future sources.
- Filter section with low-pass, high-pass, band-pass, notch, and possibly multimode behavior.
- Resonance, drive, and key tracking.
- Amplitude envelope and modulation envelopes.
- Low-frequency oscillators for vibrato, tremolo, filter movement, pulse-width modulation, and rhythmic movement.
- Modulation matrix for routing sources to destinations.
- Macro controls that move multiple parameters at once.
- Built-in effects such as chorus, delay, reverb, distortion, EQ, and compression.
- Preset system with metadata, categories, tags, favorites, and author notes.

Expressive performance features:

- Velocity sensitivity.
- Pitch bend.
- Mod wheel and assignable continuous controls.
- Aftertouch or pressure.
- Per-note expression as a long-term design target.
- Glide, legato, retrigger, and note-priority modes.
- Sustain pedal behavior.
- Keyboard tracking for filter, pitch, envelope, and modulation scaling.
- Optional microtuning and alternative tuning support.

Learning and sound-design features:

- Oscilloscope-style waveform view.
- Spectrum analyzer.
- Filter response visualization.
- Envelope and LFO shape visualization.
- Modulation activity meters.
- Signal-flow overview.
- Patch notes explaining how a sound works.
- Safe randomization with musical constraints.
- Patch morphing between two or more states.
- Sound-design recipes for common targets such as bass, pad, pluck, lead, drone, percussion, and effects.

Quality and reliability features:

- Clear separation between real-time audio work and non-real-time work.
- Parameter smoothing to avoid clicks and zipper noise.
- Gain staging and headroom rules.
- Anti-aliasing strategy for bright oscillators and nonlinear effects.
- Preset compatibility policy.
- Testable behavior for voice allocation, envelopes, modulation, and preset loading.
- Documentation that explains constraints before implementation begins.

## Possible Development Stages

Stage 0: Knowledge base and architecture

- Define synthesis terms.
- Document digital audio fundamentals.
- Compare synthesis methods.
- Define the conceptual architecture.
- Identify real-time audio constraints.
- Define the initial product scope.
- Avoid implementation stack decisions.

Stage 1: Minimal musical voice

- Design one playable voice conceptually.
- Include oscillator, amplitude envelope, and gain control.
- Define expected behavior for note start, hold, and release.
- Establish timing and parameter-smoothing expectations.

Stage 2: Polyphonic subtractive instrument

- Add voice allocation and polyphony rules.
- Add oscillator mixing.
- Add filter behavior and filter envelope.
- Add LFO modulation.
- Define musical defaults and safe ranges.

Stage 3: Modulation-first sound design

- Add a modulation matrix.
- Add macros.
- Add per-voice and global modulation distinctions.
- Add visualization requirements for modulation depth and movement.
- Add randomization and patch-morphing concepts.

Stage 4: Presets and user workflow

- Define patch structure.
- Define metadata and categories.
- Define preset browsing, saving, comparing, and versioning.
- Add sound-design recipes and example patch descriptions.

Stage 5: Effects and polish

- Add delay, chorus, reverb, distortion, EQ, and dynamics concepts.
- Define effect routing.
- Add gain-staging guidance.
- Define quality modes for CPU-heavy processing.

Stage 6: Expressive control

- Add detailed behavior for velocity, pitch bend, pressure, mod wheel, sustain, and macros.
- Add per-note expression concepts.
- Add glide, legato, retrigger, mono priority, and microtuning behavior.

Stage 7: Advanced synthesis expansion

- Add wavetable synthesis.
- Add FM synthesis.
- Add granular synthesis.
- Add sample-based or physical-modeling options.
- Keep each synthesis type modular so it can share modulation, effects, presets, and performance control.

Stage 8: Productization decisions

- Choose technical stack.
- Choose distribution target.
- Choose whether the synth is standalone, plugin-based, browser-based, embedded, or multi-target.
- Define performance budgets.
- Define platform-specific constraints.

## Documentation Map

- [Project instructions](AGENTS.md)
- [Claude-oriented project context](CLAUDE.md)
- [Docs index](docs/README.md)
- [Conceptual architecture](docs/architecture.md)
- [Digital audio fundamentals](docs/knowledge-base/digital-audio-fundamentals.md)
- [Synthesis methods](docs/knowledge-base/synthesis-methods.md)
- [Oscillators and sound sources](docs/knowledge-base/oscillators-and-sound-sources.md)
- [Filters and tone shaping](docs/knowledge-base/filters-and-tone-shaping.md)
- [Modulation and envelopes](docs/knowledge-base/modulation-and-envelopes.md)
- [Voices and performance control](docs/knowledge-base/voices-and-performance-control.md)
- [Effects, mixing, and output](docs/knowledge-base/effects-mixing-and-output.md)
- [Glossary](docs/knowledge-base/glossary.md)
- [Research sources](docs/knowledge-base/research-sources.md)

## Current Boundaries

This project is intentionally pre-implementation.

For now:

- Do not choose a technical stack.
- Do not add implementation code as part of synthesis research.
- Do not optimize for a specific runtime, plugin format, UI framework, or deployment target.
- Do not treat the current placeholder program as the product architecture.
- Do keep documentation specific, practical, and musically grounded.

