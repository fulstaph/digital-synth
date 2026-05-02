# Synthesis Glossary

This glossary defines synthesis and digital audio terms used throughout the project.

## A

### Additive Synthesis

A synthesis method that builds complex sounds by summing simpler waveforms, usually sine waves. Important because it exposes sound as a spectrum of partials and can create precise harmonic structures.

### ADSR

Attack, decay, sustain, and release. The classic envelope model. Important because it defines the articulation of most synthesizer sounds.

### Aftertouch

Pressure applied after a note starts. Important because it lets a performer shape sustained notes expressively.

### Algorithm (FM)

A routing arrangement of operators in FM synthesis that determines which operators modulate which. Important because different algorithms produce fundamentally different timbral families.

### Aliasing

False frequencies created when a digital system produces or processes frequencies above Nyquist. Important because it can make digital synths sound harsh or broken.

### Amplitude

Signal level. Important because it relates to loudness, gain staging, envelopes, modulation, and clipping.

### Amplitude Modulation

Changing signal level with another signal. Important for tremolo, ring-mod-like tones, and audio-rate sideband effects.

### Arpeggiator

A feature that automatically plays held notes one at a time in a pattern. Important for rhythmic electronic music and live performance.

### Attack

The envelope stage where a value rises toward its peak. Important because it defines how quickly a sound begins.

### Audio Rate

A rate of change that occurs every sample. Important for oscillators, audio-band modulation, and precise signal processing.

### Automation

Recorded or host-controlled parameter movement. Important because it allows patches to change over a performance or song.

## B

### Band-Limiting

Restricting a signal to avoid frequencies above a target limit. Important for avoiding aliasing.

### Band-Pass Filter

A filter that keeps a range of frequencies and reduces frequencies above and below that range. Important for focused, nasal, resonant, or formant-like tones.

### Bit Crushing

Reducing the effective bit depth of a signal as a creative effect. Important for lo-fi textures and retro digital character.

### Bit Depth

The resolution of a digital audio representation. Important for noise floor, dynamic range, and final audio quality.

### BLEP

Band-limited step function, a method for reducing aliasing by replacing sharp waveform discontinuities with band-limited versions. Important for clean digital oscillator quality.

### Blue Noise

Noise with increasing energy at higher frequencies (3dB per octave rise). Important for adding air and brightness without low-end mud.

### Buffer

A block of samples processed together. Important because buffer size affects latency and CPU deadline pressure.

### Bypass

A control that disables a processing stage. Important for comparison, troubleshooting, and sound design.

## C

### Carrier

In FM synthesis, the oscillator heard directly. Important because its frequency and routing shape the perceived pitch and tone.

### Chorus

An effect using short modulated delays to thicken and widen sound. Important for pads, keys, basses, and stereo movement.

### Clipping

Flattening or limiting a signal when it exceeds an allowed range. Important because accidental clipping can sound harsh, while intentional clipping can be a distortion effect.

### Comb Filter

A filter with repeated peaks and notches across the spectrum. Important for flanging, resonators, and physical-modeling-like tones.

### Compression

Dynamic processing that reduces level above a threshold. Important for controlling peaks and changing sustain or punch.

### Control Rate

A rate of change slower than audio rate. Important for efficient modulation and parameter updates.

### Convolution

Applying the acoustic characteristics of one signal to another using impulse responses. Important for realistic space simulation and creative processing.

### Cutoff Frequency

The main frequency boundary of a filter. Important because it is one of the most musically significant tone controls.

## D

### Decay

The envelope stage after attack, moving from peak to sustain level. Important for plucks, percussion, basses, and articulation.

### Decibel

A logarithmic unit for level ratios. Important because audio level perception is closer to logarithmic than linear.

### Delay

An effect that repeats sound after a time interval. Important for echo, rhythm, depth, and spatial design.

### Delay Stage (Envelope)

An optional envelope phase that inserts a waiting period before the attack begins. Important for layered sounds and delayed vibrato onset.

### Detune

