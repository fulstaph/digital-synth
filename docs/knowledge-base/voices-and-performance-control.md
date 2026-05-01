# Voices And Performance Control

Voices and performance control define how a synthesizer responds to musical input. This is where synthesis becomes an instrument rather than a sound generator.

## Voice

A voice is one active note or sound event inside the synthesizer.

A voice usually contains:

- Pitch state.
- Oscillator states.
- Envelope states.
- Filter state.
- Per-note modulation state.
- Release behavior.
- Voice-level gain.

Why it matters:

If the synth is playing a four-note chord, it is usually rendering four voices at the same time. Each voice may be at a different point in its envelope and may have different performance values.

## Polyphony

Polyphony is the number of notes or voices that can sound at the same time.

Why it matters:

- More polyphony allows chords, long releases, and layered textures.
- More polyphony increases CPU use and output level.
- Polyphony interacts with sustain pedal and release tails.

Design implication:

Voice count should be configurable or at least clearly defined. The architecture should handle voice limits musically.

## Monophonic Mode

Monophonic mode allows only one note at a time.

Why it matters:

Many basses, leads, and classic synth lines rely on monophonic behavior.

Important mono behaviors:

- Last-note priority.
- Low-note priority.
- High-note priority.
- Legato.
- Glide.
- Envelope retrigger options.

Design implication:

Mono mode is not just polyphony set to one. It needs dedicated note-priority and articulation rules.

## Paraphony

Paraphony allows multiple pitches but shares some parts of the signal path, often one filter or amplifier envelope.

Why it matters:

Paraphony has a distinctive behavior. New notes can retrigger a shared envelope and affect already-held notes.

Design implication:

Paraphony is optional and should be documented separately if added. It should not be confused with true polyphony.

## Voice Allocation

Voice allocation decides which internal voice handles a new note.

Why it matters:

Good allocation is invisible. Poor allocation causes cutoffs, clicks, stolen notes, or inconsistent releases.

Common strategies:

- Use an inactive voice.
- Reuse the oldest released voice.
- Steal the quietest voice.
- Steal the oldest voice.
- Prefer not to steal voices still in attack.

Design implication:

The voice allocator should prioritize musical continuity. Released or quiet voices are better stealing targets than prominent held notes.

## Voice Stealing

Voice stealing happens when a new note needs a voice but all voices are already active.

Why it matters:

Stealing is unavoidable when polyphony is limited. The question is how audible and musical it is.

Good stealing behavior:

- Avoids clicks.
- Avoids cutting off loud notes if possible.
- Prefers released notes.
- Applies a tiny fade if needed.
- Behaves predictably.

Design implication:

Voice stealing should be designed, not left as accidental behavior.

## Note On

Note on is the event that starts a note.

Information usually associated with note on:

- Pitch.
- Velocity.
- Timing.
- Channel or note identity.
- Optional expression state.

Why it matters:

Note-on timing and velocity strongly affect articulation.

## Note Off

Note off is the event that releases a note.

Why it matters:

Note off usually starts the release stage of envelopes. It should not usually silence a voice instantly unless the patch or mode requires that behavior.

Design implication:

The synth should distinguish release from immediate stop.

## Velocity

Velocity usually represents how hard or fast a note was played.

Common destinations:

- Amplitude.
- Filter cutoff.
- Filter envelope amount.
- Attack time.
- Sample start or layer.
- Modulation depth.

Why it matters:

Velocity gives performers dynamic and timbral control per note.

Design implication:

Velocity should be treated as a core per-note modulation source.

## Aftertouch And Pressure

Aftertouch or pressure is control applied after a note starts.

Types:

- Channel pressure affects all notes on a channel.
- Polyphonic pressure can affect individual notes.

Common destinations:

- Vibrato depth.
- Filter cutoff.
- Volume.
- Wavetable position.
- Effect intensity.

Why it matters:

Pressure lets a performer shape sustained notes without starting new notes.

Design implication:

The architecture should distinguish global pressure from per-note pressure.

## Pitch Bend

Pitch bend changes pitch continuously.

Why it matters:

- Allows expressive slides.
- Supports guitar-like bends, brass scoops, and electronic pitch gestures.
- Can be global or per-note in expressive systems.

Design implication:

Pitch bend range should be defined per patch or globally. Pitch smoothing and precision matter.

## Mod Wheel

The mod wheel is a common continuous performance control.

Common uses:

- Add vibrato.
- Open filter.
- Increase brightness.
- Increase effect intensity.
- Move a macro.

Why it matters:

The mod wheel is often the main expressive control available on traditional MIDI keyboards.

Design implication:

The mod wheel should be routable rather than hard-wired to one behavior.

## Sustain Pedal

The sustain pedal keeps notes sounding after keys are released.

Why it matters:

Sustain pedal behavior is essential for keyboard performance and can create many overlapping releases.

Design implication:

The voice allocator must understand held notes, released-but-sustained notes, and pedal release behavior.

## Glide And Portamento

Glide or portamento smoothly changes pitch from one note to another.

Why it matters:

It is essential for many bass and lead styles.

Important options:

- Always glide.
- Glide only when notes overlap.
- Fixed time.
- Fixed rate.

Design implication:

Glide belongs especially to mono and legato behavior but can also exist polyphonically in advanced designs.

## Legato

Legato means connected note behavior. In synthesizers, it often means a new overlapping note changes pitch without retriggering envelopes.

Why it matters:

Legato creates smooth melodic lines.

Design implication:

Legato behavior must define envelope retrigger, glide, note priority, and what happens when earlier held notes are released.

## Key Tracking

Key tracking maps note pitch to a modulation value.

Common destinations:

- Filter cutoff.
- Envelope time scaling.
- Oscillator brightness.
- Noise level.
- Resonator tuning.

Why it matters:

It helps patches behave consistently across registers.

Design implication:

Key tracking should be a modulation source available beyond the filter.

## Microtuning

Microtuning uses tuning systems other than standard twelve-tone equal temperament.

Why it matters:

It supports experimental music, world tuning systems, just intonation, historical temperaments, and expressive pitch design.

Design implication:

Microtuning can be deferred, but pitch architecture should avoid assumptions that make it impossible later.

## MPE And Per-Note Expression

MPE stands for MIDI Polyphonic Expression. It allows expressive controls such as pitch bend, pressure, and timbre to apply per note rather than only globally.

Why it matters:

Per-note expression makes electronic instruments behave more like expressive acoustic instruments. One note in a chord can bend while another stays still.

Design implication:

Even before implementation, the architecture should keep per-note control conceptually possible. Voice-level modulation is the natural place for per-note expression.

## MIDI 2.0 Concepts

MIDI 2.0 expands the older MIDI model with higher-resolution control, richer communication, and more explicit capabilities.

Why it matters:

Future expressive control may involve higher-resolution parameter changes, per-note controllers, and better device communication.

Design implication:

The project should avoid designing around only low-resolution global control. Parameters should conceptually support precise automation and expressive control.

## Automation

Automation is parameter movement recorded or controlled by a host, sequencer, or timeline.

Why it matters:

Automation lets a patch change over a song or performance without manual movement.

Design implication:

Automated parameters need smoothing, timing rules, and clear interaction with modulation.

## Performance Control Recommendations

For the first architecture:

- Define polyphonic voice behavior.
- Define monophonic behavior separately.
- Include velocity as a core modulation source.
- Include pitch bend and mod wheel as performance controls.
- Design sustain pedal behavior conceptually.
- Keep per-note expression possible in the voice model.
- Document voice stealing and release behavior before implementation.

