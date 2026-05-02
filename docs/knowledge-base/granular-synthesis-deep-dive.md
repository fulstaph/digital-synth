# Granular Synthesis Deep Dive

Granular synthesis deconstructs audio into tiny fragments called grains and reassembles them into new textures, pitches, and temporal structures. Its defining capability is the independent control of time, pitch, density, and spatial position — dimensions that are tightly coupled in most other synthesis methods.

This document expands on the conceptual overview in synthesis-methods.md and provides a comprehensive treatment of granular synthesis as it relates to the Digital Synth project.

## Grain

A grain is a very short segment of audio, typically between 1 and 100 milliseconds.

Each grain is an independent sound event with its own pitch, amplitude, duration, envelope shape, source position, and pan. The grain is the atomic unit of granular synthesis: every texture, cloud, or pitched tone the method produces is built from the accumulation of many individual grains.

Grain duration is a critical parameter because it determines the perceptual character of the result.

Below about 10 milliseconds, individual grains are so short that the listener no longer hears them as separate events. Instead, the repetition rate of grains begins to define a perceived pitch. This is similar to how a buzzing string vibrating hundreds of times per second produces a pitched note rather than a series of clicks.

Between roughly 10 and 50 milliseconds, grains occupy a middle ground where they fuse into a textured mass but do not impose a strong pitch of their own. The source material's own pitch content comes through, and the overall result sounds like a texture or cloud.

Above about 50 milliseconds, grains become long enough that each one may be heard as a discrete event, especially at lower densities. The result can sound rhythmic, stuttered, or fragmented.

Because grain duration shifts the output between pitched, textured, and rhythmic territory, it is one of the first parameters a user should understand. A synthesizer that exposes grain duration as a simple knob is giving the user control over a fundamental perceptual boundary.

Modulating grain duration over time can move a sound from a smooth cloud into a rhythmic stutter and back again.

Short grains also contain less spectral information from the source. A 5-millisecond grain captures only a tiny snapshot of the source waveform, while a 100-millisecond grain captures a musically meaningful segment. This tradeoff between temporal resolution and spectral resolution is inherent to granular synthesis and cannot be eliminated, only managed.

## Grain Envelope

The grain envelope is the amplitude shape applied to each grain over its duration.

Without an envelope, a grain is simply a rectangular slice of audio: it begins abruptly, plays at full level, and ends abruptly. These hard edges produce clicks at the grain boundaries. When hundreds of grains overlap, the accumulated clicks create a harsh, noisy artifact that usually overwhelms the intended texture.

Common envelope shapes:

- Gaussian: a smooth bell curve. The grain fades in from silence, reaches peak amplitude near its center, and fades back to silence. This produces the fewest boundary artifacts and creates the smoothest textures.

- Raised cosine: similar in smoothness to Gaussian but with a slightly different spectral character due to its mathematical shape. In practice, the difference is subtle, but it can matter when many grains overlap and their spectral contributions accumulate.

- Trapezoid: a flat top section with tapered edges at the start and end. This preserves more of the grain's center content at full amplitude while still tapering the boundaries to avoid clicks. Trapezoid envelopes can sound slightly more present or forward than Gaussian envelopes because more of each grain is heard at full level.

Why envelope shape matters for overlap:

A Gaussian envelope means that grains are at low amplitude for a significant portion of their duration, so adjacent grains must overlap substantially to avoid amplitude dips. A trapezoid envelope maintains full amplitude for longer, requiring less overlap to fill gaps.

The choice of envelope shape should be exposed as a user control, but it can also have a reasonable default. Gaussian is the safest default because it minimizes artifacts in the widest range of settings.

## Grain Density And Overlap

Density is the number of grains produced per second. It is the primary control for moving between rhythmic grain events and continuous sound.

At low density, below roughly 10 grains per second, individual grains are audible as separate events. The listener hears a series of short bursts, and the result has a rhythmic or pointillistic character.

At moderate density, between roughly 10 and 50 grains per second, the grains begin to fuse into a continuous texture. Individual events become harder to distinguish, and the overall impression shifts from rhythm to mass.