A small pitch offset. Important for width, beating, unison, and analog-style movement.

### Detune Curve

The distribution shape of pitch offsets across unison voices. Important because it determines whether unison sounds tight or wide.

### Distortion

Nonlinear change to a waveform. Important because it creates harmonics, aggression, warmth, or character.

### Drift

Slow random or semi-random variation in pitch, phase, or tone. Important for organic movement and analog-style behavior.

### Dry Signal

The unprocessed signal. Important for wet/dry effect balance.

## E

### Envelope

A time-shaped modulation source. Important because it defines how parameters change during a note.

### Envelope Follower

A modulation source that derives its value from the amplitude of an audio signal. Important for reactive modulation and sidechain-like behavior.

### EQ

Equalization. Frequency-specific level adjustment. Important for final tone shaping and mix fit.

### Equal-Power Panning

A panning law that preserves perceived loudness across the stereo field. Important for consistent spatial placement.

### Event Rate

A processing rate triggered by specific events like note-on or note-off. Important for voice allocation and envelope triggering.

### Expression

Performance control beyond basic note on and off, such as pressure, pitch bend, velocity, or per-note timbre. Important because it makes the synth feel playable.

## F

### Feedback

Routing part of a signal back into an earlier point. Important for delay, FM, filters, resonators, and physical modeling.

### Feedback Operator

An FM operator that modulates itself, adding harmonics progressively. Important for creating sawtooth-like or noise-like textures without additional operators.

### FFT

Fast Fourier Transform, a method for analyzing a signal's frequency content. Important for spectral processing and visualization.

### Filter

A processor that changes frequency balance. Important because it is central to subtractive synthesis and timbral motion.

### Filter FM

Audio-rate modulation of filter cutoff frequency. Important for metallic and aggressive timbres.

### Filter Topology

The internal arrangement and feedback structure of a filter. Important because topology determines the musical character of a filter independently of its type and settings.

### Flanger

An effect based on very short modulated delay and feedback. Important for sweeping comb-filter movement.

### FM Synthesis

Frequency modulation synthesis. A method where one oscillator changes the frequency of another. Important for complex digital timbres, bells, electric pianos, and metallic sounds.

### Formant

A resonant frequency region associated with vocal or body-like character. Important for vocal-like synthesis and acoustic modeling.

### Formant Synthesis

Generating vowel-like sounds using parallel resonant filters tuned to formant frequencies. Important for vocal timbres without a microphone input.

### Frequency

Cycles per second, measured in hertz. Important for pitch, filters, LFOs, oscillators, and delay-based effects.

## G

### Gain

Signal level adjustment. Important for mixing, headroom, distortion, and output.

### Gain Staging

Managing signal levels across a signal path. Important because it prevents accidental clipping and preserves intended tone.

### Glide

Smooth pitch movement between notes. Important for basses, leads, and legato performance.

### Grain

A very short segment of audio (1-100ms) used as a building block in granular synthesis. Important because grain duration determines whether the result sounds pitched, textured, or rhythmic.

### Grain Density

The number of grains produced per second in granular synthesis. Important because density determines whether individual grains are heard or fuse into continuous sound.

### Grain Envelope

The amplitude shape applied to each grain. Important for smoothness and avoiding clicks at grain boundaries.

### Granular Synthesis

A method that creates sound from tiny audio grains. Important for textures, time-stretching, clouds, and experimental sound design.

## H

### Haas Effect

A stereo widening technique using short delays (1-30ms) between channels to create directional perception. Important for natural-sounding spatial placement.

### Harmonic

A frequency component at an integer multiple of the fundamental. Important because harmonic structure strongly determines timbre.

### Headroom

Level margin before clipping. Important because polyphony, resonance, effects, and unison can raise signal level.

### High-Pass Filter

A filter that keeps high frequencies and reduces low frequencies. Important for thinning sounds and controlling low-end buildup.

### Hold

An optional envelope stage that maintains a level for a time before decay. Important for percussion and shaped attacks.

### Hold Stage (Envelope)

