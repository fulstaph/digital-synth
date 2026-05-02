# Parameter Design Guidance

Every control in a synthesizer is a parameter with a type, range, scaling curve, default value, display unit, and smoothing behavior. Inconsistent parameter design makes an instrument feel unpredictable: some knobs respond musically while others feel dead across most of their range. This document establishes design-level principles for parameter specification.

## Parameter Types

Parameters fall into four fundamental types, and the type of a parameter determines nearly everything about how it behaves in the instrument. Choosing the wrong type leads to problems that ripple across the preset system, the modulation engine, the interface, and the user experience.

A continuous parameter accepts any value within a defined range. Filter cutoff frequency, oscillator detune amount, envelope attack time, and effect mix level are all continuous. These parameters require smoothing when changed in real time, store as floating-point values in presets, and can be swept smoothly by modulation sources or automation. Most of the parameters in a synthesizer are continuous, and they are the ones that benefit most from careful scaling and resolution decisions.

A discrete parameter accepts specific integer steps within a range. Oscillator unison voice count, semitone transpose amount, and arpeggiator step count are discrete. These parameters snap to valid steps, do not require smoothing between adjacent values, and store as integers. They still need careful range definition because each step should represent a musically distinct choice. A discrete parameter with too many steps that sound identical wastes control range, while one with too few steps limits the musician unnecessarily.

A toggle parameter is a special case of discrete with exactly two states: on or off. Effect bypass, oscillator sync enable, and legato mode are toggles. They switch immediately, never interpolate, and store as boolean values. Despite their simplicity, toggles need clear labeling and predictable behavior. The user should always know which state is active and what changes when the toggle flips.

An enumerated parameter selects from a named list of options. Filter mode (lowpass, highpass, bandpass, notch), oscillator waveform shape (sine, saw, square, triangle), and LFO sync mode (free, tempo-synced) are enumerated. Like toggles, they switch immediately and do not interpolate. The distinction from discrete parameters matters for preset morphing: interpolating between "lowpass" and "bandpass" is meaningless, while interpolating between three and five unison voices at least has a plausible midpoint. Enumerated parameters also carry the question of ordering: should the list be alphabetical, ordered by sonic character, or ordered by common usage? The order affects how quickly users find the option they want.

Why type matters for this project: the parameter type determines the control widget presented in the interface, whether the audio engine must smooth value changes, how the preset system serializes the value, and whether preset morphing or randomization can handle the parameter gracefully. Getting the type wrong is difficult to fix later because it affects every subsystem that touches the parameter.

## Range And Scaling

Every continuous parameter has a minimum value, a maximum value, and a scaling curve that maps the control position (zero to one, bottom to top, left to right) to the actual parameter value. The scaling curve is the single most important factor in whether a parameter feels musical or frustrating.

Linear scaling means equal movement of the control produces equal change in the value. A knob that controls mix percentage from zero to one hundred percent works well with linear scaling because each increment of the knob adds the same perceptual amount of the wet signal.

Logarithmic scaling means equal movement of the control produces equal ratios of change in the value. This is required for any frequency-based parameter. A filter cutoff that spans 20 Hz to 20 kHz must use logarithmic scaling because the musical distance between 100 Hz and 200 Hz (one octave) is the same as the distance between 1000 Hz and 2000 Hz (also one octave). With linear scaling, the bottom ninety-five percent of the knob would cover the low frequencies and the top five percent would fly through the entire treble range.

Exponential scaling is useful for time-based parameters such as envelope attack, decay, and release times, and delay time. Short times need precision (the difference between 1 ms and 5 ms attack is dramatic) while long times need range (the difference between 2.0 s and 2.1 s release is inaudible). Exponential mapping concentrates resolution at the short end and stretches the long end.

Why wrong scaling breaks playability: if a filter cutoff uses linear scaling across 20 Hz to 20 kHz, the entire musically useful sweep from about 200 Hz to 5 kHz occupies roughly a quarter of the knob range. The rest of the range is either inaudibly low or harshly high. The parameter technically works but feels dead for most of its travel. Correct scaling distributes the musically useful range evenly across the full control travel.

## Default Values

Every parameter needs a default value, and that value should be musically useful rather than mathematically convenient. The complete set of default values across all parameters defines the init patch, which is the starting point for every sound design session. A musician who opens a new patch and presses a key should hear a clean, recognizable, and pleasant tone that invites further shaping.

Good defaults for a subtractive synthesizer: a single oscillator tuned to concert pitch producing a sawtooth wave, filter set to lowpass mode with cutoff moderately open (around 4 to 8 kHz) and low resonance, amplitude envelope with instant attack and moderate release (around 200 ms), all effects bypassed, all modulation depths at zero, master volume at a safe listening level.

