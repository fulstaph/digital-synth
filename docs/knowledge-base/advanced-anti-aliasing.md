# Advanced Anti-Aliasing

The existing knowledge base establishes that aliasing is a core quality concern in digital synthesis. Whenever a generated signal contains energy above the Nyquist frequency, that energy folds back into the audible range as inharmonic content that was never intended.

This document explores the design-level strategies for preventing aliasing. Understanding these strategies is necessary for making informed architecture decisions about sound quality, processing cost, and which tradeoffs are acceptable for a given use case. Each strategy has a different cost profile, a different quality ceiling, and different applicability to various synthesis methods.

## The Naive Oscillator Problem

A naive digital oscillator constructs a waveform by computing sample values directly from a mathematical formula. For a sawtooth wave, the value ramps linearly from one extreme to the other, then resets sharply. For a square or pulse wave, the value holds at one level, then jumps to another. These sharp transitions, the vertical reset of the sawtooth and the hard edges of the square, contain an enormous amount of harmonic energy that extends theoretically to infinity.

In theory, an ideal sawtooth contains every integer harmonic of the fundamental frequency, with amplitude decreasing only as one over the harmonic number. An ideal square wave contains every odd harmonic on a similar slope. At any finite sample rate, only a limited number of those harmonics can be represented correctly. The rest do not vanish. They fold back below the Nyquist frequency and appear as new frequencies that have no harmonic relationship to the fundamental.

Consider a sawtooth at 1000 Hz running at a 48 kHz sample rate. The Nyquist limit is 24 kHz, so only the first 24 harmonics can be represented. The 25th harmonic would land at 25 kHz, but instead of being silently discarded, it reflects back and appears at 23 kHz. The 26th harmonic appears at 22 kHz, and so on.

These reflected frequencies are not musically related to the original pitch. They form an inharmonic series that clashes with the intended tone. The folded-back frequencies are mirror images of the originals about the Nyquist frequency, and they move in the opposite direction when the pitch changes, which is why aliased artifacts shift unpredictably as notes move up and down.

The audible result is often described as metallic, harsh, glassy, or dirty. These aliased components do not track pitch correctly: they move in unexpected directions as the note changes, creating a characteristic inharmonic shimmer that distinguishes a cheap digital oscillator from a well-designed one.

When a musician plays a scale on a naive oscillator, the aliased artifacts shift unpredictably from note to note, producing an inconsistent timbre that no amount of filtering or mixing can fully correct after the fact. The aliased frequencies are already baked into the signal. A low-pass filter cannot separate them from the desired harmonics because they sit within the same frequency range.

The problem worsens dramatically as pitch rises because higher notes leave less room between their fundamental and Nyquist. A naive sawtooth at C6 (roughly 1047 Hz) can only fit about 23 clean harmonics at 48 kHz. At C7, that number drops to roughly 11. By C8, the fundamental itself is so high that very few harmonics fit at all, and those that fold back are loud enough relative to the intended harmonics to be clearly audible even in a mix.

This pitch-dependent severity means that aliasing is hardest to hear during casual testing, which typically uses comfortable mid-range notes, and most obvious during real musical performance, which often explores the full range of the keyboard.

Pulse and square waves share the same fundamental problem. Their transitions are just as sharp, and while they contain only odd harmonics, those harmonics still extend above Nyquist at moderate pitches. Any waveform with a discontinuity, meaning any sudden jump in value, will produce aliasing when generated naively.

Waveforms without discontinuities are far less affected. A sine wave produces no aliasing at any pitch because it contains only a single harmonic: the fundamental. A triangle wave has no value discontinuities, only slope discontinuities at its peaks and troughs, so its aliasing is much milder than that of a sawtooth or square. Understanding which waveform features cause aliasing, and how severely, is the foundation for choosing the right anti-aliasing strategy.

This is not a subtle academic concern. Aliasing in oscillators is one of the most immediately audible quality differences between synthesizers, and any serious design must address it before the oscillator output reaches the rest of the signal chain.

## Oversampling

Oversampling means processing audio at a sample rate higher than the final output rate. If the output runs at 48 kHz, a 2x oversampled stage runs internally at 96 kHz. A 4x oversampled stage runs at 192 kHz. After processing, the signal is low-pass filtered to remove content above the true output Nyquist, then reduced back to the output sample rate. This reduction step is called decimation.

