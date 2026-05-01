# Conceptual Architecture

This document defines the high-level architecture for Digital Synth without choosing a technical stack. It describes the synthesizer as a set of musical and signal-flow responsibilities.

## Architectural Goal

Digital Synth should be designed as a modular real-time instrument. It should accept musical gestures, allocate voices, generate sound, shape that sound over time, mix voices together, process the result through effects, and deliver stable audio output.

The architecture must preserve two ideas at the same time:

- A synthesizer is a musical instrument controlled by humans and automation.
- A synthesizer is also a real-time signal system where timing errors become audible glitches.

The design should therefore separate fast, deterministic audio behavior from slower editing, browsing, visualization, and file-management behavior.

## System Overview

The conceptual signal path is:

Performance input becomes note and control events.

Note and control events are interpreted by the performance layer.

The voice allocator decides which voices should start, continue, release, or stop.

Each active voice generates one note or sound event.

Per-voice sound sources create raw audio.

Per-voice modulation changes pitch, amplitude, filter cutoff, oscillator shape, and other destinations over time.

Per-voice filters and shaping stages sculpt the raw sound.

All voices are mixed into a shared instrument signal.

Global modulation, effects, gain staging, and output processing shape the final sound.

Visualization and preset systems observe or control the instrument without disrupting real-time audio.

## Major Subsystems

### 1. Performance Input

The performance input subsystem translates musical intent into internal control events.

Inputs may eventually include:

- Keyboard notes.
- MIDI note messages.
- Velocity.
- Pitch bend.
- Mod wheel.
- Continuous controllers.
- Sustain pedal.
- Aftertouch or pressure.
- Per-note expression.
- Sequencer events.
- Automation.
- Macro control changes.

This subsystem matters because the same sound engine can feel very different depending on how input is interpreted. A bass patch may require monophonic legato and glide. A pad may require smooth sustain-pedal handling. A modern expressive controller may require independent pitch or pressure per note.

Design requirements:

- Preserve timing as accurately as possible.
- Keep note events distinct from continuous control changes.
- Distinguish global controls from per-note controls.
- Allow future support for richer expression without redesigning the whole engine.
- Normalize control ranges conceptually so modulation can be reasoned about consistently.

### 2. Voice Allocator

The voice allocator decides which internal voice handles each note.

It is responsible for:

- Starting new voices.
- Releasing voices when notes end.
- Reusing finished voices.
- Stealing active voices when polyphony is exhausted.
- Handling monophonic note priority.
- Handling legato and retrigger behavior.
- Tracking sustain pedal state.

The allocator is musically important because it determines continuity. Poor allocation causes cut-off releases, clicks, inconsistent envelopes, and chords that behave unpredictably.

Design requirements:

- Support polyphonic and monophonic modes.
- Define maximum voice count.
- Define note priority for mono mode, such as last-note, low-note, or high-note priority.
- Define voice stealing policy.
- Prefer stealing quiet, released, or oldest voices before stealing prominent active voices.
- Support legato behavior where appropriate.
- Handle repeated notes predictably.

### 3. Voice

A voice is one active note or sound event. It owns the state needed to render that note over time.

A typical voice includes:

- Pitch state.
- Oscillator phase state.
- Envelope states.
- Per-voice LFO states if LFOs are configured per voice.
- Filter state.
- Per-note expression state.
- Release state.
- Final voice gain.

Voices matter because musical articulation lives inside them. A chord is not only several pitches. It is several independently evolving sounds, each with its own envelope phase, modulation state, and performance data.

Design requirements:

- Voices should be self-contained enough that one voice can be started, released, or stopped without corrupting another.
- Voices should expose musically meaningful controls rather than implementation details.
- Voices should eventually support both per-voice and shared modulation sources.
- Voices should be able to finish naturally after note release.

### 4. Sound Sources

Sound sources create raw audio before filtering, effects, and final shaping.

Initial sources should include:

- Sine oscillator.
- Triangle oscillator.
- Sawtooth oscillator.
- Square oscillator.
- Pulse oscillator.
- Noise source.

Future sources may include:

- Wavetable oscillator.
- FM operator or operator group.
- Sample source.
- Granular source.
- Physical-modeling exciter or resonator.

