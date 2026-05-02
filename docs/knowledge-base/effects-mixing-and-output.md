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