Together, the upsampling, processing, and decimation form a self-contained pipeline that can wrap any stage in the signal chain. The idea is general purpose: any processing step that might create aliasing can be isolated inside an oversampled pipeline without affecting the rest of the signal chain. From the perspective of the surrounding stages, the oversampled section simply accepts input at the output rate and produces output at the output rate. The internal rate multiplication is invisible.

Why this works: raising the internal sample rate raises the internal Nyquist frequency proportionally. At 4x oversampling, the internal Nyquist is 96 kHz instead of 24 kHz. Harmonics that would have folded back at the output rate now have four times as much frequency space before they encounter a boundary.

The decimation filter, sometimes called the anti-aliasing filter, removes any remaining content above the true output Nyquist before the signal returns to the output rate. The result is a signal that is clean of aliased components, as if the processing had been done at infinite resolution and then carefully limited.

The primary tradeoff is processing cost. A 4x oversampled stage requires four times as many sample computations for every operation inside the oversampled section. An 8x stage requires eight times as many. This cost applies to every calculation within the oversampled portion of the signal chain, not just the filter at the end. For a complex effect with many internal operations, this multiplication can be substantial.

The decimation filter itself also has a cost, and its quality matters significantly. A filter that is too gentle will allow some aliased content through the transition band, partially defeating the purpose of oversampling. A filter that is too aggressive will dull the signal, rolling off legitimate content near the top of the audible range.

Designing a decimation filter that is sharp enough to reject aliasing but transparent enough to preserve brightness is a meaningful engineering concern. The transition band, the frequency range between the passband and the stopband, should be as narrow as practical. A wider transition band forces a choice between letting aliasing through or losing brightness. Half-band filters, which exploit the symmetry of the 2x oversampling case, are a common efficient choice for the decimation stage and can be cascaded for higher oversampling factors.

Oversampling is most valuable where new harmonic content is created unpredictably. Nonlinear processing stages such as distortion, saturation, waveshaping, and wavefolding are the primary candidates because they generate harmonics that cannot be predicted from the input spectrum alone. Oversampling the oscillator itself is possible but usually less efficient than using band-limited generation methods directly, since the oscillator's spectrum is deterministic and can be controlled at the source.

Common oversampling factors are 2x, 4x, and 8x. Higher factors provide more headroom but with diminishing returns: the jump from no oversampling to 2x typically produces the largest quality improvement, while the jump from 4x to 8x may be difficult to hear in most musical contexts. The choice of factor should be guided by the severity of the harmonic generation at the stage being oversampled.

It is worth noting that oversampling also increases the latency of the oversampled stage by the length of the decimation filter. For stages in the main audio path, this latency adds to the total input-to-output delay. For offline rendering this does not matter, but for real-time performance it is a factor to consider.

The upsampling stage also has a filter, called the interpolation filter, which fills in the new samples between the original ones. This filter's quality affects how accurately the signal is represented at the higher rate. A naive upsampler that inserts zeros between existing samples and then filters will work, but more sophisticated approaches can reduce the cost of the interpolation filter while maintaining accuracy.

Design implication: oversampling should be treated as a targeted tool applied to specific processing stages rather than a blanket setting applied to the entire signal path. Applying it where it is not needed wastes processing budget that could be spent on more voices, more effects, or lower latency.

## Band-Limited Wavetables

A band-limited wavetable stores pre-computed waveform cycles at multiple levels of harmonic content. Each level represents the same basic waveshape but with progressively fewer harmonics. The oscillator selects the appropriate level based on the current pitch: at low pitches where many harmonics fit below Nyquist, it reads from a richly detailed table. At high pitches where only a few harmonics are safe, it reads from a table that has been filtered to contain only those harmonics.

This approach is sometimes called mip-mapping, borrowing the term from texture rendering in computer graphics, where the same idea solves an analogous problem. Just as a texture viewed from a distance needs fewer detail levels to avoid visual aliasing, a waveform played at a high pitch needs fewer harmonics to avoid audio aliasing. The underlying mathematics are closely related: both domains deal with sampling a continuous signal at a finite resolution and managing the consequences when detail exceeds that resolution.

The number of mip-map levels needed depends on the pitch range the oscillator must cover. A typical approach creates one level per octave, since each octave doubles the fundamental frequency and roughly halves the number of harmonics that fit below Nyquist. Some designs use finer divisions, creating a level every semitone or every few semitones, for smoother transitions between levels.

