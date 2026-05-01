# Synthesis Methods

This document compares major synthesis methods and explains how each could fit into Digital Synth.

## Subtractive Synthesis

Subtractive synthesis starts with harmonically rich sound and removes or emphasizes parts of the spectrum using filters.

Typical signal path:

Oscillators and noise are mixed, passed through a filter, shaped by envelopes and LFOs, then processed by effects.

Common sources:

- Sawtooth waves.
- Square waves.
- Pulse waves.
- Noise.
- Detuned oscillator pairs.

Common controls:

- Oscillator waveform.
- Oscillator tuning.
- Oscillator mix.
- Filter cutoff.
- Filter resonance.
- Filter envelope amount.
- Amplifier envelope.
- LFO rate and depth.

Why it matters:

Subtractive synthesis is the best first foundation for this project. It is musically broad, historically important, and easy to connect to common sound-design goals.

Musical strengths:

- Bass.
- Lead.
- Pad.
- Brass-like sounds.
- Plucks.
- Percussion.
- Classic analog-style tones.
- Noise-based effects.

Limitations:

- It is less natural for highly complex evolving digital timbres unless extended.
- It can become static without modulation.
- It depends heavily on oscillator and filter quality.

Design implication:

Digital Synth should begin with a subtractive core and make modulation, voice behavior, and filter movement central.

## Additive Synthesis

Additive synthesis builds complex sounds by summing simpler components, usually sine waves called partials.

Terms:

- A partial is one frequency component of a sound.
- A harmonic partial is an integer multiple of the fundamental frequency.
- An inharmonic partial is not an integer multiple of the fundamental.

Common controls:

- Number of partials.
- Partial frequency.
- Partial amplitude.
- Partial phase.
- Spectral envelope.
- Time-varying partial behavior.

Why it matters:

Additive synthesis is theoretically powerful because any periodic sound can be described as a sum of sinusoidal components. It makes spectrum-level sound design explicit.

Musical strengths:

- Organs.
- Bells.
- Evolving pads.
- Resynthesized tones.
- Precise harmonic control.
- Educational visualization.

Limitations:

- It can require many parameters.
- It can be difficult to make intuitive.
- Natural sounds need time-varying spectra, not only static partial levels.

Design implication:

Additive synthesis is valuable as a future expansion, especially if the project includes spectral visualizations. It should not be the first core engine unless the project intentionally becomes a spectral synth.

## Frequency Modulation Synthesis

Frequency modulation synthesis changes the frequency of one oscillator using another oscillator.

Important terms:

- A carrier is the oscillator that is heard directly.
- A modulator is the oscillator that changes the carrier frequency.
- An operator is often an oscillator plus its amplitude envelope and routing role.
- An algorithm is a routing arrangement of operators.
- Modulation index describes how strongly the modulator affects the carrier.

Why it matters:

FM can create rich, bright, metallic, glassy, percussive, and evolving spectra using relatively few components.

Musical strengths:

- Electric piano.
- Bells.
- Mallets.
- Metallic percussion.
- Digital basses.
- Glassy pads.
- Complex attacks.

Limitations:

- Small parameter changes can produce large timbral changes.
- It can be unintuitive.
- It can alias if not handled carefully.
- Operator routing can be hard to visualize.

Design implication:

FM should be treated as a future sound-source module or advanced engine. If added, it needs strong visualization, carefully scaled controls, and useful presets.

## Phase Modulation

Phase modulation changes oscillator phase rather than frequency directly, but it can produce FM-like results.

Why it matters:

Many digital instruments associated with FM actually use phase modulation internally because it can be more stable and implementation-friendly.

Musical strengths:

- Similar to FM.
- Good for bright digital tones.
- Can be easier to make pitch-stable.

Design implication:

If the project later adds FM-like synthesis, it should document whether the musical model is FM, phase modulation, or a hybrid. The user-facing concept can remain operator-based.

## Wavetable Synthesis

Wavetable synthesis uses stored single-cycle waveforms or tables of waveforms. The synth can scan through these tables over time.

Important terms:

- A wavetable is a collection of one-cycle waveforms.
- Wavetable position selects or blends among waveforms.
- Interpolation smooths movement between table entries.
- Band-limited tables reduce aliasing at different pitches.

Common controls:

- Wavetable selection.
- Position.
- Position modulation.
- Warp or bend.
- Unison.
- Filter and effects.

Why it matters:

Wavetable synthesis is excellent for evolving digital timbres and modern electronic sounds.

Musical strengths:

- Evolving pads.
- Aggressive basses.
- Digital leads.
- Motion-heavy textures.
- Morphing sounds.

Limitations:

- Poor interpolation can sound rough.
- Bright tables can alias.
- Too many tables can become preset-browser clutter without good organization.

Design implication:

Wavetable synthesis is a strong second engine after subtractive synthesis because it can reuse the same voice, modulation, filter, effects, and preset architecture.

## Granular Synthesis

Granular synthesis breaks audio into very small pieces called grains and recombines them.

A grain usually has:

- Source position.
- Duration.
- Pitch.
- Amplitude.
- Envelope shape.
- Pan.
- Density.

