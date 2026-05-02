# AGENTS.md

## Project Purpose

This repository is for designing a programmable digital synthesizer from first principles. The current phase is documentation, research, and conceptual architecture. Implementation details are intentionally deferred.

Agents working in this repository should treat the project as a music-technology design project, not as a generic software scaffold. The important work at this stage is to clarify synthesis concepts, define behavior, document musical goals, and preserve architectural choices that will survive future implementation decisions.

## Current Phase

The project is in a pre-stack, pre-implementation phase.

Allowed work:

- Create and improve conceptual documentation.
- Expand the synthesis knowledge base.
- Define terminology.
- Compare architectural options.
- Clarify product goals and development stages.
- Improve project guidance files.
- Add diagrams or explanatory prose when useful.

Avoid for now:

- Choosing a programming language or framework.
- Choosing a plugin format.
- Choosing a UI toolkit.
- Writing implementation code.
- Adding dependencies.
- Tying architecture to the placeholder source tree.

## Documentation Standards

Documentation should be detailed, precise, and practical. The reader should be able to understand what a synthesizer component does, why it matters musically, and what design tradeoffs are involved.

Good documentation in this project:

- Defines terms before relying on them.
- Explains musical importance, not only technical mechanics.
- Separates concepts from implementation details.
- Names tradeoffs clearly.
- Avoids vague claims such as "make it better" or "add warmth" unless the behavior is explained.
- Describes expected behavior in enough detail that it can later become implementation criteria.
- Uses examples as prose descriptions, not code.

## Project Architecture Principles

The synthesizer should be designed as a set of conceptual subsystems with clear responsibilities.

Important subsystems:

- Input and performance control.
- Voice allocation.
- Per-voice sound generation.
- Modulation.
- Filtering and tone shaping.
- Mixing and gain staging.
- Effects.
- Presets and patch metadata.
- Visualization and learning aids.
- Non-real-time editing and management.

The architecture should preserve the distinction between real-time audio behavior and slower control, editing, browsing, or visualization behavior. Even before choosing a stack, this distinction matters because real-time audio systems fail audibly when deadlines are missed.

## Writing Style

Use plain technical prose. Prefer clear definitions, short sections, and concrete examples. Avoid marketing language. Avoid implementation snippets. Avoid assuming the reader already knows synthesis jargon.

When adding a new synthesis topic, include:

- Definition.
- Why it matters.
- How it affects sound.
- Common controls.
- Common mistakes.
- Design implications for this project.

## Stack Neutrality

Do not introduce a technical stack into project guidance files yet. If a technical option must be discussed, frame it as a future decision area rather than a chosen path.

Examples of acceptable phrasing:

- "The future implementation should provide..."
- "A later stack decision must account for..."
- "The architecture should allow..."

Examples of unacceptable phrasing during the current phase:

- "The synth will be built with..."
- "Use this framework for..."
- "The plugin format is..."

## Completion Criteria For Documentation Changes

A documentation change is useful when it improves one of these areas:

- The project vision is clearer.
- A synthesis term is easier to understand.
- A subsystem boundary is better defined.
- A future implementation choice becomes less ambiguous.
- A musical feature has clearer expected behavior.
- A tradeoff is documented instead of hidden.

