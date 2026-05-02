# Noise Types And Modulation

Noise is both a sound source and a modulation source. Different noise spectra serve different musical purposes, and understanding the full range of available noise types enables more precise sound design and more organic modulation behavior. While white, pink, and brown noise are the most commonly discussed varieties, additional noise colors and smooth random functions expand the palette significantly. This document covers the full spectrum of noise types, their musical applications, and how they should inform the architecture of Digital Synth.

## Noise Spectrum Overview

Noise is classified by its spectral distribution, meaning the relative energy present at different frequencies. The naming convention borrows from the analogy with visible light: just as white light contains all wavelengths equally, white noise contains all frequencies at equal energy. Other noise colors describe spectra that emphasize certain frequency ranges over others, paralleling how colored light has a dominant wavelength.

White noise has equal energy per frequency, producing a flat power spectrum. It sounds bright and hissy because the ear perceives higher frequencies as louder due to the greater number of frequency bins per octave in the upper range. Pink noise has equal energy per octave, which means its power decreases at three decibels per octave as frequency rises. This rolloff matches how human hearing groups frequencies logarithmically, so pink noise sounds perceptually balanced and is often described as natural or even. Brown noise, sometimes called red noise or Brownian noise after Robert Brown, falls off at six decibels per octave. It has a strong low-frequency emphasis that sounds deep, rumbling, and thunder-like. Blue noise rises at three decibels per octave, giving it more high-frequency energy than white noise. Violet noise rises at six decibels per octave, producing the steepest high-frequency emphasis of the standard noise colors.

Why it matters:

- Each noise color occupies a different spectral region and serves different musical purposes.
- Choosing the wrong noise type for a task wastes processing effort on frequency content that will be filtered away.
- Understanding the spectral slope of each type helps predict how noise will interact with filters and other sound sources in a mix.
- The color metaphor provides a shared vocabulary for discussing noise behavior across documentation, presets, and interface labels.

## Blue And Violet Noise

Blue and violet noise are the high-frequency counterparts of pink and brown noise. Blue noise has a spectral slope that rises at three decibels per octave, meaning higher frequencies carry progressively more energy while lower frequencies are attenuated. Violet noise rises more steeply at six decibels per octave, concentrating even more energy in the upper range.

Sound character:

- Blue noise sounds airy and bright without the full-bandwidth hiss of white noise. It has a lighter quality because the low-frequency rumble is absent.
- Violet noise sounds thin and sharp, emphasizing sibilance and presence frequencies.

Musical applications:

- Adding air and shimmer to a sound without introducing low-end mud. A layer of blue noise behind a pad adds breathy texture without competing with bass frequencies.
- High-frequency modulation that naturally avoids affecting bass-heavy parameters. Blue noise routed to filter cutoff creates sparkly movement in the treble without disturbing the low end.
- Complementing pink noise in layered textures. Pink noise covers the low-to-mid range; blue noise covers the mid-to-high range. Together they can create full-spectrum texture with independent level control over each half.
- Percussion synthesis where the desired character is crisp and bright, such as hi-hats and shakers, benefits from blue or violet noise as a starting point because less high-pass filtering is needed.

Why they are less commonly seen in synthesizers:

Most classic analog synthesizers only provided white noise, and digital synthesizers followed that convention. Blue and violet noise are straightforward to generate by applying a simple spectral shaping filter to white noise, but the demand for them has historically been low. As sound design becomes more detailed and layered, these noise colors become more relevant.

## Perlin Noise And Smooth Random

Perlin noise is a gradient-based coherent random function originally developed for procedural texture generation. Unlike white noise, which is uncorrelated between samples, or sample-and-hold noise, which produces discrete steps at regular intervals, Perlin noise produces gently undulating values that change smoothly and continuously. Each output value is influenced by its neighbors, creating a signal that wanders organically without sharp transitions or obvious repetition.

Why it matters as a modulation source:

- It creates slow, natural-feeling drift similar to analog component instability. Routing Perlin noise to oscillator pitch at a low depth produces the kind of subtle tuning variation that makes a digital oscillator feel alive.
- Subtle filter movement driven by Perlin noise creates timbral evolution that sounds organic rather than mechanical or periodic.
- Gentle waveshape or wavetable position modulation from Perlin noise produces gradual tonal shifts that evolve unpredictably over long durations.

How it differs from a slow random LFO:

A slow random LFO typically uses sample-and-hold with interpolation, which picks random target values at regular intervals and glides between them. This produces movement that, while smooth, still has a detectable rhythm because the interval between new targets is fixed. Perlin noise has no fixed period and no obvious repetition. Its rate of change varies continuously, so the resulting movement has no predictable pulse. This distinction is subtle but audible over sustained notes and long pad textures where periodic artifacts become noticeable.

Common mistakes:

Treating Perlin noise as a general-purpose audio source. Its spectral content is concentrated in low frequencies and it is not suitable as a replacement for white or colored noise in the audio path. Applying too much Perlin noise depth, which turns subtle organic drift into obvious pitch instability or filter wandering.

Controls typically exposed:

- Rate or speed, controlling how quickly the noise evolves. Lower rates produce slow glacial drift; higher rates produce faster wobble.
- Depth or amount, scaling the output range before it reaches the modulation destination.
- Smoothness, if the implementation allows adjusting the interpolation character.

