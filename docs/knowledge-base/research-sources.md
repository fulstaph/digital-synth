# Research Sources

This file collects reference sources for the synthesis knowledge base. The project should prefer stable, primary, or widely respected sources when possible.

## Digital Audio And Real-Time Processing

- PortAudio documentation on latency: https://portaudio.com/docs/latency.html
- PortAudio callback guidance: https://www.portaudio.com/docs/v19-doxydocs/writing_a_callback.html
- Microsoft XAudio2 callback documentation: https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-callbacks
- Analog Devices explanation of band-limited sampling and aliasing: https://www.analog.com/en/resources/technical-articles/basics-of-bandlimited-sampling-and-aliasing.html

Why these matter:

Digital Synth must treat audio deadlines, buffering, latency, and aliasing as design constraints. These are not late implementation details. They affect architecture from the beginning.

## Audio Parameters and Automation Models

- MDN Web Audio API overview: https://developer.mozilla.org/en-US/docs/Web/API/Web_Audio_API
- MDN AudioWorklet: https://developer.mozilla.org/en-US/docs/Web/API/AudioWorklet
- MDN AudioParam: https://developer.mozilla.org/en-US/docs/Web/API/AudioParam
- MDN OscillatorNode: https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode
- MDN BiquadFilterNode: https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode
- W3C Web Audio API specification: https://www.w3.org/TR/webaudio/

Why these matter:

These sources are cited for their clear conceptual explanation of audio parameters, automation schedules, oscillator topology, and filter abstractions—not as a stack recommendation. The Web Audio API's parameter model (time-scheduled, exponential curves, linear ramps) represents a widely understood approach to real-time parameter automation that is independent of deployment target. These concepts apply equally to native audio APIs, embedded systems, and browser environments.

## Synthesizer Architecture

- JUCE tutorial on building a MIDI-controlled synthesizer: https://juce.com/tutorials/tutorial_synth_using_midi_input/
- JUCE Synthesiser class documentation: https://docs.juce.com/master/classjuce_1_1Synthesiser.html

Why these matter:

JUCE's synthesizer abstractions are useful conceptual references for voices, sounds, MIDI-triggered rendering, and polyphonic structure. They should not be treated as a chosen implementation framework during the current phase.

## Synthesis Theory

- Apple Logic documentation on subtractive synthesis: https://help.apple.com/logicpro/mac/9.1.6/en/logicpro/instruments/chapter_A_section_3.html
- John Chowning's AES entry for frequency modulation synthesis: https://secure.aes.org/forum/pubs/journal/?elib=1954
- Steinberg HALion documentation on FM synthesis: https://www.steinberg.help/r/halion/7.1/en/halion/topics/editing_zones/fm_synthesis_c.html
- Stanford explanation of Karplus-Strong synthesis: https://theory.stanford.edu/~blynn/sound/karplusstrong.html

Why these matter:

These references cover historically important synthesis methods and terminology: subtractive synthesis, FM carriers and modulators, and physical-modeling-related plucked-string synthesis.

## MIDI And Expressive Control

- MIDI Association page on MIDI Polyphonic Expression: https://midi.org/mpe-midi-polyphonic-expression
- MIDI Association page on MIDI 2.0: https://midi.org/midi-2-0

Why these matter:

Digital Synth should avoid an architecture that only supports low-resolution global controls. Modern expressive control requires per-note concepts and high-resolution parameter thinking.

## Plugin And Distribution Concepts

- Steinberg VST 3 developer portal: https://steinbergmedia.github.io/vst3_dev_portal/
- CLAP official repository: https://github.com/free-audio/clap
- CLAP feature overview on modulation: https://cleveraudio.org/1-feature-overview/_modulation/

Why these matter:

Plugin formats are future decisions, not current commitments. These sources are included because distribution and modulation capabilities may eventually influence architecture.

## Rust Implementation Spike References

