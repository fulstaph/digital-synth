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

1. [Architecture](architecture.md)
2. [Digital audio fundamentals](knowledge-base/digital-audio-fundamentals.md)
3. [Oscillators and sound sources](knowledge-base/oscillators-and-sound-sources.md)
4. [Filters and tone shaping](knowledge-base/filters-and-tone-shaping.md)
5. [Modulation and envelopes](knowledge-base/modulation-and-envelopes.md)
6. [Voices and performance control](knowledge-base/voices-and-performance-control.md)
7. [Effects, mixing, and output](knowledge-base/effects-mixing-and-output.md)
8. [Synthesis methods](knowledge-base/synthesis-methods.md)
9. [Glossary](knowledge-base/glossary.md)
10. [Research sources](knowledge-base/research-sources.md)

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