The finer the division, the smoother the timbral change as pitch varies, but the greater the memory requirement. One level per octave is a common starting point because it provides a good balance between quality and simplicity. Crossfading between levels handles the in-between pitches.

Band-limited wavetables work well for static periodic waveforms. Classic shapes like sawtooth, square, triangle, and pulse can be pre-computed at many mip-map levels with very high accuracy. Wavetable synthesis, where the player scans through a set of stored waveshapes, also benefits directly from this approach because each frame in the wavetable can be stored at multiple band-limited resolutions. This makes wavetable oscillators inherently compatible with the mip-mapping strategy.

The limitation is that band-limited wavetables assume the waveform shape is known in advance and does not change dynamically. They do not handle waveforms that change continuously in unpredictable ways. The output of a frequency-modulation oscillator, the result of hard sync, or a waveform being actively morphed by a modulation source cannot be pre-computed into a static table. These cases require different anti-aliasing strategies.

For waveform parameters that change slowly, such as a wavetable position being swept by an LFO, the mip-map approach still works because the oscillator can crossfade between adjacent table frames at the appropriate band-limited level. The approach breaks down only when the waveform changes at audio rate or in ways that are not indexed by a simple position parameter.

Band-limited wavetables are also useful as a reference for evaluating other anti-aliasing methods. Because the tables can be constructed to be provably alias-free, they provide a ground truth against which PolyBLEP, MinBLEP, or oversampling outputs can be compared.

Interpolation between table entries is important. The oscillator needs to interpolate smoothly in two dimensions: between adjacent samples within a table to produce accurate inter-sample values, and between adjacent mip-map levels to avoid timbral jumps when the pitch crosses a level boundary. Without both forms of interpolation, the oscillator can exhibit audible discontinuities as notes move across the keyboard.

The interpolation method used for reading between samples, whether linear, cubic, or higher-order, affects the tonal quality of the oscillator. Linear interpolation is cheapest but can dull the highest harmonics by acting as a subtle low-pass filter. Higher-order interpolation preserves more harmonic detail but costs more per sample. The choice of interpolation method is a secondary quality factor after the mip-map structure itself, but it is audible and worth considering.

Memory use is a secondary concern. Storing many mip-map levels for many waveforms requires more memory than a single table per shape, but the amount is modest by modern standards and the quality benefit is substantial. A typical wavetable with 10 octave levels, each containing a cycle of 2048 samples, uses a small fraction of available memory even on constrained systems. The memory cost scales linearly with the number of distinct waveforms, making it predictable and easy to budget.

Design implication: band-limited wavetables should be the default consideration for any oscillator that uses a fixed or slowly changing periodic waveform. They are efficient at runtime, alias-free by construction, and well suited to the standard waveforms that a subtractive synthesizer needs.

## Band-Limited Step Functions

Aliasing in classic waveforms comes from a specific structural feature: sharp discontinuities. The vertical reset of a sawtooth, the hard transitions of a square wave, and the abrupt edges of a pulse wave are all step functions, meaning instantaneous jumps from one value to another. In continuous mathematics these jumps have infinite bandwidth. In a sampled system, that infinite bandwidth folds into aliasing.

The band-limited step function approach replaces each discontinuity with a version of the same transition that contains no energy above Nyquist. Instead of an instantaneous jump, the transition follows a shape derived from a sinc function, which is the ideal band-limited impulse. The sinc function is the shape that results from perfectly limiting a signal's bandwidth: it has a main lobe centered at the transition point and diminishing side lobes extending outward in both directions.

The result is a step that rises and settles with a controlled ringing pattern, where the ringing contains only frequencies below Nyquist. The oscillation on either side of the transition is not a defect. It is the mathematically necessary consequence of removing all energy above Nyquist while preserving the step's total change in value. This ringing is sometimes called Gibbs phenomenon, and its amplitude and duration depend on the bandwidth limit and the windowing applied to the sinc function.

This is the conceptual foundation of the BLEP family of techniques. BLEP stands for band-limited step. The idea is: generate the waveform naively, then correct each discontinuity by subtracting the naive step and adding the band-limited step in its place. The difference between the naive step and the band-limited step is called the residual, and it is this residual that is subtracted from the output signal. The correction cancels the aliased energy without changing the intended musical content of the waveform.