- Michael Skiles, "How to build a synthesizer with python: part 1": https://mskiles.com/blog/how-to-build-a-synthesizer-with-python-part-1/
- Michael Skiles, "How to build a synthesizer with python: part 2": https://mskiles.com/blog/how-to-build-a-synthesizer-with-python-part-2/
- Michael Skiles, "How to build a synthesizer with python: part 3": https://mskiles.com/blog/how-to-build-a-synthesizer-with-python-part-3/
- CPAL documentation: https://docs.rs/cpal/latest/cpal/
- Rodio documentation: https://docs.rs/rodio/latest/rodio/
- DASP documentation: https://docs.rs/dasp/latest/dasp/
- FunDSP documentation: https://docs.rs/fundsp/latest/fundsp/
- Midir documentation: https://docs.rs/midir/latest/midir/
- RTRB documentation: https://docs.rs/rtrb/latest/rtrb/
- Hound documentation: https://docs.rs/hound/latest/hound/
- RustFFT documentation: https://docs.rs/rustfft/latest/rustfft/
- RealFFT documentation: https://docs.rs/realfft/latest/realfft/
- NIH-plug repository: https://github.com/robbert-vdh/nih-plug
- Clack plugin documentation: https://docs.rs/clack-plugin/latest/clack_plugin/
- Kwarf, "Writing a CLAP synthesizer in Rust - Part 1": https://kwarf.com/2024/07/writing-a-clap-synthesizer-in-rust-part-1/

Why these matter:

These references support the first Rust implementation spike. They should not be read as final product stack decisions. CPAL is the first dependency because it matches the stream-player goal from Skiles Part 1. The other Rust crates are documented so later dependency decisions can be tied to concrete project needs rather than added speculatively.

## Anti-Aliasing And Oscillator Quality

- Välimäki, V. and Franck, A., "Oscillator and Filter Algorithms for Virtual Analog Synthesis" (general reference for anti-aliasing approaches)
- Esqueda, F. et al., "Aliasing Reduction in Soft-Clipping Algorithms" (oversampling for nonlinear processing)

Why these matter:

Anti-aliasing is a core quality requirement for Digital Synth. These references cover the conceptual foundations of band-limited oscillators, PolyBLEP, and oversampling strategies discussed in the advanced anti-aliasing knowledge base document.

## Granular Synthesis

- Roads, Curtis, "Microsound" (foundational text on granular synthesis theory)
- Truax, Barry, "Real-Time Granular Synthesis with a Digital Signal Processor" (historical context for real-time granular processing)

Why these matter:

These are the standard conceptual references for understanding grain-based synthesis at the design level.

## Spectral Processing

- Dolson, Mark, "The Phase Vocoder: A Tutorial" (conceptual foundation for spectral time-pitch processing)
- Serra, Xavier, "A System for Sound Analysis/Transformation/Synthesis based on a Deterministic plus Stochastic Decomposition" (spectral analysis and resynthesis)

Why these matter:

Spectral processing concepts underpin both the visualization layer (spectrum analysis) and future spectral effects discussed in the knowledge base.

## Physical Modeling

- Smith, Julius O., "Physical Audio Signal Processing" (Stanford CCRMA, comprehensive physical modeling reference)
- Karplus, K. and Strong, A., "Digital Synthesis of Plucked-String and Drum Timbres" (original Karplus-Strong paper)

Why these matter:

These are the foundational references for the physical modeling concepts documented in the synthesis methods knowledge base.

## Vocoding And Formant Synthesis

- Dudley, Homer, "The Vocoder" (historical origin of the vocoder concept)
- Klatt, Dennis, "Software for a Cascade/Parallel Formant Synthesizer" (foundational formant synthesis reference)

Why these matter:

These references provide historical and theoretical context for the vocoding and formant synthesis concepts in the knowledge base.

## Research Policy

When adding future research:

- Prefer official documentation, standards bodies, classic papers, or respected educational sources.
- Note whether a source is conceptual, historical, implementation-specific, or product-specific.
- Avoid letting one source silently choose the project's stack.
- Convert source material into project-specific understanding rather than copying it.