An optional envelope phase between attack and decay that maintains peak level. Important for consistent transient punch.

## I

### Impulse Response

A recording of a system's response to a brief full-spectrum impulse. Important for convolution reverb and capturing the character of real spaces.

### Inharmonic

Not aligned to integer multiples of the fundamental. Important for bells, metal, percussion, and complex digital tones.

### Init Patch

The default starting state for sound design in a synthesizer. Important because it is the most-used starting point for creating new sounds.

### Interpolation

Smoothing or estimating values between known points. Important for wavetables, sample playback, and parameter changes.

## K

### Karplus-Strong

A physical modeling technique using a noise burst fed into a filtered delay line to produce plucked-string sounds. Important as the most accessible physical modeling method.

### Key Tracking

Mapping note pitch to another parameter. Important for making filters, envelopes, or tone behave consistently across the keyboard.

## L

### Ladder Filter

A filter topology built from cascaded stages with global feedback, known for bass loss at high resonance and warm character. Important as the reference sound for classic subtractive synthesis.

### Latency

Delay between input and output. Important because high latency makes instruments feel disconnected.

### Legato

Connected note behavior, often without envelope retrigger. Important for smooth melodic lines.

### LFO

Low-frequency oscillator used as a modulation source. Important for vibrato, tremolo, filter motion, rhythmic movement, and evolving timbres.

### Limiter

A dynamics processor that prevents peaks from exceeding a ceiling. Important for output safety.

### Looping Envelope

An envelope that repeats some or all stages continuously. Important for rhythmic gating and evolving modulation effects.

### Low-Pass Filter

A filter that keeps low frequencies and reduces high frequencies. Important as the central filter type in subtractive synthesis.

## M

### Macro

A high-level control mapped to multiple parameters. Important for performance and preset design.

### Mid/Side Processing

Encoding stereo as sum (mid) and difference (side) channels for independent processing. Important for precise stereo image control.

### MIDI

Musical Instrument Digital Interface. A standard for musical note and control messages. Important because it is a common way to control synthesizers.

### MIDI 2.0

An expanded MIDI standard with higher-resolution and richer communication concepts. Important for future expressive control.

### MinBLEP

Minimum-phase band-limited step, a causal anti-aliasing correction that avoids pre-ringing. Important for clean oscillator quality with natural transient response.

### Mixer

A stage that combines signals. Important for oscillator balance, voice summing, effects, and output.

### Modulation

Changing a parameter with another source. Important because it creates movement and expression.

### Modulation Destination

A parameter affected by modulation. Important because destinations define what can move.

### Modulation Index

In FM synthesis, the amount by which a modulator affects a carrier's frequency. Important as the primary brightness and complexity control in FM.

### Modulation Matrix

A routing system connecting modulation sources to destinations. Important because it makes the synth flexible and expressive.

### Modulation Source

A value or signal used to affect another parameter. Important because sources provide movement and control.

### Modulator

In FM, the oscillator that changes the carrier. More generally, any source that modulates another parameter. Important for creating changing spectra.

### Monophonic

One note or voice at a time. Important for basses, leads, glide, and classic synth behavior.

### MPE

MIDI Polyphonic Expression. A MIDI approach for per-note expression. Important because it allows independent pitch, pressure, and timbre control per note.

### Multimode Filter

A filter offering several modes such as low-pass, high-pass, band-pass, and notch. Important for flexible tone shaping.

### Multi-Segment Envelope

An envelope with more than four stages. Important for complex evolving sounds that ADSR cannot describe.

## N

### Noise

A random or pseudo-random signal. Important for percussion, breath, texture, risers, and modulation.

### Note Off

An event that releases a note. Important because it usually starts envelope release rather than immediately stopping sound.

### Note On

An event that starts a note. Important because it carries timing, pitch, and usually velocity.

### Notch Filter

A filter that removes a narrow frequency band. Important for hollow, phase-like, or corrective effects.

### Nyquist Frequency

Half the sample rate. Important because frequencies above it alias.