A related concept is BLAMP, which stands for band-limited ramp. Where BLEP handles discontinuities in value (jumps), BLAMP handles discontinuities in slope (sharp corners). A triangle wave, for example, has no value discontinuities but has slope discontinuities at its peaks and troughs. Applying BLAMP corrections to these corner points reduces aliasing in waveforms that are continuous but not smooth. The combination of BLEP and BLAMP can address aliasing in a wide range of standard waveform shapes.

The elegance of the step-function approach is that it targets aliasing at its source. Rather than filtering the entire signal or raising the sample rate globally, it addresses only the specific moments where aliasing is generated. This makes it both efficient and precise. The processing cost is proportional to the number of discontinuities per cycle, not the sample rate. A sawtooth wave has one discontinuity per cycle. A square wave has two. The cost scales with this count, not with the sample rate or the pitch of the note.

The challenge is that computing a true band-limited step requires an ideal sinc function, which is infinite in extent. A perfect correction would need to modify an infinite number of surrounding samples, which is not practical in a real-time system. All real-world BLEP methods are approximations that truncate, window, or reshape the sinc to make it finite and computationally feasible.

The quality of the approximation depends on how closely it matches the ideal sinc in the frequency domain. A closer match means better alias suppression but more computation or memory. A rougher match means lower cost but more residual aliasing. The different variants described below represent different strategies for this approximation, each with its own balance of quality, cost, and complexity.

The BLEP family is particularly well suited to classic analog-style waveforms because these waveforms have a small, known number of discontinuities per cycle. For waveforms where the number or position of discontinuities is unpredictable, such as the output of oscillator sync with a swept slave frequency, applying BLEP corrections becomes more involved but is still feasible if each discontinuity can be detected and located within the sample period.

The step-function perspective is also valuable for understanding why some synthesis methods produce more aliasing than others. Methods that create more discontinuities per cycle, or discontinuities with larger magnitude, will produce more aliasing and require more aggressive anti-aliasing treatment. Hard sync, for example, forces the slave oscillator to reset at the master's frequency, creating additional discontinuities that must each be corrected.

## PolyBLEP

PolyBLEP, short for polynomial band-limited step, is a practical and computationally efficient approximation of the band-limited step concept. Rather than computing a full sinc-based correction, PolyBLEP uses a small polynomial curve that is applied to the one or two samples immediately surrounding each discontinuity in the waveform.

The correction works by replacing the sharp corner of the naive step with a smooth polynomial curve that approximates the behavior of a true band-limited transition. The polynomial is chosen so that it integrates to the correct total step size, preserving the overall waveform shape while removing most of the energy that would cause aliasing.

Because the polynomial only touches the samples directly adjacent to the discontinuity, its computational cost per cycle is minimal. No lookup tables are needed. No convolution is required. The correction is a few arithmetic operations applied at one or two points in each waveform period. This makes PolyBLEP one of the cheapest anti-aliasing methods available, costing almost nothing compared to the oscillator's own waveform computation.

PolyBLEP is a common practical choice for several reasons. Its processing cost is very low because the correction is applied only at discontinuity points, which occur once or twice per waveform cycle. It is straightforward to apply to standard waveforms: a sawtooth needs correction at its reset point, a square wave at both its rising and falling transitions, and a pulse wave at both edges. Adding PolyBLEP to an existing naive oscillator requires relatively little structural change to the oscillator design.

The method also needs to account for where the discontinuity falls between samples. In most cases, the exact transition point does not land precisely on a sample boundary. It falls somewhere between two sample instants. The fractional position of the transition within the sample period affects the shape of the polynomial correction. A well-designed PolyBLEP adjusts its correction based on this fractional offset, which improves quality without adding significant cost.

PolyBLEP produces good quality for the vast majority of musical applications and listening conditions. It is also straightforward to combine with continuously variable parameters. Pulse width modulation, for example, moves the position of the discontinuity within the cycle, and the PolyBLEP correction adapts naturally to wherever that discontinuity falls. This makes PolyBLEP especially suitable for waveforms whose shape is controlled by a modulation source.

A related technique, PolyBLAMP (polynomial band-limited ramp), applies the same principle to slope discontinuities. Where PolyBLEP smooths value jumps, PolyBLAMP smooths corner points. Combining PolyBLEP for sawtooth and square waves with PolyBLAMP for triangle waves provides coverage for the full set of standard analog waveforms at a uniformly low cost.