Why it matters:

Granular synthesis can separate pitch, time, texture, and density in ways that traditional oscillators cannot.

Musical strengths:

- Ambient textures.
- Time-stretched samples.
- Clouds.
- Frozen sound.
- Glitches.
- Cinematic transitions.
- Abstract sound design.

Limitations:

- It can be CPU-intensive.
- It can become noisy or blurry.
- Grain parameters are highly interactive.
- It often needs good visualization.

Design implication:

Granular synthesis should be a later expansion. It should be designed as a source or layer that can reuse modulation, effects, presets, and macro controls.

## Sample-Based Synthesis

Sample-based synthesis plays recorded audio and shapes it with pitch, looping, envelopes, filters, and modulation.

Common features:

- One-shot playback.
- Looping.
- Key zones.
- Velocity layers.
- Root note.
- Start offset.
- Reverse playback.
- Crossfade looping.
- Sample start modulation.

Why it matters:

Samples provide realism and sonic detail that pure oscillators may not easily create.

Musical strengths:

- Drums.
- Acoustic instrument layers.
- Found sound.
- Vocal fragments.
- Hybrid textures.
- Attack transients.

Limitations:

- Requires sample management.
- Pitch shifting can sound unnatural over wide ranges.
- Looping can click or feel artificial.
- File loading is non-real-time work.

Design implication:

Sample playback should be designed carefully if added. It introduces file, memory, metadata, and browsing concerns that should not interfere with the core real-time engine.

## Physical Modeling

Physical modeling synthesizes sound by simulating the behavior of a vibrating system.

Examples:

- Plucked string.
- Bowed string.
- Blown pipe.
- Drum membrane.
- Plate.
- Resonating body.

Important concepts:

- Exciter: the energy source that starts or sustains vibration.
- Resonator: the body or system that responds.
- Damping: energy loss over time.
- Feedback: energy recirculation.
- Coupling: interaction between parts of the system.

Why it matters:

Physical modeling can produce expressive and organic sounds with strong performance response.

Musical strengths:

- Plucked strings.
- Flutes.
- Bowed textures.
- Percussion.
- Hybrid acoustic-electronic instruments.
- Expressive controller performance.

Limitations:

- Models can be hard to tune.
- Parameters may not map intuitively to musical expectations.
- Numerical stability matters.

Design implication:

Physical modeling is a valuable future direction if the project wants expressive playable instruments. It should not distract from first building a stable modulation and voice architecture.

## Waveshaping And Distortion Synthesis

Waveshaping transforms an input waveform through a nonlinear curve.

Related concepts:

- Saturation.
- Clipping.
- Folding.
- Rectification.
- Asymmetry.
- Drive.

Why it matters:

Nonlinear processing creates new harmonics. It can turn simple waveforms into aggressive, warm, or complex tones.

Musical strengths:

- Bass.
- Lead.
- Industrial tones.
- Percussion.
- Warm analog-style coloration.
- Aggressive digital textures.

Limitations:

- Nonlinear processing can alias heavily.
- Gain staging strongly affects tone.
- It can become harsh without filtering.

Design implication:

Waveshaping should be available as tone shaping or effects, but it must be designed with anti-aliasing and headroom in mind.

## Amplitude Modulation And Ring Modulation

Amplitude modulation changes the level of one signal using another signal. Ring modulation multiplies two signals, usually without preserving the original carrier in the same way simple amplitude modulation does.

Why it matters:

These techniques create sidebands, tremolo, metallic tones, and inharmonic textures.

Musical strengths:

- Tremolo.
- Metallic percussion.
- Sci-fi effects.
- Bell-like tones.
- Rhythmic movement.

Limitations:

- Inharmonic sidebands can easily sound dissonant.
- Strong modulation may reduce pitch clarity.

Design implication:

Amplitude modulation can exist both as a modulation destination and as an audio-rate synthesis feature. The architecture should distinguish slow control modulation from audio-rate signal multiplication.

## Vector And Morphing Synthesis

Vector synthesis blends several sound sources using a two-dimensional or multi-dimensional control.

Why it matters:

It gives users a performance-friendly way to move between timbres.

Musical strengths:

- Evolving pads.
- Hybrid textures.
- Performance morphs.
- Macro-controlled sound design.

Limitations:

- Requires careful gain normalization.
- Blending very different sounds can feel unpredictable.

Design implication:

Patch morphing and macro controls should be designed early enough that future vector-style features fit naturally.

## Recommended Method Roadmap

First: subtractive synthesis.

Reason:

- Broadest educational and musical value.
- Supports many classic sounds.
- Easy to document and visualize.
- Establishes voice, modulation, filter, and preset architecture.

Second: wavetable synthesis.

Reason:

- Reuses much of the subtractive architecture.
- Adds modern motion-heavy timbres.
- Makes modulation and visualization more valuable.

Third: FM or granular synthesis.

Reason:

- FM expands harmonic complexity.
- Granular expands texture and sample manipulation.
- Both benefit from a mature modulation and preset system.

Later: physical modeling and sample-based synthesis.

Reason:

- These require additional design around models, samples, memory, or performance gestures.