## O

### Operator

In FM synthesis, an oscillator with associated level control and routing role. Important for building FM algorithms.

### Oscillator

A repeating signal generator. Important as a primary source of pitched sound.

### Oversampling

Processing at a higher internal sample rate. Important for reducing aliasing in nonlinear processing and bright synthesis.

## P

### Pan

Positioning a signal between left and right channels. Important for stereo placement.

### Panning Law

The mathematical relationship between pan position and channel levels. Important for consistent perceived loudness across the stereo field.

### Parallel Processing

Blending an original signal with a heavily processed version. Important for adding processing character while retaining original dynamics.

### Parameter Smoothing

Gradually changing a parameter to avoid clicks or zipper noise. Important for polished control behavior.

### Paraphony

Multiple pitches sharing part of the signal path. Important because it behaves differently from true polyphony.

### Partial

One frequency component of a sound. Important for additive synthesis and spectral analysis.

### Partial Tracking

Analyzing recorded audio to identify and follow individual frequency components over time. Important for additive resynthesis.

### Perlin Noise

A smooth, spatially coherent random function. Important as a modulation source for organic drift and natural-feeling variation.

### Phase

Position within a waveform cycle. Important for oscillator behavior, sync, stereo, cancellation, and FM.

### Phase Modulation

Changing oscillator phase with another signal. Important because it can create FM-like sounds.

### Phase Vocoder

A spectral processing framework for independent time and pitch manipulation. Important for time-stretching and pitch-shifting tonal material.

### Phaser

An effect that creates moving spectral notches through phase-shifting stages. Important for swirling motion.

### Physical Modeling

Synthesis based on simulated vibrating systems. Important for expressive and acoustic-like instruments.

### Pitch

The perceived musical height of sound. Important for all note-based synthesis.

### Pitch Bend

Continuous pitch control. Important for expressive performance.

### PolyBLEP

Polynomial band-limited step, a computationally efficient anti-aliasing method. Important as a practical choice for real-time oscillator quality.

### Polyphony

The ability to play multiple voices at once. Important for chords, pads, releases, and layered sounds.

### Portamento

Another term for glide. Important for smooth pitch transitions.

### Preset

A saved synth state. Important for recall, sharing, testing, and sound design.

### Pulse Width

The duty cycle of a pulse wave. Important because changing it changes tone.

### PWM

Pulse-width modulation. Important for animated classic synth tones.

## Q

### Q

Filter sharpness or resonance measure. Important because it controls how focused a filter peak is.

### Quantization Noise

Noise from rounding values to limited resolution. Important for audio quality and control smoothness.

## R

### Release

The envelope stage after note off. Important because it controls how a sound fades.

### Resonance

Emphasis near filter cutoff. Important for character, motion, and self-oscillating behavior.

### Resynthesis

Analyzing recorded audio into partials and recreating it using additive oscillators. Important for editable reconstructions of real-world sounds.

### Reverb

An effect that creates a sense of space. Important for depth, ambience, and texture.

### Ring Modulation

Multiplication of two signals, often creating sidebands. Important for metallic and inharmonic tones.

## S

### Sample

One digital value representing signal amplitude at a moment. Important as the basic unit of digital audio.

### Sample Rate

Number of samples per second. Important because it determines timing and Nyquist frequency.

### Sample-Based Synthesis

Synthesis based on recorded audio. Important for realistic, organic, or found-sound material.

### Sample-Rate Reduction

Reducing effective sample rate as a creative effect. Important for lo-fi character and controlled aliasing artifacts.

### Saturation

Smooth nonlinear coloration. Important for warmth, density, and harmonic enrichment.

### Self-Oscillation

Filter behavior where high resonance produces a tone even without input. Important for special effects and pitched filter sounds.

### Sideband

A frequency component created by modulation (AM, FM, or ring modulation). Important for understanding the spectral results of modulation synthesis.

### Sidechain

Using one signal's level to control processing on another signal. Important for ducking, pumping rhythms, and mix clarity.

