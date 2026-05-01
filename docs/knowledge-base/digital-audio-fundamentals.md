# Digital Audio Fundamentals

Digital synthesis depends on representing sound as numbers. This document explains the essential digital audio concepts needed to design a synthesizer.

## Sound As Pressure Variation

Sound is changing air pressure over time. A microphone converts pressure changes into an electrical signal. A speaker converts an electrical signal back into pressure changes. A digital synthesizer creates the signal directly as numbers, then an audio system converts those numbers into sound.

The key idea is that a digital audio signal is a sequence of amplitude values. Each value represents the signal level at one instant.

## Sample

A sample is one numerical measurement of signal amplitude at a specific moment.

Why it matters:

- Every oscillator, filter, envelope, and effect eventually produces or modifies samples.
- Audio quality depends on producing the right sample values at the right times.
- Distortion, clipping, aliasing, and noise all show up as sample-level behavior.

Musical importance:

Samples are not meaningful to a musician individually, but they are the material from which all audible tone is made. A synthesizer that mishandles samples will sound noisy, harsh, unstable, or glitchy.

## Sample Rate

Sample rate is the number of samples per second. Common rates include 44.1 kHz, 48 kHz, 88.2 kHz, and 96 kHz.

Why it matters:

- It defines how often the synth computes audio.
- It determines the highest frequency that can be represented without aliasing.
- It affects CPU cost because higher sample rates require more calculations per second.
- It affects oscillator tuning, envelope timing, filter behavior, and delay times.

Design implication:

The synth should define time-based behavior in musical or physical units such as seconds, milliseconds, hertz, beats, and semitones, not in raw sample counts. Raw sample counts change when the sample rate changes.

## Nyquist Frequency

The Nyquist frequency is half the sample rate. At 48 kHz, the Nyquist frequency is 24 kHz.

Why it matters:

- Frequencies above Nyquist cannot be represented correctly.
- If a generated signal contains energy above Nyquist, that energy folds back into the audible range as aliasing.

Musical importance:

Aliasing often sounds metallic, glassy, harsh, or wrong. Sometimes aliasing is used as an effect, but most high-quality synthesizers avoid accidental aliasing in oscillators and nonlinear processors.

## Aliasing

Aliasing happens when a digital system tries to represent frequencies above Nyquist. Those frequencies fold back into lower frequencies that were not present in the intended signal.

Common causes in synthesizers:

- Naive sawtooth and square oscillators.
- Hard sync.
- Frequency modulation with bright spectra.
- Ring modulation.
- Distortion and waveshaping.
- Wavefolding.
- Rapid parameter changes.
- Pitching samples far above their recorded range.

Why it matters:

Aliasing is one of the most important quality issues in digital synthesis. Bright waveforms naturally contain many harmonics, and high notes can push those harmonics above Nyquist.

Design implication:

The synth should treat anti-aliasing as a core sound-quality requirement. Possible strategies include band-limited oscillators, oversampling, interpolation, filtering, and quality modes.

## Band-Limiting

Band-limiting means restricting a signal so it does not contain frequencies above a chosen limit, usually Nyquist or a safer margin below Nyquist.

Why it matters:

- Band-limited oscillators sound cleaner across the keyboard.
- Band-limited processing prevents unwanted folded frequencies.
- It allows bright sounds without uncontrolled digital harshness.

Musical importance:

Band-limiting helps a synth stay bright without becoming brittle. It is especially important for sawtooth, square, pulse, sync, and wavetable sounds.

## Bit Depth And Floating-Point Audio

Bit depth describes the resolution of stored or converted audio. Common file formats use 16-bit, 24-bit, or 32-bit floating-point representations. Inside a synthesizer, audio is usually best thought of conceptually as high-resolution floating-point values.

Why it matters:

- Higher resolution reduces quantization noise.
- Floating-point processing gives more internal headroom.
- Final output may still need to fit a device or file format.

Design implication:

The synth should maintain internal headroom and avoid assuming that all signals must stay within a final output range until the final output stage.

## Quantization Noise

Quantization noise is error introduced when a continuous or high-resolution value is rounded to a lower-resolution representation.

Why it matters:

- It can create low-level noise or distortion.
- It is most relevant at final conversion or when using low-resolution control data.

Musical importance:

Most modern internal synth processing can avoid obvious quantization noise, but low-resolution control changes can create audible stepping unless smoothed.

## Buffer

A buffer is a block of consecutive samples processed together.

Why it matters:

- Audio systems usually ask a program to fill buffers.
- Smaller buffers reduce latency but increase deadline pressure.
- Larger buffers are easier for the CPU but feel less responsive.

Musical importance:

Buffer size affects playability. A synth with high latency feels disconnected from the performer even if the sound is good.

## Latency

Latency is the delay between a musical action and the resulting sound.

Sources of latency:

- Input device scanning.
- Event delivery.
- Audio buffer size.
- Operating-system scheduling.
- Audio hardware.
- Plugin host buffering.
- Lookahead processing.

Why it matters:

- Performers are sensitive to latency.
- Tight rhythmic playing requires low latency.
- Excess latency makes instruments feel slow or detached.

Design implication:

The synth should avoid unnecessary buffering and should not rely on slow processing inside the audio path.

## Audio Callback

An audio callback is the recurring real-time request to produce the next block of audio.

Why it matters:

- It is the deadline-driven heart of a real-time synthesizer.
- If the callback takes too long, the listener hears a glitch.
- The callback should avoid unpredictable work.

Unpredictable work includes:

- Waiting for locks.
- File access.
- Network access.
- Large memory allocation.
- Preset parsing.
- Expensive logging.
- Complex user-interface updates.

Design implication:

Even before choosing a stack, the architecture should separate audio rendering from editing, browsing, saving, loading, and visualization.

## Control Rate Versus Audio Rate

Audio-rate processing updates every sample. Control-rate processing updates less often, usually once per block or at scheduled control points.

Why it matters:

- Audio-rate modulation is more accurate and can create audio-band effects.
- Control-rate modulation is cheaper but can sound stepped if not smoothed.

Examples:

- An oscillator output is audio rate.
- A slow LFO can often be control rate.
- FM modulation may need audio rate.
- Smooth cutoff sweeps may need either audio-rate modulation or careful smoothing.

Design implication:

The modulation system should distinguish audio-rate, block-rate, and event-rate behavior conceptually.

## Parameter Smoothing

Parameter smoothing gradually changes a parameter instead of jumping instantly.

Why it matters:

- Sudden gain changes cause clicks.
- Sudden cutoff changes can cause zipper noise.
- Sudden pitch changes may be desired for some effects but not for normal knob motion.

Musical importance:

Smoothing makes controls feel polished. Without it, a synth can sound broken even if the underlying algorithms are correct.

Design implication:

Every exposed parameter should have an intentional smoothing policy. Some parameters need fast response, some need slow smoothing, and some should be allowed to jump.

## Zipper Noise

Zipper noise is audible stepping caused by coarse or unsmoothed parameter changes.

Common causes:

- Low-resolution MIDI controller data.
- Updating parameters once per block without interpolation.
- Moving filter cutoff abruptly.
- Changing gain abruptly.

Design implication:

Controls that affect audible tone should be smoothed or interpolated unless a stepped effect is intended.

## Phase

Phase describes position within a repeating waveform cycle.

Why it matters:

- Oscillator phase affects how waveforms combine.
- Phase reset affects the attack of repeated notes.
- Stereo phase relationships affect width and mono compatibility.
- FM and sync behavior depend on phase relationships.

Musical importance:

Phase can make a sound punchy, hollow, wide, narrow, stable, or unstable. Unison sounds depend heavily on phase behavior.

## Frequency And Pitch

Frequency is measured in hertz and describes cycles per second. Pitch is the musical perception of frequency.

Why it matters:

- Oscillators need precise frequency control.
- Filters use frequency for cutoff.
- LFOs use frequency for modulation rate.
- Delay and comb effects can create pitch-like resonances.

Musical importance:

Musicians often think in notes, octaves, semitones, cents, and scale degrees, not raw hertz. The synth should allow musical control while remaining mathematically accurate.

## Amplitude And Loudness

Amplitude is signal level. Loudness is perceived volume.

Why it matters:

- Doubling amplitude does not sound like doubling loudness.
- Human hearing is frequency-dependent.
- Mixing multiple voices increases total level.

Design implication:

Important gain controls should use perceptually meaningful scaling. Internal gain staging should leave headroom for polyphony, unison, resonance, and effects.

## Decibels

Decibels are a logarithmic way to describe level ratios.

Why it matters:

- Audio levels are usually easier to reason about in decibels than raw linear values.
- Small decibel changes can be musically meaningful.
- Large gain ranges are easier to control logarithmically.

Musical importance:

Decibel-based thinking helps avoid controls where most of the useful range is squeezed into a tiny part of a knob.

## Headroom

Headroom is level margin before clipping or overload.

Why it matters:

- Polyphony increases summed level.
- Resonant filters can boost level.
- Effects can increase peaks.
- Unison can create large combined peaks.

Design implication:

The synth should define a gain-staging philosophy early. It should not rely on a final limiter to hide poor internal level management.

## Clipping

Clipping occurs when a signal exceeds the allowed range and is flattened or limited.

Why it matters:

- Accidental clipping is usually unpleasant.
- Intentional clipping can be useful as distortion.

Design implication:

The architecture should distinguish accidental overload from intentional saturation or drive.

## Dynamic Range

Dynamic range is the difference between the quietest and loudest useful signal levels.

Why it matters:

- Expressive instruments need room for quiet and loud playing.
- Compression and limiting reduce dynamic range.
- Velocity sensitivity depends on dynamic range.

Musical importance:

A synth with poor dynamic range feels flat. A synth with too much uncontrolled dynamic range may be difficult to mix.

## Real-Time Safety

Real-time safety means audio processing behaves predictably enough to meet deadlines.

Why it matters:

- A synth can be correct in theory but unusable if it glitches.
- Real-time systems prefer predictable work over occasionally faster but unreliable work.

Design implication:

The project should document which behavior belongs in the real-time path and which behavior belongs outside it. This boundary should survive future stack decisions.