The simplicity of PolyBLEP also makes it easy to verify. Comparing the spectrum of a PolyBLEP oscillator against a known-clean reference at various pitches quickly reveals how much alias suppression the method provides and where its limits lie. This kind of straightforward verification is harder with more complex methods.

The quality tradeoff is that PolyBLEP is an approximation with a very short correction span. At very high fundamental frequencies, where the correction spans a significant fraction of the waveform period, PolyBLEP may not suppress aliasing as completely as a full BLEP or a well-constructed band-limited wavetable. In practice, this matters most at the extreme top of the keyboard and is often inaudible in a musical context.

For an initial subtractive oscillator design, PolyBLEP represents a strong balance between quality, simplicity, and cost. It can be added to a naive oscillator with minimal refactoring, and the quality improvement is immediately audible. If higher quality is needed later, the oscillator can be upgraded to MinBLEP or supplemented with band-limited wavetables without changing the rest of the signal chain.

## MinBLEP

MinBLEP stands for minimum-phase band-limited step. It shares the same core idea as standard BLEP, replacing naive discontinuities with band-limited versions, but it differs in a significant structural way: the correction is causal. This means the band-limited step affects only samples at and after the discontinuity, not samples before it.

A standard sinc-based BLEP is symmetric around the discontinuity, meaning it modifies samples both before and after the transition point. This symmetry is called linear phase because the correction does not shift the relative timing of different frequency components. It is mathematically clean but introduces pre-ringing: subtle oscillation appears in the waveform slightly before the intended discontinuity actually occurs.

Pre-ringing is a known artifact of linear-phase band-limiting. While it is technically alias-free, it can impart a slightly unnatural quality, particularly in transient-rich material. The ear can perceive it as a faint anticipation or smearing before the attack of a sound.

In analog synthesizers, discontinuities happen with no advance warning: the sawtooth resets abruptly, the square wave switches state instantly. There is no energy before the transition that hints at its arrival. Pre-ringing is therefore a distinctly digital artifact that can undermine the goal of analog-like oscillator character.

MinBLEP avoids this by converting the sinc-based correction to its minimum-phase equivalent. The mathematical process preserves the magnitude spectrum of the original BLEP kernel but redistributes its energy so that none appears before the transition point. The alias-suppression quality of a full BLEP is preserved while the pre-ringing is eliminated entirely.

In practice, the waveform remains flat and undisturbed until the discontinuity occurs, then the correction settles naturally over the following samples. The length of this settling depends on the table size: longer tables provide better alias suppression but spread the post-transition ringing over more samples.

The table length is a design choice that balances quality against memory and per-discontinuity processing cost. A table of 32 to 64 samples is typical and provides good alias suppression for most musical purposes.

The tradeoff is that MinBLEP requires a pre-computed correction table, which must be generated carefully for the target sample rate. If the synthesizer supports multiple sample rates, separate tables may be needed for each, or a single high-quality table can be resampled at initialization.

Applying the correction involves convolving each discontinuity with this table, which is more work per discontinuity than a PolyBLEP polynomial but still efficient because it only applies at the specific moments where discontinuities occur, not at every sample. The convolution length is fixed and short, typically a few dozen samples. Because the table is the same for every discontinuity, it can be kept in fast memory and accessed efficiently.

MinBLEP does introduce a brief post-ringing after the transition, but this tends to sound more natural than pre-ringing because it follows the event that caused it rather than preceding it. The post-ringing is the minimum-phase system's way of settling into the new value while respecting the Nyquist limit.

For a synthesizer that aims for clean, analog-like oscillator character, MinBLEP is a strong candidate. It offers better alias suppression than PolyBLEP at high frequencies while maintaining a natural, punchy transient quality that suits musical use.

The choice between PolyBLEP and MinBLEP is not necessarily exclusive. A synthesizer could use PolyBLEP as its default for efficiency and offer MinBLEP as a high-quality option, or use PolyBLEP in a draft quality mode and MinBLEP in a standard or high mode. The oscillator interface should be designed so that the anti-aliasing method can be swapped without changing how the rest of the voice pipeline interacts with the oscillator output.

## Oversampling For Nonlinear Processing

Even if every oscillator in the synthesizer is perfectly band-limited, aliasing can reappear the moment the signal passes through a nonlinear function. Waveshaping, distortion, saturation, clipping, wavefolding, and filter drive are all nonlinear operations. Their defining characteristic is that they change the shape of the waveform in amplitude-dependent ways, and any change to waveform shape creates new harmonics that were not present in the input.

