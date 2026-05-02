# Stereo And Spatial Processing

Stereo and spatial processing determines how a synthesizer's sound occupies the listening space between speakers or headphones. Width, depth, and directionality transform a flat tone into a three-dimensional experience. Understanding spatial concepts at a design level helps ensure the instrument creates immersive sounds while remaining robust under real-world playback conditions.

## Mono, Stereo, And Multichannel

Mono is a single audio channel. Every sound source and every spatial cue collapses into one signal with no left-right distinction. Stereo is two channels, left and right, which together create the illusion of horizontal placement and width. Multichannel extends beyond stereo into formats such as 5.1 surround, 7.1 surround, Dolby Atmos, and binaural rendering for headphones.

Why it matters:

The channel configuration determines what spatial techniques are available and what the listener can perceive. A mono signal cannot express panning, width, or interaural time differences. A stereo signal can express horizontal placement and width but not height or true surround. Multichannel formats unlock elevation and envelopment but add significant complexity to the signal path and to testing.

Musical importance:

Most synthesizers target stereo output because it is the widest format supported by nearly all playback systems. Stereo is expressive enough to create convincing width, depth, and movement while remaining simple enough to reason about during sound design.

Design implication:

This project should target stereo as the primary output format. The signal path should be designed so that spatial processing operates on a stereo pair. If multichannel or binaural output becomes a goal later, the stereo architecture should be extensible rather than disposable. Internal processing should track two channels from the point where stereo information is first introduced.

## Panning Laws

A panning law defines how signal level is distributed between left and right channels as a sound moves across the stereo field. The choice of law affects how natural and consistent a sound feels as it moves from one side to the other.

Linear panning scales the left and right levels proportionally. When a sound is panned halfway to the right, the left channel drops to half level and the right channel rises to half level. This is simple to implement but produces a perceived loudness dip at the center position because two speakers reproducing the same signal at reduced level do not sum to the same perceived loudness as one speaker at full level.

Equal-power panning uses a curve, typically derived from sine and cosine functions, that preserves perceived loudness across the entire pan range. At center, both channels are at roughly seventy percent of full level, which when summed acoustically produces the same perceived energy as full level from one side. The transition across the stereo field sounds smooth and consistent.

Other panning laws exist, such as constant-voltage panning, which is sometimes used in broadcast, and modified power curves that compromise between linear and equal-power behavior. These are less common in synthesis but worth acknowledging for completeness.

Why it matters:

The panning law determines whether sounds feel consistent in volume as they move. A sound that dips in loudness at center will feel uneven during automated pan sweeps or when using unison spread. Equal-power panning is the standard for most synthesis and mixing applications because it maintains a stable image.

Common controls:

- Pan position, typically a bipolar knob from full left to full right.
- Pan law selection, if multiple laws are supported.
- Pan modulation, allowing envelopes or LFOs to automate pan position over time.

Design implication:

Equal-power panning should be the default for all pan controls in the project. If a linear option is ever exposed, it should be clearly labeled as an alternative. Pan controls should also handle the extreme positions correctly, delivering full level to one channel and silence to the other without clicks or discontinuities.

## Mid/Side Processing

Mid/side processing is a way of encoding a stereo signal as two components. The mid signal is the sum of left and right, representing content that is identical in both channels and perceived as centered. The side signal is the difference between left and right, representing content that differs between channels and perceived as width.

Processing mid and side independently allows:

- Boosting the side channel to increase width without affecting centered content.
- Compressing the mid channel to control the center image without altering stereo elements.
- Filtering only the side channel to keep low frequencies in mono, which tightens the bass.
- Applying saturation to mid and side separately for different tonal character in the center versus the edges.

The conversion between left/right and mid/side is lossless. Mid equals left plus right. Side equals left minus right. Decoding back to left/right is equally straightforward. This means mid/side processing can be inserted anywhere in the stereo signal path without permanent alteration of the format.

Why it matters:

Mid/side gives precise control over the stereo image that simple pan and width controls cannot achieve. A width knob typically scales the side component, but dedicated mid/side processing allows independent equalization, dynamics, and saturation on each component. This is especially valuable for tightening bass, controlling reverb width, and shaping the spatial character of layered sounds.