Design implication:

Perlin noise or a similar smooth random function should be available as a dedicated modulation source, separate from the standard noise generators used in the audio path. It fills a role that no combination of LFO shapes and slew processing can replicate exactly. The modulation architecture should treat it as a per-voice source so that each note receives independent drift, preventing all voices from wandering in the same direction at the same time.

## Noise As Audio Source Versus Modulation Source

Noise serves two fundamentally different roles in a synthesizer, and the requirements for each role differ in important ways.

When noise is in the audio signal path, it contributes directly to the audible output. Audio-path noise needs full bandwidth up to the Nyquist frequency, careful level management to avoid dominating the mix, and the same sample-rate processing as any other audio signal. It passes through filters, effects, and the amplifier stage alongside oscillator output. Its spectral character matters because the listener hears it directly.

When noise is used as a modulation source, it controls parameters rather than producing sound itself. Modulation-path noise can often run at control rate rather than audio rate because most modulation destinations do not change meaningfully at every single sample. Band-limiting or smoothing the noise before it reaches a destination prevents zipper noise and erratic parameter jumps. Rate reduction is acceptable because the modulation target only needs updates at the rate the parameter can meaningfully change.

Why this distinction matters for design:

- A single noise generator may need different configurations depending on whether it is routed to an audio mixer input or to a modulation matrix slot.
- Audio-rate noise consumes more processing resources than control-rate noise. If every modulation noise source runs at audio rate unnecessarily, the cost adds up across voices and routings.
- Smoothing and rate reduction that improve modulation behavior would degrade audio-path noise by removing high-frequency content that the listener expects to hear.
- The interface should help users understand whether they are adding noise to the sound or using noise to move a parameter, because the results and the appropriate settings differ.

Common mistakes:

Using unfiltered audio-rate white noise as a modulation source for filter cutoff, which produces harsh, erratic movement. Using heavily smoothed control-rate noise as an audio source, which sounds dull and low-fidelity. Not providing separate level and routing controls for audio and modulation noise paths.

## Filtered Noise

Passing noise through a filter transforms its spectral character and can produce a wide range of musically useful textures that raw noise types cannot achieve on their own.

A low-pass filter applied to white noise progressively removes high-frequency content, producing results that approximate pink noise at gentle slopes and brown noise at steeper slopes. The filter cutoff becomes a timbral control that sweeps the noise from bright and hissy to dark and rumbling. A high-pass filter on white noise removes low-frequency energy, creating results similar to blue or violet noise depending on the slope. A band-pass filter on noise isolates a specific frequency region, producing focused textures that range from narrow whistling tones to wide breathy bands depending on the filter bandwidth.

Musical applications:

- Sweeping a low-pass filter on noise creates classic riser and fall effects used across electronic music genres.
- Resonant band-pass filtering of noise creates pitched noise bursts that are essential for synthesized percussion. Snare drums, hi-hats, claps, and metallic impacts all rely on filtered noise with specific frequency emphasis and envelope shaping.
- Narrow band-pass noise simulates wind, breath, and air sounds. Modulating the center frequency of the band-pass filter creates the impression of wind changing direction or intensity.
- Using the voice filter as the noise filter means that noise inherits the same timbral shaping applied to the oscillators, keeping the noise texture consistent with the overall patch character.

Why filtered noise is often more musically useful than raw noise:

Raw white noise contains energy across the entire spectrum, much of which may conflict with other elements in a mix. Filtering allows the sound designer to place noise energy precisely where it is needed, whether that is a narrow band of air around a vocal frequency, a low rumble beneath a bass, or a bright sizzle above a pad.

Common mistakes:

Using noise with no filtering at all, which often sounds harsh and uncontrolled. Assuming that selecting a different noise color is always better than filtering white noise, when in practice a filter provides continuous control over the spectral shape rather than a fixed set of options.

## Design Recommendations For Digital Synth

Audio-path noise sources:

- Include white, pink, and brown noise as selectable noise types in the audio signal path. These three cover the most common use cases and are expected by users.
- Add blue noise as a fourth option for variety. It fills the high-frequency niche that the other three do not cover and is inexpensive to generate from white noise.
- Noise should be available as a source alongside the pitched oscillators, with independent level control so it can be mixed into the voice at any proportion.

Modulation-path noise sources:

- Include Perlin noise or a comparable smooth random function as a dedicated modulation source. This should be distinct from the standard random LFO modes already described in the modulation documentation, because its lack of periodicity and its spatial coherence produce a character that interpolated sample-and-hold cannot match.
- Allow noise modulation sources to operate at control rate with appropriate smoothing, reducing processing cost without sacrificing modulation quality.

Routing flexibility:

- Allow any noise source to be routed as both an audio source and a modulation source through the modulation matrix. A user who wants white noise modulating filter cutoff at audio rate should be able to achieve that, even though control-rate noise is the default for modulation paths.
- Filtered noise should be achievable by routing noise through the voice filter rather than requiring a separate filtered-noise generator. This keeps the architecture modular and avoids duplicating filter logic.

Interface considerations:

- Label noise types clearly using both the color name and a brief spectral description so users unfamiliar with the color convention can understand what each type does.
- When noise is used as a modulation source, the interface should indicate the rate and smoothing behavior so users can predict how the modulation will feel before hearing it.