A sine wave passed through a hard-clipping function, for example, emerges with odd harmonics extending far above the input frequency. A rich sawtooth passed through a wavefolder generates dense sidebands that can easily exceed Nyquist. The band-limited nature of the input signal does not protect against this because the nonlinear function itself creates new spectral content that the original band-limiting could not anticipate.

The severity of the aliasing depends on how aggressive the nonlinear function is. Gentle saturation curves add relatively few high harmonics and may produce only mild aliasing that is difficult to hear. Hard clipping or wavefolding can generate harmonics extending many times above the input frequency, creating severe and clearly audible aliased content. The more the waveform shape is distorted, the more new harmonics are created, and the more oversampling is needed to prevent them from folding back.

The standard design-level solution is to oversample the nonlinear stage specifically. The signal is upsampled before entering the distortion, waveshaper, or saturator. It is then processed at the higher rate where Nyquist sits far above the newly created harmonics. Finally, it is decimated back to the output rate with a low-pass filter that removes everything above the true Nyquist.

This matters for several common synthesizer features. Filter drive, where the filter's resonance or input gain pushes the signal into nonlinear behavior, is a frequent and often overlooked source of aliasing. Many classic filter designs become nonlinear at high drive settings, and without oversampling, the driven filter will generate aliased harmonics that make the resonance sound harsh rather than warm.

Distortion effects applied after the filter are another common source because they take the already-shaped signal and further modify its waveform. Waveshaping oscillators that use transfer functions to sculpt the waveform generate extreme harmonic content by design and almost always require oversampling to sound clean.

The amount of oversampling needed depends on the nonlinearity's severity. Soft saturation may sound acceptable with 2x oversampling. Hard clipping, aggressive wavefolding, or bitcrushing may need 4x or 8x to adequately suppress the aliased content.

The design should allow different oversampling factors for different stages based on their harmonic generation behavior. A gentle tube-saturation model and a hard digital clipper have very different oversampling needs, and forcing them to share the same factor wastes processing on the gentle stage or under-protects the aggressive one.

In each case, oversampling the specific nonlinear stage is more efficient than oversampling the entire signal chain. Only the stage that creates new harmonics needs the expanded frequency headroom. The upsampling and decimation filters add some cost, but this cost is confined to the nonlinear section rather than multiplying across every operation in every voice.

The architecture should make it straightforward to wrap any nonlinear stage in an oversample-process-decimate pipeline without restructuring the surrounding signal flow. This means the oversampling wrapper should be a general-purpose utility that any processing stage can use, not a special case built into one particular effect.

Designing this wrapper as a reusable component early in the architecture simplifies adding new nonlinear features later. A well-designed wrapper accepts a processing function, an oversampling factor, and a filter quality setting, and returns an oversampled version of that function that operates transparently at the output sample rate. This kind of composable design pays for itself as soon as the second nonlinear stage is added to the synthesizer.

## Quality Modes

A synthesizer that offers only one anti-aliasing strategy forces users into a fixed tradeoff between quality and performance. Quality modes solve this by making the tradeoff selectable. The user or the host environment chooses a quality level, and the synthesizer adjusts its internal anti-aliasing processing accordingly.

A draft or economy mode might use PolyBLEP oscillators with no oversampling on nonlinear stages. This minimizes processing cost, keeps latency low, and still provides acceptable quality for sound design exploration, live performance on limited hardware, or running many voices simultaneously. The sound may exhibit some aliasing at extreme settings, but it remains musically useful. This mode enables the highest polyphony because each voice uses the least processing per sample.

A standard mode might use MinBLEP or high-quality wavetable oscillators with 2x oversampling on distortion and waveshaping stages. This eliminates most audible aliasing for typical musical use while keeping the processing cost reasonable for real-time playback on modern hardware. For most users in most situations, this mode offers the best balance between fidelity and performance.

Naming the modes clearly helps users understand what they are choosing. Labels like "Draft", "Standard", and "Ultra" are more informative than abstract numbers. Brief descriptions of what each mode changes, visible in the interface, help users make informed choices without needing to understand the technical details.

A high or offline mode might use 4x or 8x oversampling on all nonlinear stages with the highest-quality decimation filters available. This is intended for final rendering, offline bouncing, or situations where processing cost is not a concern and maximum fidelity is the goal. In this mode, the synthesizer prioritizes sound quality absolutely, and it is acceptable for the processing to exceed real-time speed.