At high density, above roughly 50 grains per second and potentially reaching 200 or more, the result becomes a continuous sound whose timbral character depends on the other grain parameters.

Overlap:

Overlap occurs when a grain's duration is longer than the interval between successive grain onsets. If grains are 50 milliseconds long and new grains begin every 20 milliseconds, each grain overlaps with its neighbors.

High overlap creates smoother textures because the amplitude contributions of many grains blend together, filling any gaps. Low or no overlap can create a pulsing or stuttering effect, especially if the grain envelope tapers to silence before the next grain begins.

Interaction between density and duration:

Doubling the density without adjusting grain size doubles the number of simultaneous overlapping grains, which increases both CPU load and amplitude accumulation. A synthesizer should provide gain compensation or normalization to prevent the output from clipping as density increases.

Musical significance:

Density lets a performer sweep from sparse, rhythmic textures to dense, continuous clouds using a single control. Modulating density with an LFO or envelope can create evolving textures that breathe between sparse and thick.

## Source Position And Scanning

Each grain reads its audio content starting from a position in the source material. This source position determines which part of the recording each grain captures.

Normal scanning:

Scanning forward through the source at the original speed recreates the original playback, with each successive grain picking up where the last one left off.

Slow scanning:

Scanning at half speed stretches time. The listener hears the source material play back at half its original tempo, but because each grain is individually pitched, the pitch does not change. This is the foundation of granular time-stretching.

Frozen position:

Freezing the source position means every grain reads from the same location. The result is a sustained version of a single spectral moment from the source.

The character of the sustained sound depends on the grain parameters: short grains at high density create a buzzing tone, while longer grains at moderate density create a shimmering, evolving texture even from a static source position. Frozen position is one of the most distinctive effects granular synthesis can produce and has no equivalent in traditional oscillator-based synthesis.

Random position offsets:

Random offsets add variation by scattering each grain's read position around the current scan point. Small offsets create subtle timbral movement. Large offsets pull grains from distant parts of the source, creating a collage effect that can range from interesting textural variety to complete incoherence depending on the source material and offset range.

Why source position matters:

Source position control is what makes granular synthesis fundamentally different from simple sample playback. In a conventional sampler, position, pitch, and time are locked together. Granular synthesis breaks that lock, making position an independent, modulatable parameter.

## Time-Pitch Decoupling

The defining capability of granular synthesis is its ability to change playback speed without changing pitch, and to change pitch without changing playback speed. These two dimensions, which are inseparable in traditional sample playback, become independent controls.

Time-stretching:

Time-stretching is achieved by scanning the source position slowly while keeping each grain's playback rate at the original pitch. The overall progression through the source material slows down, but each individual grain reproduces its slice of audio at the correct pitch.

Extreme time-stretching factors, such as ten or a hundred times slower, produce ambient textures where brief source moments become extended sonic environments.

Pitch-shifting:

Pitch-shifting is achieved by changing the playback rate of each individual grain while keeping the scan speed constant. Each grain is transposed up or down, so the listener hears a pitch-shifted version of the source that progresses through time at the original speed.

The quality of pitch-shifting depends on grain size and envelope: small pitch shifts sound natural, while large shifts begin to introduce artifacts because the grain content no longer matches smoothly with its neighbors.

Combined control:

Combined control of time and pitch allows effects that are impossible with traditional oscillators or sample playback:

- A sound can be frozen in time while its pitch sweeps across octaves.
- A melody can be slowed to a crawl while every note retains its original pitch.
- A drum loop can be pitch-shifted down an octave without halving its tempo.

This decoupling is the central musical reason granular synthesis exists and should be clearly communicated to users through both documentation and interface design.

## Grain Scheduling Modes

Grain scheduling determines how the timing of grain generation is organized. The scheduling mode affects whether the result sounds mechanical, organic, or responsive.

Synchronous:

Synchronous scheduling produces grains at regular, evenly spaced intervals. If the density is 100 grains per second, a new grain begins exactly every 10 milliseconds. This regularity creates a predictable pulse. At moderate densities, the regularity can be heard as a subtle rhythmic pattern or buzzing quality.

