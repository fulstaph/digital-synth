# Modulation And Envelopes

Modulation is the process of changing one parameter with another signal or control source. It is the difference between a static tone and a living instrument.

## Modulation

Modulation means using one value to change another value over time.

Examples:

- An envelope changes volume.
- An LFO changes pitch.
- Velocity changes filter cutoff.
- A macro changes filter cutoff, reverb mix, and oscillator detune at once.
- Pressure changes vibrato depth.

Why it matters:

Most interesting synthesizer sounds depend on movement. Modulation creates articulation, rhythm, expression, instability, and evolution.

## Modulation Source

A modulation source is anything that produces a changing or controllable value.

Common sources:

- Envelope.
- LFO.
- Velocity.
- Key tracking.
- Mod wheel.
- Aftertouch.
- Pitch bend.
- Random value.
- Step sequencer.
- Macro.
- Per-note expression.
- Audio-rate oscillator.

Why it matters:

The quality of a modulation system depends on the usefulness and clarity of its sources.

## Modulation Destination

A modulation destination is a parameter affected by a modulation source.

Common destinations:

- Pitch.
- Oscillator level.
- Oscillator shape.
- Pulse width.
- Filter cutoff.
- Filter resonance.
- Amplifier gain.
- Pan.
- Effect mix.
- Delay time.
- Reverb size.
- Wavetable position.
- FM amount.

Why it matters:

Destinations determine what can move. A synth with many sources but few destinations still feels limited.

## Modulation Depth

Modulation depth is the amount by which a source affects a destination.

Why it matters:

- Small depth creates subtle movement.
- Large depth creates obvious transformation.
- Negative depth can invert a source.

Design implication:

Depth should use destination-aware units where possible. Pitch modulation may be in cents or semitones. Filter modulation may be in octaves or normalized cutoff movement. Gain modulation may need perceptual scaling.

## Modulation Matrix

A modulation matrix is a routing system that connects sources to destinations.

Why it matters:

- It lets users create complex patches without hard-wired limitations.
- It makes the synth extensible.
- It enables expressive performance mappings.

Good modulation matrix behavior:

- Clear source names.
- Clear destination names.
- Visible depth amounts.
- Bipolar and unipolar handling.
- Per-voice and global routing distinction.
- Conflict behavior when many sources affect one destination.
- Visual indication of active modulation.

Design implication:

Digital Synth should make the modulation matrix central rather than hidden. It is one of the main ways the instrument becomes a sound-design laboratory.

## Unipolar And Bipolar Modulation

A unipolar source moves in one direction from a minimum to a maximum. A bipolar source moves around a center point in both directions.

Examples:

- An ADSR envelope is usually unipolar.
- A sine LFO is usually bipolar.
- Velocity is usually unipolar.
- Pitch bend is usually bipolar.

Why it matters:

If the synth treats these carelessly, modulation depth will feel confusing. A unipolar envelope opening a filter is different from a bipolar LFO moving cutoff above and below its base value.

Design implication:

The docs and future interface should show whether a source is unipolar or bipolar.

## Per-Voice Modulation

Per-voice modulation is evaluated separately for each active note.

Examples:

- Each note has its own amplitude envelope.
- Each note has its own filter envelope.
- Each note has independent pressure.
- Each note has an LFO that starts when the note starts.

Why it matters:

Per-voice modulation lets chords articulate naturally. One note can be in attack while another is in release.

## Global Modulation

Global modulation affects the entire instrument at once.

Examples:

- A global LFO sweeps filter cutoff for all voices together.
- A macro raises reverb mix.
- A master volume control changes the whole output.

Why it matters:

Global modulation creates unified movement. It is useful for tempo-synced sweeps, performance macros, and final shaping.

Design implication:

The architecture should support both per-voice and global modulation because they produce different musical results.

## Envelope

An envelope is a time-shaped modulation source, usually triggered by a note or event.

Why it matters:

Envelopes define articulation. They determine whether a sound is percussive, sustained, swelling, fading, plucked, bowed, struck, or droning.

## ADSR Envelope

ADSR stands for attack, decay, sustain, and release.

Attack:

The time it takes to rise from silence or the current level to the peak.

Decay:

The time it takes to fall from the peak to the sustain level.

Sustain:

The level held while the note remains active.

Release:

The time it takes to fade after the note is released.

Why it matters:

ADSR is one of the most important synthesizer concepts because it maps naturally to musical articulation.

Sound examples:

- Short attack, short decay, low sustain, short release: pluck.
- Medium attack, high sustain, medium release: pad.
- Instant attack, full sustain, no release: organ-like tone.
- Fast attack, decay to medium sustain, short release: bass.

Design implication:

The synth should include ADSR behavior early and later consider curve shapes, delay, hold, looping, and velocity scaling.

## Envelope Curves

Envelope curves describe the shape of transitions between stages.

Common curve types:

- Linear.
- Exponential.
- Logarithmic.
- S-curve.

Why it matters:

The same attack time can feel very different depending on curve shape. Exponential curves often sound more natural for amplitude.

Design implication:

Curve shape may be a future advanced control. Even if hidden, the default curve should be chosen musically.

## Retrigger

Retrigger describes whether an envelope restarts when a new note is played.

Why it matters:

- In polyphonic mode, each new voice usually starts its envelopes.
- In monophonic legato mode, envelopes may or may not restart.

Musical importance:

Retriggered envelopes create clear attacks. Non-retriggered legato creates smooth connected lines.

Design implication:

Retrigger behavior must be defined for mono, legato, and repeated notes.

## LFO

An LFO is a low-frequency oscillator used as a modulation source.

Common LFO shapes:

- Sine.
- Triangle.
- Saw up.
- Saw down.
- Square.
- Random.
- Sample and hold.

Common destinations:

- Pitch.
- Filter cutoff.
- Amplifier gain.
- Pan.
- Pulse width.
- Wavetable position.
- Effects.

Why it matters:

LFOs provide repeating or random movement. They are central to vibrato, tremolo, wah, rhythmic gating, stereo movement, and evolving timbres.

## LFO Rate

LFO rate controls how fast the modulation repeats.

Modes:

- Free rate, measured as frequency.
- Tempo-synced rate, measured in rhythmic divisions.

Why it matters:

Free LFOs feel organic. Tempo-synced LFOs support rhythmic electronic music.

Design implication:

The synth should eventually support both free and synced concepts, even if only one appears in an initial version.

## LFO Phase

LFO phase controls where in the LFO cycle the modulation begins.

Why it matters:

- Phase reset can make repeated notes consistent.
- Free-running phase can make motion continuous.
- Phase offset can create stereo or multi-voice movement.

Design implication:

LFOs should define whether they are per-voice reset, per-voice free-running, or global.

## Vibrato

Vibrato is periodic pitch modulation.

Why it matters:

- Adds expressiveness.
- Commonly controlled by mod wheel, aftertouch, or delayed LFO fade-in.

Design implication:

Vibrato depth should be musically scaled. A small amount can sound expressive; too much can sound unstable or comedic.

## Tremolo

Tremolo is periodic amplitude modulation.

Why it matters:

- Creates pulsing volume movement.
- Can be subtle or rhythmic.

Design implication:

Tremolo should be distinguished from vibrato. Many beginners confuse the terms.

## Random Modulation

Random modulation introduces unpredictable variation.

Types:

- Per-note random value.
- Smooth random drift.
- Sample-and-hold stepped random.
- Noise-based modulation.

Why it matters:

Randomness can make repeated notes feel human, analog, or unstable.

Design implication:

Random modulation should be bounded and intentional. Too much randomness makes patches unreliable.

## Step Sequencer Modulation

A step sequencer modulation source outputs a sequence of values over time.

Why it matters:

- Creates rhythmic parameter changes.
- Supports patterns, grooves, and evolving motion.

Design implication:

Step modulation should eventually integrate with tempo concepts, smoothing, probability, and retrigger behavior.

## Macro Controls

A macro is a high-level control mapped to one or more parameters.

Why it matters:

- Makes complex patches playable.
- Gives users meaningful performance gestures.
- Helps preset designers expose the important dimensions of a sound.

Examples:

- Brightness macro opens filter cutoff, increases resonance slightly, and raises oscillator brightness.
- Space macro increases reverb mix and delay feedback.
- Intensity macro increases drive, filter envelope amount, and modulation depth.

Design implication:

Macros should be part of the preset design model, not an afterthought.

## Modulation Design Recommendations

For the first architecture:

- Include at least one amplitude envelope.
- Include at least one routable envelope.
- Include at least one LFO.
- Include velocity and key tracking as modulation sources.
- Define unipolar and bipolar behavior.
- Distinguish per-voice and global modulation.
- Make modulation visible and inspectable.
- Add macro controls early in the product design, even if implementation comes later.

