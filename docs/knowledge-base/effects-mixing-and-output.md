# Effects, Mixing, And Output

Effects, mixing, and output processing turn raw synthesis into a finished instrument sound. They also create many opportunities for gain problems, latency, and hidden complexity.

## Mixer

A mixer combines multiple signals.

In a synth, mixing happens at several levels:

- Oscillators mixed inside a voice.
- Noise mixed with pitched sources.
- Voices mixed into the instrument output.
- Dry signal mixed with effects.
- Final stereo output mixed to the device or host.

Why it matters:

Mixing determines balance, headroom, and how much each source contributes to the final timbre.

Design implication:

The project should define gain staging before adding many sources or effects.

## Gain Staging

Gain staging is the management of signal level throughout the signal path.

Why it matters:

- Filters and distortion respond differently at different input levels.
- Multiple voices increase total level.
- Effects can clip internally.
- Final output should avoid accidental clipping.

Musical importance:

Good gain staging makes patches feel consistent and professional. Bad gain staging makes presets jump in volume or distort unintentionally.

## Panning

Panning places a sound between left and right speakers.

Why it matters:

- Creates stereo placement.
- Helps separate layers.
- Supports unison spread and movement.

Design implication:

Panning can be per source, per voice, per effect, or global. The architecture should define where stereo width is introduced.

## Stereo Width

Stereo width is the perceived spread between left and right channels.

Sources of width:

- Panned voices.
- Unison spread.
- Chorus.
- Delay differences.
- Reverb.
- Phase differences.

Why it matters:

Width makes sounds feel large, but excessive phase differences can cause problems in mono.

Design implication:

Width should be controllable and musically safe. A mono-compatible option is useful.

## Distortion And Saturation

Distortion changes the waveform nonlinearly. Saturation is often a smoother, more gradual form of distortion.

Why it matters:

- Adds harmonics.
- Increases perceived loudness.
- Creates warmth, grit, or aggression.

Design implication:

Distortion should include level compensation where possible. It may need oversampling to reduce aliasing.

## Chorus

Chorus creates the impression of multiple similar sources by using short modulated delays.

Why it matters:

- Adds width.
- Adds motion.
- Thickens pads, keys, basses, and leads.

Design implication:

Chorus should be gain-managed and should avoid making low frequencies unfocused unless that is desired.

## Flanger

Flanging uses a very short modulated delay with feedback, creating moving comb-filter notches.

Why it matters:

- Creates sweeping, metallic, jet-like movement.

Design implication:

Feedback and delay range should be carefully controlled to avoid harsh or unstable results.

## Phaser

A phaser uses phase-shifting stages to create moving notches in the spectrum.

Why it matters:

- Creates swirling motion.
- Often sounds smoother than flanging.

Design implication:

Phaser rate, depth, feedback, and mix should be musically scaled.

## Delay

Delay repeats sound after a time interval.

Common controls:

- Time.
- Feedback.
- Mix.
- Filtering.
- Stereo offset.
- Sync to tempo.

Why it matters:

Delay creates echo, rhythm, space, and density.

Design implication:

Delay time changes can click or pitch-shift depending on design. Feedback can run away if not controlled.

## Reverb

Reverb simulates or creates the impression of acoustic space.

Common controls:

- Size.
- Decay.
- Pre-delay.
- Damping.
- Diffusion.
- Mix.

Why it matters:

Reverb places sounds in space and can transform dry synth tones into pads, atmospheres, and cinematic textures.

Design implication:

Reverb can be CPU-heavy and can blur articulation. It should have clear wet/dry balance and useful defaults.

## Equalization

EQ adjusts frequency balance.

Common uses:

- Remove low-end rumble.
- Reduce harshness.
- Add presence.
- Shape final tone.

Design implication:

EQ can be a final polish tool, but it should not compensate for poorly designed oscillators, filters, or gain staging.

## Compression

Compression reduces dynamic range when signal level exceeds a threshold.

Why it matters:

- Controls peaks.
- Adds sustain.
- Makes patches more consistent.
- Can add punch.

Design implication:

Compression should be optional and understandable. Over-compression can remove performance expression.