Bad defaults come in several flavors. Everything at zero produces silence, which forces the user to diagnose which parameter needs to change before any sound appears. Everything at maximum produces harsh noise. Arbitrary midpoint values (every knob at fifty percent) produce an unpredictable and usually unpleasant sound because fifty percent of a filter cutoff range, fifty percent resonance, and fifty percent of every effect simultaneously is not a musically coherent starting point.

Why defaults compound: a single bad default can make the init patch confusing. If the amplitude envelope defaults to zero attack and zero release but the filter envelope defaults to a slow attack, the user hears a note that fades in strangely and may not realize the filter envelope is responsible. Every default should be reviewed in the context of all the other defaults, not in isolation.

The init patch is also the implicit reset point for any "initialize parameter" action. If a user resets a single parameter to its default during sound design, that default should make sense within whatever patch they have built, which generally means neutral and non-destructive values: zero modulation depth, bypassed effects, and centered pan.

## Display Units

Parameters should display their values in units that are musically meaningful. The internal representation can use whatever format is convenient for the audio engine, but the value shown to the user must communicate in terms a musician understands.

Frequency parameters such as filter cutoff and LFO rate should display in Hz or kHz. Time parameters such as envelope attack, decay, release, and delay time should display in milliseconds for short durations and seconds for long ones. Pitch parameters such as oscillator transpose should display in semitones, and fine-tuning parameters should display in cents. Level parameters such as oscillator volume and effect send should display in decibels. Mix parameters such as dry/wet balance should display as a percentage. Detune parameters should display in cents. Rate parameters that can synchronize to tempo should display the rhythmic division (1/4, 1/8, 1/8 triplet) when synced and Hz when free-running.

Why display units matter: a cutoff value displayed as "0.73" (a normalized internal value) tells the musician nothing. The same cutoff displayed as "2.4 kHz" tells them the filter is operating in the mid-frequency range and they can predict roughly how the sound will be shaped. A release time displayed as "0.38" is cryptic, but "380 ms" immediately communicates the decay character. Musicians develop intuitions calibrated in real-world units. The instrument should speak the same language.

Display formatting should be context-sensitive. A cutoff of 18732 Hz is more readable as "18.7 kHz." An attack of 0.3 ms is more useful than "0 ms" rounded down. Precision should match audibility: showing a release time to four decimal places implies a precision that no human ear can detect.

## Smoothing Policy

When a parameter changes value during audio playback, the audio engine must decide whether to jump instantly to the new value or interpolate smoothly from the old value to the new one. This decision is the smoothing policy, and it should be defined per parameter rather than applied uniformly.

Fast smoothing (sub-millisecond, typically one to two milliseconds) is required for continuous parameters in the audio signal path that would produce audible clicks or zipper noise if they changed abruptly. Filter cutoff, resonance, oscillator gain, and amplifier level all need fast smoothing. The smoothing time must be short enough that the control still feels responsive, but long enough to eliminate discontinuities in the output waveform.

Medium smoothing (a few milliseconds, typically five to twenty milliseconds) suits parameters where abrupt changes sound wrong but sample-accurate response is unnecessary. Effect wet/dry mix, stereo pan position, and send levels fall into this category. Slightly longer smoothing here avoids clicks without adding perceptible lag.

No smoothing (instant switching) is correct for discrete, toggle, and enumerated parameters. Waveform selection, filter mode, effect bypass, and any parameter that selects between distinct states should switch immediately. Smoothing a transition between "saw" and "square" waveforms is meaningless and would produce audio garbage during the crossfade.

Why the policy matters: over-smoothing makes controls feel sluggish and unresponsive, which is especially damaging for performance gestures like filter sweeps where the musician expects immediate tactile feedback. Under-smoothing produces clicks, zipper noise, and other artifacts that sound like bugs. The correct smoothing time is the shortest duration that eliminates artifacts for that specific parameter.

One important nuance: the same parameter may need different smoothing behavior depending on the source of the change. A filter cutoff moved by a hardware knob benefits from smoothing. The same cutoff driven by an internal envelope at audio rate may already be smooth and adding additional smoothing would round off the envelope shape.

## Resolution And Precision

Resolution describes how many distinct values a parameter can take between its minimum and maximum. Higher resolution means smaller steps between adjacent values, which produces smoother sweeps and finer control.