Synchronous scheduling is computationally straightforward and makes it easy to predict overlap and CPU load. However, the regularity can sound mechanical or introduce audible periodicity artifacts.

Asynchronous:

Asynchronous scheduling randomizes the timing of grain generation within a range defined by the density parameter. Instead of producing a grain exactly every 10 milliseconds, the system produces grains with randomized intervals that average to the target density.

This creates a more organic, cloud-like texture because the precise timing of each grain is unpredictable. Asynchronous scheduling is generally preferred for ambient and textural applications because it avoids the rhythmic artifacts of synchronous mode.

Triggered:

Triggered scheduling produces grains in response to external events rather than a continuous stream. A note-on message might trigger a burst of grains. An envelope threshold might start or stop grain production. An external clock might synchronize grain generation to a musical tempo.

Triggered scheduling makes granular synthesis responsive to performance and sequencing, connecting it to the rest of the instrument's musical behavior.

A synthesizer should ideally support all three modes, as they serve different musical purposes.

## Spatial Distribution

Each grain can be independently positioned in the stereo field. This spatial control is one of granular synthesis's most immediate and distinctive qualities.

When grains are assigned random pan positions across the stereo field, the result is an immersive, spacious texture where the sound seems to come from everywhere at once. Even a monophonic source recording can be transformed into a wide stereo image through granular spatial distribution.

Spread control:

The spread parameter determines the width of the random pan range. At full spread, grains may appear anywhere from hard left to hard right. At zero spread, all grains are panned to center, collapsing the spatial image into a mono point source. Intermediate spread values create a controllable width that can be modulated over time.

Other spatial modes:

- Fixed pan assignment places every grain at the same stereo position. Useful when granular processing should occupy a specific location in a mix.
- Panning that follows source position can preserve spatial information from stereo source material.
- Panning modulated by an LFO or envelope can create sweeping spatial movement.

Interaction with density:

At low density, random panning creates discrete events that bounce across the stereo field. At high density, random panning fuses into a wide, enveloping texture. Density and spatial controls together shape the perceived size and movement of the granular output.

For the Digital Synth project, spatial distribution is especially relevant to the learning environment goal. Visualizing grain positions in a stereo field can help users understand both what granular synthesis is doing and how stereo imaging works in general.

## Granular As Source Versus Effect

Granular processing can operate in two fundamentally different roles within a synthesizer architecture.

As a sound source:

Granular synthesis replaces the oscillator stage, reading from stored samples or buffers and producing the primary audio material that then flows through filters, effects, and the rest of the signal chain.

A sample is loaded, and the granular engine scans through it, producing grains that become the voice's audio output. This is analogous to how a wavetable oscillator reads from stored waveforms, but with the added dimensions of grain scheduling, density, and time-pitch decoupling.

As an effect:

Granular processing receives audio from another source — an oscillator, a microphone, a different synthesis engine — and transforms it by breaking it into grains and reassembling them.

A subtractive oscillator's output can be fed into a granular processor, creating hybrid textures that combine the harmonic character of the oscillator with the temporal manipulation of granular processing. Live audio input allows real-time granular transformation of external signals, which is musically powerful for performance and sound design.

Architecture implication:

Both roles should be considered in the Digital Synth architecture. The granular engine should be designed so that its source can be either a stored buffer or a live audio stream without requiring a fundamentally different signal path. This flexibility allows granular to function as an oscillator replacement in one patch and as an insert effect in another.

## Parameter Interaction

Granular synthesis parameters are highly interdependent. This interdependence is both the method's greatest strength and its primary challenge for interface design.

Grain size and density:

These interact to determine overlap. If grain size is 50 milliseconds and density is 40 grains per second (one grain every 25 milliseconds), each grain overlaps with its successor by roughly half its duration. Doubling the density to 80 grains per second means each grain overlaps with three or four neighbors simultaneously. The texture becomes thicker, louder, and more continuous, but CPU load also increases.

Grain size and pitch:

