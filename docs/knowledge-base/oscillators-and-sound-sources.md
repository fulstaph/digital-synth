# Oscillators And Sound Sources

Oscillators and sound sources provide the raw material of synthesis. Filters, envelopes, effects, and modulation can shape that material, but the source determines the starting spectrum and behavior.

## Oscillator

An oscillator is a signal generator that produces a repeating waveform.

Why it matters:

- It defines the basic pitch and harmonic content of a sound.
- It is usually the first audible part of a subtractive synthesizer voice.
- Its quality affects the entire instrument.

Musical importance:

A good oscillator should feel stable, tunable, and clean when needed, but it should also support character through detune, phase behavior, sync, drift, and modulation.

## Waveform

A waveform is the shape of one cycle of an oscillator.

Why it matters:

- The waveform shape determines harmonic content.
- Different waveforms suggest different musical roles.
- Modulating waveform shape can create motion.

## Sine Wave

A sine wave contains only one frequency component: the fundamental.

Sound character:

- Pure.
- Smooth.
- Hollow if isolated.
- Useful for sub bass and test tones.

Importance:

- Foundation of additive synthesis.
- Foundation of many FM sounds.
- Useful as a clean sub-oscillator.
- Useful for LFO shapes.

Design implication:

Sine waves should be accurate and free from unnecessary harmonics. They are also useful as a reference for tuning and level behavior.

## Triangle Wave

A triangle wave contains mostly odd harmonics that decrease quickly in level.

Sound character:

- Soft.
- Rounded.
- More harmonic than sine.
- Less bright than square or saw.

Importance:

- Good for mellow leads and basses.
- Useful when a sine is too plain but a square is too strong.
- Common in classic analog-style synthesis.

Design implication:

Triangle waves are useful for musical variety and should be included early.

## Sawtooth Wave

A sawtooth wave contains both even and odd harmonics.

Sound character:

- Bright.
- Buzzing.
- Dense.
- Classic synthesizer tone.

Importance:

- Core source for subtractive basses, leads, brass, pads, and strings.
- Excellent for filtering because it contains many harmonics to remove or emphasize.

Design implication:

Sawtooth oscillators must be band-limited. A naive sawtooth aliases strongly, especially at high notes.

## Square Wave

A square wave contains odd harmonics.

Sound character:

- Hollow.
- Reedy.
- Strong.
- Focused.

Importance:

- Useful for basses, leads, chiptune-like tones, clarinet-like sounds, and pulse-width modulation variants.

Design implication:

Square waves also need band-limiting. They have sharp transitions that create many high harmonics.

## Pulse Wave

A pulse wave is like a square wave with adjustable duty cycle. The duty cycle describes how much of the cycle is high versus low.

Sound character:

- Hollow at 50 percent duty cycle.
- Thinner as the pulse narrows.
- Animated when pulse width is modulated.

Importance:

- Pulse-width modulation is one of the classic ways to create motion from a simple oscillator.
- Narrow pulses can create nasal, biting, or reedy tones.

Design implication:

Pulse width should have safe limits. Extremely narrow pulses can cause level changes, aliasing, and unstable-sounding results.

## Noise

Noise is a random or pseudo-random signal rather than a pitched periodic waveform.

Types:

- White noise has equal energy per hertz.
- Pink noise has more low-frequency energy and sounds smoother.
- Brown noise has even stronger low-frequency emphasis.
- Sample-and-hold noise changes at discrete intervals and is often used as a stepped modulation source.

Sound character:

- Air.
- Hiss.
- Breath.
- Impact.
- Texture.
- Randomness.

Importance:

- Essential for drums, wind, breath, risers, sweeps, and attack transients.
- Useful as both audio and modulation source.

Design implication:

Noise should be level-managed carefully because it can dominate a mix. Filtered noise is often more musically useful than raw noise.

## Sub-Oscillator

A sub-oscillator produces a pitch below the main oscillator, usually one or two octaves lower.

Why it matters:

- Adds weight to basses and leads.
- Can reinforce the fundamental.
- Helps a patch remain powerful on small speakers if balanced carefully.

Design implication:

Sub-oscillators should avoid uncontrolled low-frequency buildup. They should have clear level control.

## Detune

Detune is a small pitch offset between oscillators or voices.

Why it matters:

- Creates beating and width.
- Makes static tones feel alive.
- Supports unison, chorus-like effects, and analog-style instability.

Musical importance:

Small detune can sound warm. Large detune can sound wide, aggressive, or out of tune.

Design implication:

Detune should be scaled musically. Fine detune is usually measured in cents. Coarse tuning is usually measured in semitones or octaves.

## Unison

Unison means several oscillators or voice copies play the same note with slight differences in pitch, phase, pan, or tone.

Why it matters:

- Creates width and density.
- Common in modern basses, leads, and pads.
- Can quickly increase level and CPU cost.

Design implication:

Unison needs gain compensation, detune control, stereo spread, and phase behavior rules.

## Oscillator Sync

Oscillator sync resets one oscillator's phase using another oscillator.

Why it matters:

- Produces strong, bright, harmonically complex tones.
- Sync sweeps create classic aggressive lead sounds.

Musical importance:

Sync is useful for ripping leads, hard-edged basses, and animated timbres.

Design implication:

Sync can produce extreme high-frequency content and aliasing. It should be treated as a quality-sensitive feature.

## Phase Reset And Free-Running Phase

Phase reset means an oscillator starts each note from a defined phase. Free-running phase means oscillator phase continues independently and each note begins wherever the oscillator happens to be.

Phase reset:

- Produces consistent attacks.
- Useful for drums, basses, and precise transients.
- Can sound static in repeated notes.

Free-running phase:

- Feels more organic.
- Creates variation between notes.
- Can make attacks less predictable.

Design implication:

The synth should eventually expose or define phase behavior per oscillator or per patch.

## Drift

Drift is slow, subtle variation in pitch, phase, or tone.

Why it matters:

- Adds life to otherwise static digital oscillators.
- Helps emulate analog instability when desired.

Design implication:

Drift should be controllable and musically bounded. Too much drift sounds out of tune.

## Wavetable Source

A wavetable source reads from one-cycle waveforms and can move between them.

Why it matters:

- Adds evolving timbres beyond static waveforms.
- Makes modulation visually and musically compelling.

Design implication:

Wavetable position should be a modulation destination. Movement should be smooth unless stepped movement is intentional.

## Sample Source

A sample source plays recorded audio.

Why it matters:

- Captures complex real-world detail.
- Provides attacks, textures, and organic variation.

Design implication:

Sample loading and analysis should be non-real-time. Playback behavior should include root note, looping, start position, and pitch behavior if sample synthesis is added.

## Granular Source

A granular source produces many short grains from a sample or buffer.

Why it matters:

- Creates textures that are difficult with normal oscillators.
- Allows time, pitch, and density to become separate design dimensions.

Design implication:

Granular synthesis needs careful parameter design because grain duration, density, pitch, position, and envelope interact strongly.

## Source Design Recommendations

For the first architecture:

- Include sine, triangle, saw, square, pulse, and noise.
- Include at least two pitched oscillators and one noise source conceptually.
- Include oscillator level, tuning, detune, phase behavior, and waveform selection.
- Treat band-limiting as required, not optional polish.
- Keep future sources modular so wavetable, FM, sample, and granular sources can fit later.

