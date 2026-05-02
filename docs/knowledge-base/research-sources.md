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

## Research Policy

When adding future research:

- Prefer official documentation, standards bodies, classic papers, or respected educational sources.
- Note whether a source is conceptual, historical, implementation-specific, or product-specific.
- Avoid letting one source silently choose the project's stack.
- Convert source material into project-specific understanding rather than copying it.

