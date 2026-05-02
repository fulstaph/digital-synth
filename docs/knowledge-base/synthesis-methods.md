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

### Partial Tracking

Partial tracking is the process of analyzing recorded audio to identify and follow individual frequency components as they evolve over time. An analysis algorithm examines successive frames of a recording, detects prominent frequency peaks, and connects them across frames into continuous trajectories called partial tracks. Each track captures the frequency, amplitude, and phase of one spectral component throughout its lifetime.

Why it matters: partial tracking enables resynthesis of real-world sounds using additive methods. Once a sound has been decomposed into tracked partials, each one becomes a controllable sine component that can be edited independently. A designer can remove individual partials, shift their frequencies, scale their amplitudes, or retime their evolution without affecting the rest of the spectrum.

The accuracy of partial tracking depends on analysis window size, overlap, and the algorithm's ability to handle crossing partials and transient events. For this project, partial tracking is relevant as the bridge between recorded audio and the additive engine. It determines how faithfully a real sound can be imported into the synthesizer and how much editorial control the user gains after import.

### Spectral Envelope

The spectral envelope describes the overall amplitude shape across all partials at a given moment in time. Rather than looking at each partial individually, the spectral envelope captures the broad contour of energy distribution across the frequency range. It defines the brightness, formant character, and timbral quality of the sound independently from the specific partial frequencies underneath.

Why it matters: changing the spectral envelope while keeping partial frequencies constant can transform a bright tone into a dark one, or shift the perceived vowel character of a sound, without changing its harmonic identity. Two sounds can share the same partial frequencies but sound entirely different because their spectral envelopes emphasize different frequency regions.

Spectral envelopes are central to formant synthesis, vocal modeling, and timbral morphing. For this project, providing spectral envelope controls alongside individual partial editing gives users both macro-level timbral shaping and micro-level harmonic precision. The spectral envelope acts as a master brightness and character control that operates above the individual partial layer.

### Real-Time Partial Control

Real-time partial control means treating individual partials or groups of partials as modulation destinations that can be adjusted during performance. Rather than setting static partial levels, the player or modulation system can continuously reshape the harmonic spectrum. Partials can be grouped by harmonic number, by frequency range, by odd or even classification, or by custom selection.

Why it matters: grouping strategies enable precise harmonic sculpting. Increasing odd harmonics creates hollow, clarinet-like tones. Boosting high-numbered partials adds brightness and edge. Reducing specific partials creates notch-like spectral gaps that give the sound a distinctive filtered quality. Modulating partial groups with LFOs or envelopes produces spectral animation that is impossible to achieve with conventional subtractive filtering.

For this project, real-time partial control turns additive synthesis from a static spectrum editor into a playable instrument. The interface challenge is presenting potentially hundreds of partials in a way that supports both broad gestures and detailed editing. Grouping, scaling, and macro controls are essential to making partial-level modulation musically practical rather than overwhelming.

### Resynthesis

Resynthesis is the process of analyzing recorded audio into its constituent partials and then recreating the sound using additive oscillators. The analysis phase decomposes the recording into partial tracks with time-varying frequency, amplitude, and phase data. The synthesis phase drives a bank of sine oscillators using that data to reproduce the original sound.

Why it matters: resynthesis bridges the gap between sampled realism and synthetic flexibility. Once a sound has been resynthesized, it can be edited in ways that raw samples do not permit. Partials can be removed to thin the spectrum, frequencies can be shifted to transpose without time-stretching artifacts, time can be stretched without pitch change, and two analyzed sounds can be morphed by interpolating between their partial data.

The quality of resynthesis depends on having enough oscillators to capture the spectral detail, accurate partial tracking during analysis, and proper handling of the residual noise component that partials alone may not fully represent. For this project, resynthesis represents an advanced capability that connects the additive engine to real-world sound sources and enables a unique category of sound design not available through other synthesis methods.

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

### Resampling And Pitch Shifting

Samples are transposed to different pitches by changing the playback rate. Playing a sample faster raises its pitch while shortening its duration. Playing it slower lowers its pitch while stretching its duration. This basic resampling approach is simple and efficient but introduces quality problems over wide pitch ranges.

Why it matters: upward transposition compresses the spectral content, making the sound thin, nasal, and artificial. Downward transposition expands the spectrum, producing a slow, muffled quality with audible artifacts. The further a sample is transposed from its original recorded pitch, the more obvious these degradations become. This is why realistic sample-based instruments use multisampling, recording the same instrument at many pitches across its range so that no single sample must be transposed more than a few semitones.

Multisampling introduces its own challenges: transitions between sample zones must be smooth, and the total number of recordings grows quickly. The project should plan for pitch-shift quality limits as a core design consideration when supporting sample playback, and treat multisample mapping as essential for any instrument that needs to sound natural across its full range.

### Loop Point Behavior

Sustaining a sample beyond its natural length requires looping a portion of the waveform. Finding stable loop points means identifying positions where the waveform can repeat without producing audible clicks, pitch jumps, or tonal artifacts. Good loop points occur where the waveform crosses zero with similar slope and harmonic content on both sides of the loop boundary.

Why it matters: loop quality directly determines whether sustained sounds feel natural or mechanical. A poorly chosen loop point creates a periodic click or a timbral discontinuity that the ear quickly identifies as artificial. Even small mismatches in amplitude, phase, or harmonic content at the loop boundary produce audible artifacts that worsen with repetition.

Crossfade looping blends the end of the loop region into the beginning, smoothing the transition by mixing overlapping segments. This reduces boundary artifacts but requires enough stable audio on both sides of the loop point to produce a convincing blend. The crossfade length, shape, and position all affect the smoothness of the result. The project should treat loop point management as a critical part of sample instrument quality.