Intermediate modes are also possible. A system might offer three or four levels rather than just two extremes. The key architectural requirement is that the stages which change behavior between modes are isolated behind a consistent interface, so the rest of the signal chain does not need to know which mode is active. Changing the quality mode should feel like turning a knob: the output changes, but the signal flow and voice structure remain the same.

Quality modes raise an important design concern around preset consistency. A preset designed and tuned in high-quality mode may sound slightly different in draft mode because the presence or absence of aliasing artifacts changes the timbral character of the sound. Bright, heavily distorted patches are the most affected. Patches that rely on subtle saturation or gentle waveshaping may sound nearly identical across modes.

The synthesizer should document this behavior clearly and may want to store the intended quality mode as part of the preset metadata so the user knows what mode the designer was listening in. Some synthesizers display a warning when a preset is loaded in a different quality mode than the one it was designed for.

Users should be able to preview the difference between modes easily. The ability to switch quality modes during playback, without interrupting the audio stream, makes the tradeoff tangible and helps users make informed decisions about when the higher cost is worthwhile. A clear visual indicator of the current quality mode prevents confusion about why a patch sounds different than expected.

For plugin hosts that support offline rendering, the synthesizer could automatically use its highest quality mode during bounce or export operations, regardless of the real-time mode setting. This ensures that final renders always benefit from maximum anti-aliasing quality without requiring the user to remember to switch modes manually. Many commercial synthesizers implement this behavior, and users have come to expect it.

## Common Anti-Aliasing Mistakes

One of the most frequent mistakes is treating anti-aliasing as optional polish, something to add late in development after the core synthesis engine is working. Aliasing is not cosmetic. It affects tuning stability, timbral consistency across the keyboard, and the perceived quality of every patch.

A user who builds presets on a non-anti-aliased engine will find that those presets sound different once anti-aliasing is added, because the aliased harmonics that were part of the perceived sound are now gone. Retrofitting anti-aliasing into an engine designed without it is significantly harder than designing for it from the start because the oscillator interface, buffer management, and processing order may all need to change.

A related mistake is assuming that running at a higher output sample rate, such as 96 kHz instead of 48 kHz, eliminates the need for anti-aliasing. Higher sample rates push Nyquist higher, which reduces aliasing, but they do not eliminate it. A naive sawtooth at 96 kHz still aliases at high pitches. The higher sample rate buys headroom but does not replace proper band-limiting.

Another common mistake is applying oversampling uniformly across the entire signal chain. This wastes processing power on stages that do not generate new harmonics. A band-limited oscillator feeding into a linear filter does not benefit from oversampling because no new above-Nyquist content is being created at those stages. Oversampling should target the specific points where new harmonics appear: nonlinear processors, distortion stages, waveshapers, and driven filters. Blanket oversampling also increases latency unnecessarily for the linear portions of the signal path.

Forgetting about synthesis methods that generate extreme harmonic content is a related error. Frequency modulation, oscillator sync, hard sync, and ring modulation can produce spectra far denser and wider than standard subtractive waveforms. Band-limited wavetables do not help here because the output waveform depends on continuously changing modulation values that cannot be pre-computed.

These methods need their own anti-aliasing strategies, which may involve oversampling, specialized per-method algorithms, or accepting constrained operating ranges where aliasing is kept below an audible threshold. Planning for these methods early, even if they are not implemented in the first release, prevents architectural decisions that would make them difficult to add later.

Assuming that band-limited wavetables solve all oscillator aliasing problems is another trap. Wavetables work well for static or slowly changing periodic shapes, but they do not cover waveforms that change rapidly or are generated procedurally. Any architecture that relies solely on wavetables will encounter aliasing as soon as it adds FM, sync, or dynamic waveshaping features. The anti-aliasing strategy must be flexible enough to cover both table-based and algorithmic sound generation.

Underestimating the importance of the decimation filter is a more technical but equally damaging mistake. A poor decimation filter can undo much of the benefit of oversampling by letting aliased content leak through its transition band. Conversely, an overly steep decimation filter can introduce its own artifacts, including ringing and phase distortion near the cutoff frequency.

The decimation filter deserves the same design attention as any other filter in the signal chain. It should be tested independently for both alias rejection and signal transparency, and its characteristics should be documented as part of the quality mode specification.