## Limiting

Limiting is a strong form of dynamics control that prevents peaks from exceeding a ceiling.

Why it matters:

- Protects against excessive output.
- Helps prevent clipping.

Design implication:

A limiter can be useful as a safety layer, but it should not hide bad internal gain staging.

## Effect Order

Effect order changes sound dramatically.

Examples:

- Distortion before delay repeats the distorted tone.
- Delay before distortion distorts the echoes together.
- Chorus before reverb widens the sound before placing it in space.
- Reverb before compression can make ambience more obvious.

Design implication:

The first architecture should define a simple fixed effect order. Flexible routing can come later.

## Send Versus Insert Effects

An insert effect processes the whole signal passing through it. A send effect receives a copy of the signal and mixes its output back in.

Why it matters:

- Reverb and delay often work well as sends.
- Distortion and EQ often work well as inserts.

Design implication:

The first synth can use a simple insert chain, but future architecture should allow send-style thinking for ambience.

## Sidechain Concepts

Sidechaining uses one signal's amplitude to control processing applied to a different signal. The control signal, called the sidechain input, is analyzed for its level. That level then modulates a parameter of a processor that acts on the main signal.

The most common application is sidechain compression. A rhythmic source such as a kick drum feeds the sidechain input of a compressor placed on a bass or pad. Each time the kick hits, the compressor reduces the level of the bass, creating a pumping rhythm that clears space in the mix and adds rhythmic movement.

Why it matters:

Sidechain behavior creates rhythmic interaction between sounds. It produces ducking effects that improve mix clarity and add groove. Without sidechaining, sustained sounds compete with transient sounds for the same frequency space, causing masking.

In a synthesizer context, internal sidechaining can use one voice layer to duck another, or use the amplitude envelope of one oscillator to shape how an effect behaves. A sub oscillator could duck slightly when the main oscillator is loud, or an effects processor could open up based on envelope activity from another layer.

Controls typically exposed include the sidechain source selector, threshold, ratio, attack time, release time, and the amount of gain reduction applied.

Design implication:

Sidechain should be designed as a routing option rather than a fixed behavior. Treating it as a modulation source routed to a dynamics processor allows creative use beyond basic ducking, including rhythmic gating, envelope following, and frequency-dependent ducking.

## Multiband Processing

Multiband processing splits a signal into separate frequency bands and processes each band independently. The signal is divided by crossover filters into typically two to four regions such as low, mid, and high. Each band then passes through its own processor before the bands are recombined.

Why it matters:

Full-spectrum processing forces compromises. Heavy distortion across the entire signal muddies the bass. Broadband compression causes the bass to pump when the high end is loud. Multiband processing avoids these problems by isolating frequency regions.

With multiband processing, distortion can be applied aggressively to the midrange while keeping the low end clean. Compression can tighten the high frequencies without causing low-frequency pumping. Different saturation characters can color different parts of the spectrum independently.

Controls typically include crossover frequencies that define band boundaries, and per-band controls for whatever processor is applied. Gain controls per band allow rebalancing after processing.

Mistakes to avoid include setting crossover points in musically sensitive regions where they create audible artifacts, and over-processing individual bands so the recombined signal sounds disjointed rather than cohesive.

Design implication:

Multiband processing adds significant complexity and CPU cost because each band requires its own processing chain. It belongs in the effects chain as an advanced option rather than a default. The initial architecture should support it as a future addition without requiring restructuring.

## Transient Shaping

Transient shaping emphasizes or reduces the attack portion of a signal independently from the sustain. A transient shaper detects the onset of a sound, the brief moment when the waveform rises sharply, and either boosts or attenuates the level during that attack window while leaving the sustained portion unchanged.

Why it matters:

Transient shaping controls the punch and presence of a sound without changing its overall level or sustain character. Boosted transients make plucks, percussion, and key sounds more defined and forward in the mix. Reduced transients make sounds softer and more ambient without reducing volume, which is useful for pushing sounds further back in a mix or creating pad-like textures from percussive sources.

Unlike compression, which reacts to level and reshapes the entire dynamic envelope, transient shaping targets only the attack-to-sustain relationship. This makes it a more surgical tool for controlling how a sound feels in time.

