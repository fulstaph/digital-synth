# Filters And Tone Shaping

Filters and tone-shaping stages sculpt the spectrum and character of a synthesizer sound. In subtractive synthesis, they are central to the instrument.

## Filter

A filter changes the level of different frequency ranges.

Why it matters:

- It shapes brightness, warmth, thinness, weight, and motion.
- It turns harmonically rich oscillators into musically articulated sounds.
- It is one of the most important modulation destinations.

Musical importance:

A static oscillator can sound plain. A moving filter can make the same oscillator feel like a pluck, swell, brass stab, bass, pad, or rhythmic pulse.

## Cutoff Frequency

Cutoff frequency is the main frequency boundary where the filter begins changing the signal.

Why it matters:

- It is the primary tone control for many filter types.
- Moving cutoff changes perceived brightness.
- Cutoff is often controlled by envelopes, LFOs, velocity, key tracking, and macros.

Design implication:

Cutoff should be smoothed when controlled continuously. Abrupt cutoff changes often create zipper noise or clicks.

## Resonance

Resonance emphasizes frequencies near the cutoff.

Why it matters:

- Adds character and focus.
- Makes filter movement more audible.
- Can create whistle-like tones.
- Some filters self-oscillate at high resonance.

Musical importance:

Low resonance is smooth. Moderate resonance adds bite. High resonance can become acidic, nasal, or screaming.

Design implication:

Resonance should have musically useful scaling and safe gain behavior. High resonance can create level spikes.

## Filter Slope

Filter slope describes how steeply a filter attenuates frequencies past the cutoff. It is often described in decibels per octave.

Common slopes:

- Gentle slopes preserve more surrounding tone.
- Steeper slopes create stronger separation.

Why it matters:

Slope strongly changes character. A gentle low-pass can sound open and natural. A steep low-pass can sound more electronic and controlled.

Design implication:

If multiple slopes are supported, they should be exposed as musical choices rather than hidden technical variants.

## Low-Pass Filter

A low-pass filter keeps low frequencies and reduces high frequencies.

Sound character:

- Darkens sound as cutoff moves down.
- Smooths bright oscillators.
- Common in subtractive synthesis.

Common uses:

- Bass shaping.
- Pad sweeps.
- Pluck articulation.
- Brass-like attacks.
- Removing harshness.

Design implication:

Low-pass filtering should be part of the first subtractive architecture.

## High-Pass Filter

A high-pass filter keeps high frequencies and reduces low frequencies.

Sound character:

- Thins sound.
- Removes rumble.
- Creates airy or brittle tones.
- Can make space in a mix.

Common uses:

- Thin pads.
- Noise effects.
- Risers.
- Removing low-end buildup.
- Layering.

Design implication:

High-pass filtering is important for mix control and sound design, though low-pass is usually the first subtractive priority.

## Band-Pass Filter

A band-pass filter keeps a range of frequencies and reduces frequencies above and below it.

Sound character:

- Focused.
- Vocal.
- Nasal.
- Resonant.

Common uses:

- Formant-like sounds.
- Sweeps.
- Percussion.
- Effects.
- Thin leads.

Design implication:

Band-pass mode helps a synth create specialized tones without adding new oscillators.

## Notch Filter

A notch filter removes a narrow band of frequencies.

Sound character:

- Hollow.
- Phasey.
- Moving notches can sound like phasing.

Common uses:

- Special effects.
- Removing resonances.
- Animated motion.

Design implication:

Notch filtering is useful but not required for the earliest version.

## Comb Filter

A comb filter creates repeated peaks and notches across the spectrum, usually through short delay and feedback.

Sound character:

- Metallic.
- Hollow.
- Flanged.
- Resonant.
- String-like in some settings.

Common uses:

- Physical-modeling-like tones.
- Flanging.
- Resonator effects.
- Metallic percussion.

Design implication:

Comb filtering belongs in advanced tone shaping or effects. It can become unstable with feedback and needs careful gain handling.

## State-Variable And Multimode Filters

A multimode filter can provide several outputs or modes such as low-pass, high-pass, band-pass, and notch.

Why it matters:

- Gives more sound-design range from one conceptual filter.
- Makes filter mode a performance or preset choice.

Design implication:

A multimode filter is a good long-term design target, but the conceptual model should start with clear individual filter behaviors.

## Key Tracking

Key tracking changes filter cutoff based on note pitch.

Why it matters:

- Higher notes often need higher cutoff to remain bright.
- It can make filter resonance track pitch.
- Full key tracking can make a self-oscillating filter play melodies.

Musical importance:

Without key tracking, a patch may sound bright in the low register but dull in the high register, or the reverse.

Design implication:

Key tracking should be a standard modulation source or dedicated filter control.

## Velocity To Filter

Velocity-to-filter mapping changes cutoff or envelope amount based on how hard a note is played.

Why it matters:

- Adds articulation.
- Mimics acoustic instruments where harder playing often produces brighter sound.
- Makes patches feel responsive.

Design implication:

Velocity should be routable to filter cutoff and filter envelope amount.

## Filter Envelope

A filter envelope moves cutoff or another filter parameter over the life of a note.

Why it matters:

- Creates plucks.
- Creates brass-like openings.
- Creates percussive brightness.
- Shapes attacks and releases.

Design implication:

The first subtractive architecture should include a dedicated filter envelope or a general envelope that can be routed to cutoff.

## Drive And Saturation

Drive increases signal level into a nonlinear stage. Saturation softens or colors peaks by adding harmonics.

Why it matters:

- Adds warmth, grit, density, or aggression.
- Can make filters sound more characterful.
- Changes perceived loudness and brightness.

Design implication:

Drive should be intentional and gain-managed. If placed before the filter, it changes what the filter receives. If placed after the filter, it colors the filtered result.

## Waveshaping

Waveshaping uses a nonlinear curve to transform the waveform.

Why it matters:

- Creates harmonics and complex timbres.
- Can produce distortion, folding, clipping, or asymmetry.

Design implication:

Waveshaping can alias strongly, so quality modes or oversampling should be considered when implementation begins.

## Wavefolding

Wavefolding folds a waveform back on itself when it exceeds a threshold.

Sound character:

- Bright.
- Complex.
- Metallic.
- Aggressive.
- Often associated with West Coast synthesis.

Design implication:

Wavefolding is powerful but should be introduced after the basic subtractive path is stable.

## Equalization

EQ changes frequency balance using bands rather than acting as the primary musical filter.

Why it matters:

- Helps fit patches into a mix.
- Can correct excessive low-end, harshness, or muddiness.

Design implication:

EQ belongs more naturally in the effects or output stage than in the core voice, though both options are possible.

## Common Filter Mistakes

Mistake: Cutoff jumps without smoothing.

Result: Zipper noise or clicks.

Mistake: Resonance causes uncontrolled level spikes.

Result: Harshness, clipping, or listener discomfort.

Mistake: Filter envelope range is poorly scaled.

Result: Most useful sounds occur in a tiny control range.

Mistake: No key tracking.

Result: Patches do not translate well across the keyboard.

Mistake: Filter is treated as a utility rather than a musical control.

Result: The instrument feels static.

## Filter Design Recommendations

For the first architecture:

- Include a low-pass filter as a core component.
- Add cutoff, resonance, envelope amount, key tracking, and velocity influence.
- Design modulation routing so LFOs, envelopes, macros, and performance controls can target cutoff.
- Preserve headroom around resonance.
- Add additional modes after the basic low-pass behavior is clear.