Design implication:

Mid/side processing should be available as an advanced option in the effects chain. It does not need to be a default control on every patch, but it should be accessible for users who want deeper stereo image control. The encoding and decoding are computationally trivial.

## Haas Effect

The Haas effect describes the perceptual phenomenon where a very short delay between left and right channels, typically between one and thirty milliseconds, creates a strong impression of directionality. The ear perceives the earlier arrival as the source direction. The delayed channel reinforces the sense of space without being heard as a distinct echo.

This is fundamentally different from panning. Panning changes the level balance between channels. The Haas effect changes the time balance between channels. A panned sound feels placed to one side by amplitude. A Haas-delayed sound feels placed to one side by arrival time, which more closely matches how humans localize real sound sources in physical space.

Why it matters:

The Haas effect is one of the most effective ways to create stereo placement that feels natural and three-dimensional. It gives the impression of a sound existing in a space rather than simply being louder in one speaker. When combined with panning, it creates more convincing spatial positioning than either technique alone.

Risks:

Haas-based processing can cause phase cancellation when the stereo signal is summed to mono. The delayed copy partially cancels the original, producing comb filtering that thins the sound. The severity depends on the delay time and the frequency content of the source. Short delays cause wider comb-filter notches, which can be more audible on simple waveforms.

Common controls:

- Delay time, typically expressed in milliseconds, controlling the offset between channels.
- Direction, determining which channel receives the delay.
- Mix or amount, controlling how much of the delayed signal is blended in.

Design implication:

The Haas effect should be available as a spatial placement tool, but it should be paired with a mono-compatibility check. Users should be able to audition the mono sum to hear whether cancellation is problematic for a given patch. The delay range should be constrained to the perceptually useful window to prevent users from accidentally creating audible echoes.

## Stereo Widening Techniques

Stereo widening refers to a family of methods for making a sound feel wider than its original stereo image. Each technique has a different sonic character and different implications for mono compatibility.

Chorus-based widening uses modulated delays with different settings per channel. The left and right channels receive slightly different pitch and timing variations, which the ear interprets as multiple sources spread across the stereo field. This produces lush, animated width but introduces pitch modulation that may not suit every sound.

Micro-delay widening applies a slight fixed delay to one channel, directly related to the Haas effect. It creates a sense of spatial spread with minimal coloration. The tradeoff is that it affects mono compatibility in proportion to the delay time.

Phase-based widening inverts or shifts the phase of selected frequencies in one channel. This creates channel differences that the ear interprets as width. It can be very effective but is the most dangerous technique for mono compatibility because phase-inverted content cancels completely when summed.

All-pass filter dispersion uses frequency-dependent phase shifts to create subtle differences between channels without changing the frequency content. This produces a diffuse sense of width that is less dramatic than other methods but generally more mono-safe because the magnitude spectrum is preserved.

Why it matters:

Each widening technique occupies a different point on the spectrum between dramatic width and safe mono behavior. Knowing the character and tradeoffs of each method allows the sound designer to choose the right tool for the context. A pad might benefit from chorus-based widening, while a bass sound needs a mono-safe approach.

Mistakes and edge cases:

- Applying aggressive widening to bass frequencies creates an unfocused low end that translates poorly to most playback systems.
- Stacking multiple widening techniques can push the stereo image into extreme phase territory, making mono collapse severe.
- Widening a signal that is already wide from unison spread may produce diminishing returns or introduce unwanted artifacts.

Design implication:

The project should support at least two widening methods with different characters. Each method should document its mono-compatibility behavior. A single width control can blend between methods or expose them as distinct options depending on the interface design.

## Mono Compatibility

Mono compatibility describes how well a stereo signal retains its character when the left and right channels are summed into a single channel.

Why it matters in practice:

- Phone speakers are effectively mono.
- Club and venue sound systems often sum stereo to mono for subwoofer feeds or zone distribution.
- Broadcast standards sometimes require mono compatibility.
- Bluetooth speakers and smart speakers frequently reproduce a mono signal despite having two drivers.

How phase-based stereo techniques fail in mono:

When two channels contain phase-inverted content, summing them causes cancellation. Frequencies that are perfectly out of phase disappear entirely. Frequencies that are partially out of phase are reduced in level. The result is a thinner, hollower, or quieter version of the intended sound. In extreme cases, key elements of a patch can vanish in mono playback.

Degrees of failure:

Not all mono-compatibility problems are equal. A subtle loss of high-frequency air when summing to mono is usually acceptable. A noticeable drop in overall level is concerning. Complete disappearance of a core element like the fundamental of a bass patch is a critical failure. The severity depends on the musical context and the intended playback environment.

Design-level testing:

Checking patches in mono should be a standard verification step during sound design and preset creation. The interface should provide a simple way to audition the mono sum, either as a dedicated button or as a monitoring option. Presets that ship with the instrument should be verified for acceptable mono behavior.

Design implication:

Mono compatibility should be treated as a first-class design constraint, not an afterthought. The default stereo processing should favor mono-safe techniques. Aggressive widening options can be available but should be clearly marked as having mono-compatibility tradeoffs. The visualization layer should eventually show stereo correlation to help users identify phase problems visually.

## Spatial Contributions From Effects

Individual effects each contribute a different dimension to the spatial image of a sound. The complete sense of space in a synthesizer patch is not produced by a single stereo control but by the combined behavior of several processing stages.

Reverb creates depth. It simulates the reflections of a physical space, making a sound feel near or far depending on the ratio of direct signal to reflected signal. Longer decay and higher wet mix push a sound further back in the perceived space. Shorter, tighter reverb keeps it closer.

Delay creates echoes that imply room size, surface reflections, and spatial complexity. Stereo delay with different times per channel adds width and movement. Tempo-synced delay creates rhythmic spatial patterns. Feedback-heavy delay builds density that fills the stereo field.

Chorus and modulation effects create micro-displacement. The slight pitch and timing differences between channels suggest multiple sources spread across the listening space. This is the primary mechanism for the classic wide pad and ensemble string sounds.

How these effects interact:

Width from chorus feeds into depth from reverb, creating a sound that is both wide and deep. Delay echoes are placed in the reverb space, adding complexity. The order of these effects determines how each spatial dimension interacts with the others. Chorus before reverb widens the input to the reverb. Reverb before chorus places the reverb tail under modulation.

Why it matters:

Spatial processing is distributed across the effects chain rather than being a single width or space control. Understanding this distribution helps users build patches with intentional spatial character instead of stacking effects blindly. It also informs the architecture: the effects chain must preserve stereo information correctly at every stage.

Design implication:

The project should document which effects contribute to width, which contribute to depth, and which contribute to movement. The default effect order should produce a coherent spatial result. Users who reorder effects should understand the spatial consequences.

## Design Recommendations For Digital Synth

Equal-power panning should be the default panning law for all pan controls in the instrument. This ensures consistent perceived loudness across the stereo field.

Mid/side processing should be available as an advanced option within the effects section. It does not need to appear on the main interface but should be accessible for users who want precise stereo image control.

Unison stereo spread should be mono-safe by default. The spread mechanism should distribute voices across the stereo field using level-based panning rather than phase manipulation. A width control can allow users to push beyond the mono-safe default when they choose to accept the tradeoff.

Reverb and delay should be the primary depth controls. The interface should encourage users to think of reverb as distance and delay as spatial complexity rather than treating them as interchangeable ambience tools.

All stereo widening techniques should be testable in mono. The interface should provide a mono-sum audition control. Presets should be verified for acceptable mono behavior before inclusion in the factory library.

The visualization layer should eventually show stereo image information. A stereo correlation meter or a simple left-right activity display would help users understand what their patch is doing spatially. A correlation reading near zero or negative indicates phase problems that will cause mono cancellation.

The spatial processing architecture should keep stereo information clean throughout the signal path. Once a signal becomes stereo, every subsequent processing stage should handle two channels correctly. Accidental mono summing inside the effects chain would destroy spatial information that cannot be recovered.

Spatial processing should be documented as a cross-cutting concern. It is not a single module but a set of behaviors distributed across panning, effects, voice management, and output. The architecture documents should identify every point where stereo information is created, modified, or potentially lost so that the full spatial signal path is understood before implementation begins.