Sound sources matter because they define the raw harmonic material. A filter can remove or emphasize material, but it cannot restore harmonics that were never present.

Design requirements:

- Avoid aliasing as a first-class quality concern.
- Support pitch control in musical units.
- Support detune.
- Support level and pan per source if useful.
- Support oscillator sync or phase reset as optional behavior.
- Keep advanced source types modular.

### 5. Mixer

The mixer combines sound sources inside a voice and later combines voices into the instrument output.

It is responsible for:

- Balancing oscillator levels.
- Combining noise with pitched sources.
- Managing gain before filters and nonlinear shaping.
- Combining all active voices.
- Preserving headroom.

Mixing matters because many synthesizer problems are gain-staging problems. Too much signal before a filter or distortion stage can change tone dramatically. Too much summed polyphony can clip. Too little signal can make effects and dynamics behave poorly.

Design requirements:

- Define gain ranges musically.
- Provide enough headroom for unison and polyphony.
- Decide where intentional drive can occur.
- Prevent accidental clipping from being the default behavior.

### 6. Filters And Tone Shaping

Filters shape the frequency spectrum. Tone-shaping stages may also include drive, saturation, wavefolding, or equalization.

Important filter types:

- Low-pass.
- High-pass.
- Band-pass.
- Notch.
- Peak or bell.
- Comb.
- State-variable or multimode behavior.

Filters matter because they turn static waveforms into expressive sounds. Cutoff movement can make a patch speak, breathe, brighten, darken, swell, or snap.

Design requirements:

- Include cutoff and resonance as core controls.
- Support modulation of cutoff.
- Consider key tracking so higher notes can remain bright.
- Consider velocity sensitivity for articulation.
- Avoid zipper noise when cutoff changes.
- Define safe resonance behavior.

### 7. Modulation System

The modulation system routes changing control signals to sound parameters.

Common modulation sources:

- Envelopes.
- LFOs.
- Velocity.
- Key tracking.
- Mod wheel.
- Aftertouch or pressure.
- Random values.
- Step sequencer lanes.
- Macros.
- Per-note expression.

Common modulation destinations:

- Pitch.
- Oscillator level.
- Oscillator shape.
- Pulse width.
- Filter cutoff.
- Filter resonance.
- Amplifier gain.
- Pan.
- Effect parameters.
- Wavetable position.
- FM amount.

Modulation matters because it creates life. Without modulation, even good oscillators sound static. With good modulation, simple sources can become expressive instruments.

Design requirements:

- Support bipolar and unipolar sources.
- Support modulation depth.
- Support per-voice modulation and global modulation.
- Support smoothing where needed.
- Avoid hidden modulation that makes patches hard to understand.
- Provide visual feedback for active modulation.
- Make modulation matrix behavior explicit and inspectable.

### 8. Envelopes

Envelopes are time-based modulation sources, usually triggered by notes.

The most important envelope is the amplifier envelope, which controls loudness over the life of a note. Filter and modulation envelopes shape brightness, pitch movement, timbre, or effects.

Envelopes matter because they define articulation. A pluck, pad, organ, brass swell, snare hit, and drone can use similar oscillators but completely different envelopes.

Design requirements:

- Include attack, decay, sustain, and release.
- Consider hold, delay, looping, and curve shape as future options.
- Allow envelope amount to be positive or negative for modulation destinations.
- Define retrigger behavior.
- Define legato behavior.
- Avoid clicks from extremely fast changes unless intentionally allowed.

### 9. Low-Frequency Oscillators

An LFO is an oscillator used as a control source rather than an audible source.

LFOs commonly control:

- Pitch for vibrato.
- Amplitude for tremolo.
- Filter cutoff for wah or motion.
- Pan for stereo movement.
- Pulse width for animation.
- Wavetable position for evolving digital sound.

LFOs matter because they create recurring movement. They can be subtle and expressive or rhythmic and obvious.

Design requirements:

- Support common shapes.
- Support rate in free-running and tempo-related concepts.
- Support fade-in or delay.
- Support phase reset options.
- Distinguish per-voice LFOs from global LFOs.
- Allow depth control from macros or performance gestures.

### 10. Effects

Effects process the mixed sound after synthesis, though some effects may eventually exist per voice.

Important effects:

- Distortion or saturation.
- Chorus.
- Phaser.
- Flanger.
- Delay.
- Reverb.
- EQ.
- Compressor.
- Limiter.

Effects matter because they place the raw synth tone into a musical context. A dry oscillator-filter sound can be useful, but many finished synth sounds depend on stereo movement, ambience, saturation, and time-based processing.

Design requirements:

- Define effect order.
- Preserve gain staging.
- Allow bypass per effect.
- Avoid effects hiding weak core synthesis behavior.
- Consider quality modes for CPU-heavy processing.
- Make wet and dry balance clear.

### 11. Presets

Presets store a complete instrument state.

A good preset system includes:

- Parameter values.
- Modulation routings.
- Macro assignments.
- Effect settings.
- Category.
- Tags.
- Author.
- Description.
- Performance notes.
- Compatibility information.

Presets matter because they are both creative artifacts and test cases. A synth with great architecture but poor preset workflow feels unfinished.

Design requirements:

- Support stable patch recall.
- Support metadata.
- Support browsing by category and tags.
- Support favorites.
- Support patch comparison.
- Support future compatibility migration.
- Encourage descriptive patch notes.

### 12. Visualization And Learning Layer

Visualization should help users connect controls to sound.

Useful visualizations:

- Waveform view.
- Spectrum analyzer.
- Filter response curve.
- Envelope curve.
- LFO curve.
- Modulation matrix activity.
- Voice activity display.
- Signal-flow diagram.

Visualization matters because synthesis is often abstract. If the user can see a filter envelope opening a low-pass filter, they can learn faster and edit more confidently.

Design requirements:

- Visualizers should observe the synth without interfering with audio stability.
- Visualizers should be accurate enough to teach but not so busy that they distract.
- Visual feedback should reveal hidden movement such as macro-controlled modulation.

## Real-Time And Non-Real-Time Boundaries

Digital audio is deadline-driven. The audio engine must produce each buffer before the output device needs it. If it misses the deadline, the result is usually a click, pop, dropout, or stutter.

Real-time responsibilities:

- Render active voices.
- Apply modulation needed for the current audio block.
- Process filters and effects.
- Mix output.
- Respond to scheduled note and parameter events.

Non-real-time responsibilities:

- Load and save presets.
- Search preset metadata.
- Render complex visualizations.
- Analyze samples.
- Build large lookup tables.
- Manage files.
- Update slow user-interface state.
- Fetch remote data if that ever exists.

Design rule:

The architecture should never require the audio-rendering path to wait for slow work.

## Global Versus Per-Voice Design

Some synth behavior should happen per voice. Other behavior should happen once globally.

Per-voice examples:

- Oscillator phase.
- Note pitch.
- Note envelope.
- Per-note filter state.
- Per-note expression.
- Per-note modulation.

Global examples:

- Master volume.
- Master effects.
- Global LFO if shared movement is desired.
- Preset metadata.
- Transport-synced controls.
- Final limiter.

This distinction matters because per-voice modulation creates independent movement per note, while global modulation moves the whole instrument together. Both are useful. A pad may need every note to share a slow global sweep. An expressive controller may need each note to bend independently.

## Initial Architectural Recommendation

The first coherent architecture should be:

- Performance input layer.
- Voice allocator.
- Polyphonic subtractive voice.
- Per-voice oscillators.
- Per-voice mixer.
- Per-voice filter.
- Per-voice amplifier envelope.
- Modulation sources and routing.
- Global effects chain.
- Preset and metadata model.
- Visualization layer.

This architecture is narrow enough to build in stages and broad enough to support future wavetable, FM, granular, sample, or physical-modeling sources.

## Architecture Risks

Risk: The synth becomes a collection of disconnected effects and sources.

Mitigation: Keep voice, modulation, preset, and performance-control concepts shared across all synthesis methods.

Risk: The modulation system becomes powerful but unreadable.

Mitigation: Make routing visible, name sources and destinations clearly, and show active modulation depth.

Risk: Early stack decisions limit musical behavior.

Mitigation: Finish conceptual behavior before choosing implementation tools.

Risk: The synth sounds technically correct but musically flat.

Mitigation: Design expressive controls, modulation defaults, gain staging, and patch workflow as core features, not polish.