Controls typically include an attack knob that boosts or cuts the transient, a sustain knob that boosts or cuts the body of the sound, and sometimes a detection sensitivity control.

Design implication:

Transient shaping is a useful addition to the effects chain, especially for percussive patches, plucks, and any sound where attack definition matters. It is relatively low in CPU cost compared to multiband processing and adds meaningful tonal control with simple parameters.

## Parallel Processing

Parallel processing blends the original dry signal with a heavily processed wet version at a controlled ratio, rather than passing the full signal through the processor. The dry path preserves the character and dynamics of the original. The wet path contributes qualities from extreme processing that would be unmusical if applied directly to the entire signal.

Common applications include parallel compression, where a heavily compressed copy is mixed with the uncompressed original to add density and sustain without destroying transients. Parallel saturation adds harmonic richness without flattening the dynamics of the source. Parallel reverb adds spatial depth without washing out the direct signal.

Why it matters:

Parallel processing retains the natural character of the original signal while gaining the tonal and dynamic qualities of aggressive processing. Direct heavy compression kills dynamics. Direct heavy distortion destroys clarity. Blending those processed signals in parallel preserves what matters about the original while adding what the processing contributes.

The distinction between parallel processing and a simple wet/dry mix is subtle but important. A wet/dry mix on an insert effect reduces the processed signal's contribution. Dedicated parallel routing allows separate gain staging, EQ, and further processing on the parallel path before blending.

Design implication:

Parallel processing is often achieved through wet/dry mix controls on individual effects, but dedicated parallel routing provides more flexibility. The architecture should allow effects to operate in parallel paths where needed, not only in a serial insert chain.

## Per-Voice Versus Global Effects

Some effects make musical sense applied to each voice independently. Per-voice distortion gives each note its own harmonic color based on that note's pitch and level. Per-voice filtering shapes each note's spectrum individually, so low notes and high notes can respond differently.

Other effects make sense applied to the combined output of all voices. Reverb, delay, and chorus typically work better as global effects because they create a shared spatial environment. Applying separate reverb to each voice would sound fragmented rather than cohesive, and the CPU cost would be extreme.

Why it matters:

Per-voice effects produce different musical results than global effects. Distortion on individual voices before mixing sounds different from the same distortion applied to the summed mix of all voices. Individual voice distortion preserves intermodulation separation between notes. Global distortion introduces intermodulation between all sounding notes, which can be desirable or undesirable depending on the musical context.

Per-voice effects are more expensive because processing multiplies by voice count. Eight voices with per-voice distortion require eight distortion instances rather than one.

Controls should clearly indicate whether an effect operates per-voice or globally, so users understand both the sonic and performance implications of their choices.

Design implication:

The architecture should distinguish per-voice processing from global processing at a structural level. Initial effects can all be global to keep CPU cost predictable. Per-voice effects can be introduced as an advanced feature once the voice architecture is stable and performance budgets are understood.

## Wet And Dry Mix

Dry signal is the original signal. Wet signal is the processed effect signal.

Why it matters:

Wet/dry balance controls how obvious an effect is.

Design implication:

Wet/dry controls should be clear and, for some effects, equal-power or perceptually balanced.

## Master Output

The master output is the final signal delivered to the audio system or host.

Responsibilities:

- Final gain.
- Optional metering.
- Optional limiting.
- Final stereo output.

Design implication:

The master output should provide safety and visibility without changing patch character unpredictably.

## Metering

Metering displays signal level.

Useful meters:

- Peak meter.
- RMS or loudness-style meter.
- Voice activity meter.
- Clipping indicator.

Why it matters:

Meters help users understand level and avoid accidental clipping.

Design implication:

Metering should observe signal without interfering with real-time audio stability.

## Effects Roadmap Recommendation

First effects:

- Simple saturation or drive.
- Chorus.
- Delay.
- Reverb.
- Output level and metering.

Later effects:

- Phaser.
- Flanger.
- EQ.
- Compressor.
- Limiter.
- Advanced distortion.
- Per-voice effects.

The first effects should support common musical use without distracting from the core voice architecture.