At short durations, when grains are shorter than about 10 milliseconds, the repetition rate of grains begins to impose its own pitch. Changing grain size in this range changes the perceived pitch independently of any pitch-shift setting. This can be musically useful but is often confusing for users who expect pitch control to come only from the pitch parameter.

Source position speed and density:

Fast scanning with low density means each grain reads from a noticeably different source position, creating a fragmented collage. Slow scanning with high density means many grains read from nearly the same position, creating a thick, focused texture.

Envelope shape and overlap:

Gaussian envelopes require higher overlap to avoid amplitude dips between grains. Trapezoid envelopes tolerate lower overlap because their flat top section maintains level.

Design consequence:

A well-designed interface should help users understand these interactions, either through documentation, tooltips, visual feedback, or carefully scaled parameter ranges that make it difficult to reach settings that produce silence or noise by accident.

## Common Granular Mistakes

Several common mistakes produce poor results in granular synthesis and should be anticipated in both documentation and interface design.

Density too low for grain size:

Setting density too low for the current grain size creates gaps between grains. If grains are short and spaced far apart, the output is a series of clicks or pops separated by silence. Users may not realize that increasing grain size or density will fill the gaps. A synthesizer can mitigate this by displaying a visual indicator of grain coverage or by warning when settings are likely to produce gaps.

No grain envelope:

Using rectangular grains (no envelope) produces clicking at every grain boundary. When hundreds of rectangular grains overlap, the result is a harsh, noisy texture dominated by click artifacts rather than the source material. Rectangular envelopes are almost never desirable, and the default should always be a smooth envelope shape.

Excessive pitch shift:

Applying large pitch shifts without understanding their limitations can produce poor results. Shifting pitch up by an octave means each grain plays at double speed, completing in half its nominal duration. Large downward shifts stretch each grain, potentially revealing artifacts. Users may expect pitch-shifting quality comparable to dedicated pitch-correction tools, but granular pitch-shifting is a different process with different tradeoffs.

Random position offsets too wide:

Setting random position offsets too wide relative to the source material's length makes the output a disjointed collage of unrelated source fragments. This can be an intentional creative effect, but users who are trying to time-stretch a phrase may not realize that wide position randomization is destroying the phrase's continuity.

CPU cost at high density:

Each active grain requires its own read position, envelope calculation, and mixing. At densities of several hundred grains per second with long grain durations, the number of simultaneously active grains can become large enough to strain real-time performance. The architecture should set reasonable upper limits or provide feedback about CPU load.

## Design Recommendations For Digital Synth

Granular synthesis should be introduced as a later expansion after subtractive and wavetable engines are established. The subtractive core defines the voice, modulation, filter, and effects architecture. The wavetable engine validates that the architecture can support a different oscillator type. Granular synthesis is the third engine and the first to introduce fundamentally new concepts like grain scheduling and time-pitch decoupling.

Dual role:

Granular synthesis should be available both as a sound source (replacing the oscillator in a voice) and as an effect (processing audio from other sources or external input). The architecture should allow a voice to use a granular source that feeds into the same filter, amplifier, and effects chain used by subtractive and wavetable voices.

Modulation integration:

Grain parameters — density, size, source position, pitch, envelope shape, and spatial spread — should be modulatable from the same modulation matrix used by other synthesis methods. LFOs, envelopes, velocity, aftertouch, mod wheel, and macros should all be available as modulation sources for granular parameters. Reusing the existing modulation infrastructure avoids duplicating design work and ensures a consistent user experience across synthesis methods.

Visualization:

Visualization is especially important for granular synthesis because the process is abstract and the parameter interactions are complex. A display showing grain activity, source position, density, and spatial distribution would strongly support the learning environment goal. Users should be able to see grains appearing, moving through the source, and spreading across the stereo field. This makes the synthesis method tangible rather than mysterious.

Coexistence with other engines:

The architecture should allow granular synthesis to coexist with other sources within the same voice or as a global processor. A voice might use a subtractive oscillator and a granular layer simultaneously. A global granular effect might process the mixed output of multiple voices. These configurations require that the granular engine is designed as a modular component rather than a standalone instrument.
