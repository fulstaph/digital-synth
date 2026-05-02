# CLAUDE.md

## Project Context

Digital Synth is currently a documentation-first project for designing a digital synthesizer. The repository should develop a strong conceptual foundation before implementation begins. Treat all current work as research, architecture, and product design unless explicitly told otherwise.

The project should not be narrowed to a specific technical stack yet. Avoid making assumptions about runtime, plugin format, user-interface framework, deployment target, or audio backend.

## What This Project Is Trying To Become

The long-term goal is an expressive programmable synthesizer that can serve as:

- A playable musical instrument.
- A sound-design laboratory.
- A learning environment for synthesis concepts.
- A platform for experimenting with multiple synthesis methods.

The likely first mature instrument is a subtractive synthesizer with a strong modulation system, careful voice behavior, useful effects, preset management, and visual explanations of what the sound engine is doing.

## Current Documentation Priorities

Prioritize:

- Clear synthesis definitions.
- Conceptual architecture.
- Musical goals and user-facing behavior.
- Real-time audio constraints explained at a design level.
- Development stages.
- Tradeoffs among synthesis methods.
- Preset, modulation, and performance-control concepts.

Do not prioritize:

- Implementation code.
- Language-specific details.
- Framework-specific APIs.
- Dependency selection.
- Build tooling.
- Platform packaging.

## Important Design Principles

The synthesizer should be modular, expressive, and understandable.

Modular means each major part has a clear responsibility: input handling, voices, oscillators, modulation, filters, effects, presets, and visualization should be separable at the design level.

Expressive means the instrument should respond musically to performance gestures such as velocity, pitch bend, pressure, modulation controls, macros, legato, glide, and eventually per-note expression.

Understandable means the documentation and future interface should help users connect controls to sound. A user should be able to learn why a patch behaves the way it does.

## Documentation Rules

Keep the writing detailed but grounded. Define terms before using them. When describing a feature, explain both the technical role and the musical reason it matters.

Avoid code examples. Avoid pseudocode. Avoid choosing a stack. Avoid adding implementation-specific recommendations unless the user explicitly asks for that phase.

When expanding the knowledge base, prefer this shape:

- What the term or subsystem means.
- Why it matters.
- How it affects sound.
- Which controls are usually exposed.
- What mistakes or edge cases matter.
- How it should influence this project.

## Project Boundary

This repository may contain placeholder source files, but those files are not the product design. The authoritative material for the current phase is the documentation.