Another overlooked issue is failing to account for pitch modulation. Vibrato, pitch bend, and portamento can temporarily push the effective frequency of an oscillator into a range where its anti-aliasing is insufficient. A note at C5 with deep vibrato may briefly reach pitches where the harmonic content exceeds what the current anti-aliasing method can cleanly handle. The anti-aliasing strategy should be evaluated not just at static pitches but during dynamic pitch changes, where the instantaneous frequency may vary rapidly.

Finally, neglecting to test anti-aliasing at high pitches is a mistake that allows problems to ship unnoticed. Aliasing is least audible at low pitches and most severe at high pitches. Testing should include sustained high notes, fast arpeggios in the upper register, and sweeps across the full pitch range while listening carefully for inharmonic artifacts. A methodical approach that includes spectral analysis catches problems that casual listening at comfortable middle-register pitches will miss entirely.

## Design Recommendations For Digital Synth

For the initial subtractive oscillators, band-limited wavetables or PolyBLEP represent the best starting points. Both methods are well understood, efficient, and produce good results for standard waveforms across the keyboard.

PolyBLEP is simpler to integrate with continuous waveform parameters like pulse width because the correction automatically adapts to the discontinuity position. It is also the easiest of the three to understand and debug, which has value during early development.

Band-limited wavetables are a natural fit if the oscillator architecture already uses table lookup, and they scale well to custom user waveforms imported by the user. MinBLEP should be considered as an upgrade path for situations where the highest oscillator quality is desired without the memory overhead of wavetables.

Any nonlinear processing stage should be oversampled. This includes filter drive, distortion effects, saturation, waveshaping, and wavefolding. The oversampling factor should be at least 2x for standard use, with 4x available in higher quality modes. The decimation filter quality matters and should be tested carefully, both for alias rejection and for transparency near the top of the audible range. A decimation filter that dulls the brightness of the output defeats the purpose of a high-quality nonlinear stage. Testing the oversampled stage in isolation, comparing its output to a non-oversampled version, is a useful way to verify that the oversampling is providing a net improvement without introducing its own coloration.

Quality modes should be planned as a long-term architecture feature even if only one mode is available initially. Designing the processing pipeline so that oversampling factors and oscillator methods can be swapped per mode is much easier than adding this flexibility later.

The mode system should be simple and global: a single quality setting that affects all relevant stages consistently, so the user does not need to configure anti-aliasing on a per-module basis. Exposing per-stage oversampling controls may be appropriate for advanced users, but the default experience should be a single, clearly labeled quality selector.

Anti-aliasing quality should be testable throughout development. Comparing the output of a high note against a known clean reference, such as a pure additive synthesis rendering of the same waveform, provides an objective measure of alias suppression. Spectral analysis of high-pitched notes can reveal aliased components as frequencies that do not fall on expected harmonic positions.

This kind of test should be part of the ongoing quality evaluation process, not a one-time check performed during initial development. Every change to the oscillator, filter, or nonlinear processing code has the potential to introduce or worsen aliasing, so the test should be repeatable and ideally automated. Regression in anti-aliasing quality should be treated as seriously as any other functional regression.

The architecture should also anticipate that future synthesis methods beyond basic subtractive will need anti-aliasing support. FM synthesis, oscillator sync, waveshaping oscillators, and additive-to-subtractive hybrid approaches each bring their own aliasing challenges. Designing the anti-aliasing infrastructure as a reusable, composable capability rather than a one-off oscillator feature will reduce the cost of adding new synthesis methods as the instrument grows.

Anti-aliasing should be thought of as a system-level concern that touches every part of the signal chain where harmonics are created or transformed, not as a property of any single module.

A useful guiding principle is: any stage that creates harmonic content needs its own anti-aliasing strategy. Oscillators need band-limiting. Nonlinear effects need oversampling. Synthesis methods that produce unpredictable spectra need either oversampling or specialized techniques.

If every harmonic-generating stage handles its own anti-aliasing correctly, the rest of the signal chain can operate without worrying about aliased content propagating through it. Linear operations like mixing, panning, and linear filtering do not create new harmonics and therefore do not need anti-aliasing treatment.

This modular approach to anti-aliasing mirrors the modular approach to synthesis that the project's architecture already emphasizes. Each module takes responsibility for the quality of its own output, and the system as a whole benefits from the sum of those local guarantees.