Standard MIDI control messages provide 7-bit resolution, which yields 128 discrete steps. For parameters like effect bypass or waveform selection, 128 steps is more than enough. For parameters like filter cutoff mapped logarithmically across the audible frequency range, 128 steps means each step jumps by a noticeable fraction of an octave. A slow cutoff sweep controlled by 7-bit MIDI sounds like a staircase rather than a ramp, with each step producing an audible click even through smoothing.

High-resolution alternatives exist. MIDI 1.0 supports 14-bit control change pairs (combining two 7-bit messages) for 16384 steps. MIDI 2.0 natively provides 32-bit resolution for control changes. Internal parameter systems running in software have no inherent resolution limit and typically use 32-bit or 64-bit floating point.

Why resolution matters for this project: the internal parameter representation should always use high-resolution floating point, regardless of the external control source. When a low-resolution MIDI controller drives a parameter, the smoothing system should interpolate between the coarse steps to produce a continuous result. The instrument should never sound stepped because of its own internal resolution, only because of external controller limitations, and even those should be mitigated.

Parameters that are especially sensitive to resolution include filter cutoff (because logarithmic scaling amplifies stepping at low frequencies), oscillator pitch (because the ear is extremely sensitive to pitch discontinuities), and any parameter used as a modulation target at audio rate.

## Parameter Grouping And Organization

Parameters belong to logical groups that reflect the architecture of the synthesizer. Typical groups include oscillator parameters (waveform, tuning, detune, unison count, level), filter parameters (mode, cutoff, resonance, key tracking amount, envelope depth), envelope parameters (attack, decay, sustain, release for each envelope), LFO parameters (waveform, rate, sync mode, depth, phase), effect parameters (type-specific controls, mix, bypass), and global parameters (master volume, polyphony mode, pitch bend range, glide time).

Why grouping matters: the parameter group structure determines how presets are organized and serialized, how the user interface is laid out, how modulation destinations are named and browsed, and how documentation and tooltips reference parameters. A filter cutoff that appears in the preset file under the filter group, on the interface panel labeled "Filter," and in the modulation destination list as "Filter: Cutoff" creates consistency that helps the user build a mental model of the instrument.

Naming conventions within groups should be consistent. If filter parameters are prefixed with "Filter:" then envelope parameters should be prefixed with "Amp Env:" or "Filter Env:" rather than mixing naming styles. The group name should appear in modulation routing displays so the user can distinguish "Filter: Cutoff" from "EQ: Frequency" without ambiguity.

Groups also define the scope of parameter randomization and initialization. A user who randomizes "all oscillator parameters" expects tuning, waveform, detune, and level to change, but not the filter or effects. A user who resets the filter group expects cutoff, resonance, and mode to return to defaults without touching the envelopes.

Subgroups may be necessary for instruments with multiple instances of the same module. A synthesizer with three oscillators needs Oscillator 1, Oscillator 2, and Oscillator 3 as subgroups within the oscillator group, each containing the same parameter set with independent values.

## Design Recommendations For Digital Synth

Based on the principles above, the following recommendations should guide parameter specification for this project.

Define the parameter type, range, scaling curve, default value, display unit, and smoothing policy for every parameter before implementation begins. This specification serves as the contract between the audio engine, the preset system, the modulation router, and the user interface. Ambiguity in the specification will surface as inconsistencies in the instrument.

Use logarithmic scaling for all frequency-domain parameters: filter cutoff, EQ frequency bands, LFO rate in free-running mode, and any parameter measured in Hz. Use exponential scaling for all time-domain parameters: envelope attack, decay, and release times, delay time, and glide time. Use linear scaling for mix, pan, and percentage parameters. Review each parameter individually and confirm the scaling choice by imagining a slow sweep from minimum to maximum and checking that the musically interesting changes are distributed evenly across the control travel.

Display musically meaningful units for every parameter. Never show raw normalized values to the user. Format display values with appropriate precision: whole numbers for Hz below 1 kHz, one decimal place for kHz, whole numbers for milliseconds, one decimal place for seconds.

Use high internal resolution (floating-point) for all continuous parameters. Implement smoothing by default for all continuous parameters in the audio signal path, with the smoothing time defined per parameter based on its audible sensitivity to discontinuities.

Review the complete set of default values as a collective init patch. Play the init patch, evaluate whether it sounds clean and inviting, and adjust any default that contributes to an unpleasant or confusing result. The init patch should be a sound that a new user can immediately understand and begin modifying.

Organize parameters into logical groups with consistent naming conventions. Use these groups as the basis for preset structure, interface layout, and modulation destination browsing. Document each parameter with its full specification so that the audio engine, preset system, and interface can all be developed against the same source of truth.