### Sine Wave

A pure waveform with one frequency. Important for additive synthesis, FM, sub bass, and testing.

### Slew

Smoothing applied to a modulation signal to soften sharp transitions. Important for reducing stepping artifacts in modulation.

### Sound Source

Anything that produces raw audio. Important as the beginning of a synth voice.

### Spectral Envelope

The overall amplitude shape across all partials at a given moment. Important for controlling timbral brightness independently from harmonic structure.

### Spectral Morphing

Blending the spectral content of two signals in the frequency domain. Important for creating hybrid timbres.

### Spectrum

The frequency content of a sound. Important because timbre is largely spectral.

### Square Wave

A waveform with strong odd harmonics. Important for hollow, reedy, and classic synth tones.

### State-Variable Filter

A filter topology providing simultaneous low-pass, high-pass, band-pass, and notch outputs. Important for flexible tone shaping with preserved bass at high resonance.

### Step Sequencer

A programmable sequence of values that advances one step per clock tick. Important for rhythmic parameter patterns.

### Subtractive Synthesis

A method that starts with rich sources and removes frequencies with filters. Important as the recommended first synthesis architecture.

### Supersaw

Multiple detuned sawtooth voices layered together. Important as a defining modern electronic synthesizer sound.

### Sustain

The envelope level held while a note remains active. Important for distinguishing plucks, pads, organs, and drones.

### Swing

Delaying alternate steps in a rhythmic pattern to create shuffle feel. Important for groove and rhythmic character.

### Sync

Oscillator behavior where one oscillator resets another's phase. Important for bright, aggressive harmonic sweeps.

## T

### Timbre

Tone color. Important because it is the main perceived identity of a sound beyond pitch and loudness.

### Time-Pitch Decoupling

Independent control of playback speed and pitch. Important as granular synthesis's defining capability.

### Transient Shaping

Emphasizing or reducing the attack portion of a signal independently from sustain. Important for controlling punch and presence.

### Tremolo

Amplitude modulation. Important for pulsing loudness movement.

### Triangle Wave

A soft waveform with mostly odd harmonics that fade quickly. Important for mellow tones.

## U

### Unipolar

A modulation range that moves from minimum to maximum in one direction. Important for envelopes, velocity, and macro design.

### Unison

Multiple voices or oscillators playing the same note with slight differences. Important for width and density.

## V

### VCA

Voltage-controlled amplifier in analog terminology; conceptually the amplitude control stage. Important because it shapes loudness.

### VCF

Voltage-controlled filter in analog terminology; conceptually the filter stage. Important because it shapes frequency content.

### Velocity

Note-on intensity. Important for dynamics and articulation.

### Vibrato

Pitch modulation. Important for expressive sustained notes.

### Violet Noise

Noise with steep high-frequency emphasis (6dB per octave rise). Important for adding extreme brightness and air.

### Vocoder

A processor that imposes the spectral shape of one signal onto another. Important for the classic "talking synthesizer" effect.

### Voice

One active note or sound event. Important for polyphony and per-note articulation.

### Voice Allocation

The process of assigning notes to voices. Important for musical continuity.

### Voice Stealing

Reusing an active voice when no inactive voice is available. Important for managing limited polyphony.

## W

### Wavetable

A collection of waveforms used by a wavetable oscillator. Important for evolving digital timbres.

### Wavetable Position

The selected location within a wavetable. Important as a modulation destination.

### Wavefolding

A nonlinear process that folds waveform peaks back into range. Important for complex bright tones.

### Waveform

The shape of a signal cycle. Important because it determines harmonic content.

### Waveshaping

Transforming a signal through a nonlinear curve. Important for distortion and harmonic generation.

### Wet Signal

The processed effect signal. Important for balancing effects.

## Z

### Zero-Delay Feedback

A filter design concept where the filter responds to its own output within the same computational step. Important for accurate analog-style resonance behavior.

### Zipper Noise

Audible stepping from coarse parameter changes. Important because it makes controls sound unpolished or broken.