### Velocity Layers And Key Zones

Velocity layers map different recordings to different dynamic ranges. A softly played note triggers a sample recorded at low intensity, while a forcefully played note triggers a sample recorded with strong attack and brighter harmonic content. Key zones assign different samples to different pitch regions so each part of the keyboard has an appropriate recording rather than a heavily transposed version of a distant sample.

Why it matters: real instruments sound fundamentally different at different dynamics and in different registers, not just louder or softer versions of the same timbre. A piano struck gently has a round, warm tone. The same piano struck hard produces a bright, complex attack with different harmonic emphasis. Similarly, a violin playing in its low register has a different character from the same violin in its upper range.

Multi-sample mapping with velocity layers and key zones recreates these natural variations. Without them, a sample instrument responds only to volume changes, missing the timbral shifts that make performance feel alive. The number of layers and zones directly affects realism but also multiplies memory requirements and mapping complexity.

### Memory And Loading

Sample data can be large. A detailed multisampled instrument with multiple velocity layers, round robins, and articulations can require hundreds of megabytes or more. Loading this data from storage into memory is slow compared to real-time audio deadlines and must happen outside the audio processing path.

Why it matters: memory and loading are architectural concerns that affect instrument switching time, maximum instrument size, and overall system responsiveness. Two common strategies exist. Preloading reads the entire instrument into memory before playback begins, providing instant access to all samples but consuming large amounts of memory and requiring a loading wait. Streaming reads only a small buffer ahead of time and fetches additional sample data from disk during playback, reducing memory use but requiring fast storage and careful buffer management to avoid dropouts.

The project should treat sample memory management as an infrastructure concern separate from the audio engine. Instrument switching, background loading, memory budgets, and storage speed requirements all need design attention to prevent sample management from interfering with real-time performance or degrading the user experience during instrument browsing.

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

### Exciter-Resonator Model

The exciter-resonator framework divides a physical model into two cooperating halves. The exciter provides the energy that sets the system in motion. An impulse serves as a pluck, a noise burst imitates breath, sustained friction represents bowing, and a sharp transient simulates a strike. The resonator receives that energy and shapes it according to its own physical properties. String length determines pitch, tube diameter influences timbre, and membrane tension controls tone color.

Why it matters: separating exciter from resonator turns a single physical structure into a family of instruments. The same resonator can be plucked, bowed, struck, or blown, and each combination yields a different character. This separation also gives sound designers independent control over the attack quality and the sustain body. Changing the exciter changes how the note begins without altering the resonant character, while reshaping the resonator transforms the body of the sound without touching the initial transient.

For this project, an exciter-resonator architecture means one well-designed resonator model can serve many instrument voices by swapping or blending exciters. It also maps naturally to performance controls: velocity can scale exciter intensity while pressure can modify resonator damping.

### Karplus-Strong

Karplus-Strong is a specific and accessible physical modeling technique that produces convincing plucked-string sounds from minimal components. A short burst of noise or an impulse is loaded into a delay line whose length determines the pitch of the resulting tone. The output of the delay line passes through a lowpass filter and is fed back to the input.

Why it matters: the filter controls damping behavior. More filtering produces faster high-frequency decay and a duller, softer tone. Less filtering allows brightness to sustain longer, producing a harder or more metallic pluck. The feedback amount controls overall sustain: high feedback keeps the string ringing, low feedback produces a short, muted pluck.

Key parameters are delay length for pitch, feedback amount for sustain duration, and filter cutoff for brightness and damping character. Because the technique uses only a noise burst, a delay line, and a filter, it is computationally inexpensive and easy to understand. It serves as a practical entry point for physical modeling concepts and can be extended with additional filtering, nonlinear feedback, or coupled delay lines for richer string behavior.

### Damping And Decay

Damping describes how energy dissipates within a vibrating system over time. In real physical instruments, high-frequency components lose energy faster than low-frequency components. This means that a struck or plucked sound becomes progressively darker as it sustains, not just quieter. The brightness-to-darkness evolution is one of the most recognizable qualities of acoustic sound.

Why it matters: damping behavior is musically important because it creates the natural timbral arc heard in plucked strings, struck bells, and vibrating membranes. A guitar string starts bright and mellows. A cymbal begins with a wash of high frequencies that gradually thins. Without frequency-dependent damping, modeled sounds sustain with an unnaturally constant spectrum that sounds synthetic and lifeless.

Damping parameters should include overall decay time, frequency-dependent decay balance, and possibly per-partial or per-band damping controls. The interplay between damping rate and pitch register also matters: low notes in real instruments tend to sustain longer than high notes, and a convincing model should reflect this relationship.

### Stability

Physical models can become unstable when the feedback network gains more energy than it loses during each cycle. This produces runaway oscillation that manifests as harsh noise, extreme amplitude, or silence when the system clips or overflows. Numerical precision limits compound the problem: rounding errors can accumulate across thousands of feedback iterations per second, gradually pushing a stable model toward divergence.

Why it matters: parameter changes during playback can push a previously stable model into instability. A user turning up feedback or reducing damping in real time may cross the stability boundary without warning. Unlike a filter that distorts gracefully when overdriven, an unstable physical model tends to fail catastrophically, producing unmusical noise or dead silence rather than degrading in a controllable way.

Stability is a design concern that affects parameter range limiting, internal gain compensation, and safety clamping. The project should treat stability as a first-class constraint when designing physical model interfaces. Parameters should be scaled so that musically useful ranges stay within stable bounds, and safety mechanisms should catch and contain divergence before it reaches the audio output.

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

