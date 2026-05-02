# Synthesis Knowledge Base

This directory contains the conceptual foundation for Digital Synth. It is intentionally stack-neutral and code-free. The purpose is to explain synthesis deeply before implementation work begins.

The knowledge base is organized around the questions a synthesizer designer must answer:

- What is digital audio?
- How are sounds generated?
- How are sounds shaped over time?
- How are notes allocated and controlled?
- How does modulation create musical movement?
- How do effects, mixing, and output turn raw synthesis into finished sound?
- Which terms need stable definitions before implementation?

## Reading Order

Start here:

### Core Architecture And Fundamentals

1. [Architecture](architecture.md)
2. [Digital audio fundamentals](knowledge-base/digital-audio-fundamentals.md)
3. [Control and audio rate processing](knowledge-base/control-and-audio-rate-processing.md)
4. [Parameter design guidance](knowledge-base/parameter-design-guidance.md)

### Subtractive Synthesis Core

5. [Oscillators and sound sources](knowledge-base/oscillators-and-sound-sources.md)
6. [Filters and tone shaping](knowledge-base/filters-and-tone-shaping.md)
7. [Modulation and envelopes](knowledge-base/modulation-and-envelopes.md)
8. [Voices and performance control](knowledge-base/voices-and-performance-control.md)

### Effects And Spatial Processing

9. [Effects, mixing, and output](knowledge-base/effects-mixing-and-output.md)
10. [Stereo and spatial processing](knowledge-base/stereo-and-spatial-processing.md)

### Instrument Features

11. [Arpeggiator and sequencer design](knowledge-base/arpeggiator-and-sequencer-design.md)
12. [Preset management](knowledge-base/preset-management.md)

### Synthesis Methods

13. [Synthesis methods overview](knowledge-base/synthesis-methods.md)
14. [FM synthesis deep dive](knowledge-base/fm-synthesis-deep-dive.md)
15. [Wavetable synthesis deep dive](knowledge-base/wavetable-synthesis-deep-dive.md)
16. [Granular synthesis deep dive](knowledge-base/granular-synthesis-deep-dive.md)

### Advanced Topics

17. [Advanced anti-aliasing](knowledge-base/advanced-anti-aliasing.md)
18. [Ring modulation and AM](knowledge-base/ring-modulation-and-am.md)
19. [Noise types and modulation](knowledge-base/noise-types-and-modulation.md)
20. [Bit reduction and lo-fi effects](knowledge-base/bit-reduction-and-lofi-effects.md)
21. [Spectral processing](knowledge-base/spectral-processing.md)
22. [Vocoding and formant synthesis](knowledge-base/vocoding-and-formant-synthesis.md)
23. [Convolution concepts](knowledge-base/convolution-concepts.md)

### Reference

24. [Glossary](knowledge-base/glossary.md)
25. [Research sources](knowledge-base/research-sources.md)

### Project Planning

26. [Development roadmap](development-roadmap.md)

## Knowledge Base Goals

The docs should help future contributors make deliberate choices. Each topic should explain not only what a concept is, but why the concept matters for the musical result.

For example, a filter is not only a signal-processing block. It is also one of the primary ways a synthesizer creates motion, brightness, articulation, and emotional contour. A voice allocator is not only bookkeeping. It determines whether chords feel smooth, whether released notes decay naturally, and whether a lead line behaves like an analog monosynth or a modern polyphonic instrument.

## Current Scope

The current scope is conceptual architecture and synthesis knowledge. Technical stack decisions are out of scope for now.

In scope:

- Synthesis theory.
- Signal-flow architecture.
- Modulation design.
- Voice behavior.
- Performance control.
- Preset concepts.
- Development stages.
- Feature goals.

Out of scope:

- Implementation code.
- Runtime selection.
- Framework selection.
- Plugin format selection.
- User-interface toolkit selection.
- Platform packaging.

## Documentation Quality Bar

Every major topic should answer:

- What does this mean?
- Why does it matter?
- How does it affect the sound?
- Which controls does a musician expect?
- What can go wrong?
- What should Digital Synth design for?

