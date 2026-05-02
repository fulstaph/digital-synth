# Development Roadmap

This document consolidates the project's development stages into a single reference with explicit goals, deliverables, dependencies, and completion criteria. It expands on the stages outlined in the project README and integrates the method and effects roadmaps from the knowledge base.

Each stage represents a design phase, not an implementation sprint. The output of every stage is documentation, specifications, and design decisions. Implementation begins only after Stage 8 defines a technical stack and deployment target.

---

## Stage 0: Knowledge Base and Architecture

This is the current stage.

**Goal:** Build a complete conceptual foundation covering all major synthesis topics, real-time audio constraints, and the project's modular architecture.

**Deliverables:**

- All knowledge-base documents covering digital audio fundamentals, synthesis methods, oscillators, filters, modulation, envelopes, voices, performance control, effects, mixing, and output.
- Conceptual architecture document defining subsystems and their relationships.
- Glossary with definitions for every term used in the knowledge base.
- Research sources document listing references and prior art.
- Parameter design guidance establishing conventions for specifying parameters across future stages.
- Deep-dive documents for synthesis methods planned in Stage 7 (wavetable, FM, granular).

**Dependencies:** None.

**Completion criteria:**

- Every major synthesis concept relevant to the first instrument has a dedicated section or document.
- The architecture document covers all subsystems: oscillators, filters, amplifiers, envelopes, LFOs, modulation matrix, voice management, effects chain, preset system, performance controls, visualization, and output.
- The glossary defines every technical term that appears in the knowledge base.
- No document uses an undefined term.
- The parameter design guidance is complete enough to apply consistently in Stage 1.

**Open questions:**

- How deep should advanced topics (spectral processing, convolution, vocoding) go before moving to Stage 1? These topics support Stage 7 but may not need full treatment yet.
- Should the architecture document include preliminary subsystem interface sketches, or should those wait for Stage 1?
- Are there synthesis concepts still missing from the knowledge base that would block Stage 1 work?

---

## Stage 1: Minimal Musical Voice

**Goal:** Design one playable voice end-to-end, from oscillator through filter and amplifier to output, with enough detail to specify every parameter, signal flow point, and behavioral rule.

**Deliverables:**

- Voice specification document describing the complete signal path: oscillator, filter, amplifier, output.
- Parameter specifications for every control in the voice, including ranges, scaling curves, default values, units, and smoothing requirements, following the parameter design guidance from Stage 0.
- Behavioral specification for note start, hold, and release, covering amplitude envelope shape, timing, and response to rapid retriggering.
- Anti-aliasing requirements for oscillator waveforms at this stage.
- Gain staging rules for the single-voice signal path.

**Dependencies:** Stage 0 complete.

**Completion criteria:**

- A single document fully specifies the voice so that an implementer could build it without further design questions.
- Every parameter has a defined range, default, scaling curve, and unit.
- Note lifecycle behavior (attack, sustain, release, retrigger) is unambiguous.
- The voice design is consistent with the modular architecture from Stage 0.

**Open questions:**

- Which oscillator types should the first voice include? Candidates are sine, triangle, sawtooth, square, and pulse. Including all five adds complexity; starting with fewer may speed up Stage 1 but limit early sound variety.
- Should the voice include one filter or two? A single filter is simpler; dual filters enable serial and parallel topologies but belong more naturally in Stage 2.
- How much modulation should exist before Stage 2? A hardwired envelope-to-filter route may be necessary for a musically useful voice, but a general modulation system is a Stage 3 concern.
- What is the minimum envelope specification? ADSR is standard, but DAHDSR or multi-segment envelopes affect the parameter count significantly.

---

## Stage 2: Polyphonic Subtractive Instrument

**Goal:** Design multi-voice behavior, voice allocation, and the complete subtractive signal path including oscillator mixing, filter envelopes, and LFO modulation.

**Deliverables:**

- Voice allocation specification covering polyphony limits, voice stealing strategies (oldest, quietest, lowest-priority), and release behavior.
- Monophony and unison mode specifications.
- Oscillator mixing rules: level, detune, octave, and phase relationships between oscillators within a voice.
- Filter envelope specification with dedicated controls separate from the amplitude envelope.
- LFO specification covering waveforms, rate ranges, sync options, and common routing targets (pitch vibrato, filter cutoff, amplitude tremolo, pulse width).
- Musical defaults: a set of parameter values that produce a usable, pleasant sound when the instrument initializes.
- Safe parameter ranges that prevent silence, distortion, or inaudible results at extremes.
- Sustain pedal behavior and its interaction with voice allocation.

**Dependencies:** Stage 1 complete.

**Completion criteria:**

- Voice allocation handles all common polyphonic cases: more notes than voices, sustained notes, rapid arpeggios, sustain pedal held, and monophonic mode.
- Default parameter values produce a musically useful sound without user adjustment.
- The subtractive signal path is fully specified from oscillator mix through filter through amplifier to voice output.
- LFO behavior is defined for both free-running and key-synced modes.

**Open questions:**

- What is the default voice count? Eight is common, but the design should allow configuration.
- How should voice stealing handle the sustain pedal? Sustained voices may need to be stealable under pressure.
- Should unison mode be a separate specification or part of the voice allocation document?

---

## Stage 3: Modulation-First Sound Design

**Goal:** Design the modulation matrix, macro system, and the visualization requirements that make modulation relationships visible to the user.

**Deliverables:**

- Modulation matrix specification: all sources (envelopes, LFOs, velocity, key tracking, aftertouch, mod wheel, macros, random), all destinations (oscillator pitch, filter cutoff, resonance, amplitude, panning, LFO rate, effect parameters), depth control, polarity (unipolar, bipolar), and per-voice versus global behavior.
- Macro system design: number of macros, how macros map to multiple destinations with independent depth and polarity, how macros are stored in presets.
- Per-voice versus global modulation distinction: which sources are per-voice (envelopes, velocity, key tracking) and which are global (master LFO, mod wheel, macros).
- Visualization requirements: modulation activity meters, source-destination connection display, depth indicators, real-time movement display.
- Randomization specification: parameter constraints for safe random patch generation, weighting rules, which parameters are randomizable and which are excluded.
- Patch morphing specification: interpolation behavior between two parameter states, which parameters interpolate smoothly and which snap.

**Dependencies:** Stage 2 complete.

**Completion criteria:**

- The modulation matrix supports all sources and destinations identified in the knowledge base.
- Macros are fully integrated into the modulation routing and the preset structure.
- The distinction between per-voice and global modulation is clear for every source and destination.
- Visualization requirements are specific enough to guide interface design.

**Open questions:**

- How many simultaneous modulation routings should the matrix support? A fixed limit simplifies the design; an unlimited model is more flexible but harder to visualize.
- Should modulation depth itself be modulatable (modulation of modulation)?
- How should randomization interact with the modulation matrix? Randomizing modulation routings is powerful but can produce unusable results.

---

## Stage 4: Presets and User Workflow

**Goal:** Design the preset system, including structure, metadata, browsing, comparison, morphing, randomization, and forward compatibility.

**Deliverables:**

- Preset structure definition: what data a preset contains (all voice parameters, modulation routings, macro assignments, effect settings, metadata).
- Metadata schema: name, author, category, tags, creation date, description, patch notes explaining how the sound works.
- Init patch definition: the default state when the user starts from scratch, designed to be a neutral, useful starting point.
- Browsing and organization specification: categories, tags, favorites, search, filtering, sorting.
- Comparison specification: how a user can audition two presets side by side.
- Morphing specification: how a user can blend between two preset states in real time.
- Randomization integration: how the safe randomization rules from Stage 3 connect to the preset workflow.
- Sound-design recipes: documented parameter combinations for common sound targets (bass, pad, pluck, lead, drone, percussion, effects).
- Forward compatibility rules: how the preset format handles parameters that do not yet exist (future synthesis engines, new effects, additional modulation sources).

**Dependencies:** Stage 3 complete. Modulation routings and macro assignments must be designed before they can be stored in presets.

**Completion criteria:**

- The preset format stores every parameter and routing defined through Stage 3.
- Forward compatibility strategy handles addition of new parameters without breaking existing presets.
- Init patch values are defined and produce a musically neutral starting point.
- Sound-design recipes demonstrate that the parameter space can produce a range of useful sounds.

**Open questions:**

- Should presets store effect settings, or should effects have independent preset management?
- How should preset versioning work when the parameter set expands in later stages?
- What level of metadata is required versus optional for user-created presets?

---

## Stage 5: Effects and Polish

**Goal:** Design the effects chain with clear specifications for first-tier and second-tier effects, routing, gain staging, and quality modes.

**Deliverables:**

- First-tier effect specifications: simple saturation or drive, chorus, delay, reverb, output level and metering. Each effect gets a parameter specification following the same conventions as voice parameters.
- Second-tier effect concepts: phaser, flanger, EQ, compressor, limiter, advanced distortion, per-voice effects. These need design outlines but not full specifications until needed.
- Effect order specification: the default signal chain from voice output through effects to master output, consistent with the effects roadmap in the knowledge base.
- Gain staging rules for the effects chain: headroom requirements, level management between effects, metering points.
- Quality modes: how effects can offer reduced-quality modes for lower CPU usage, and how this interacts with presets.
- Effect routing: initial fixed-order chain design, with notes on where flexible routing could be added later.

**Dependencies:** Stage 2 complete (voice output signal to process). Stage 4 complete (effects stored in presets).

**Completion criteria:**

- All first-tier effects have complete parameter specifications.
- Effect order is defined and justified.
- Gain staging through the effects chain prevents clipping and preserves headroom.
- Quality mode behavior is specified for at least one CPU-intensive effect (reverb).

**Open questions:**

- Should effects be designed as a fixed chain or as insertable slots from the start?
- How detailed should second-tier effect specifications be at this stage?
- Should per-voice effects (such as per-voice saturation) be part of Stage 5 or deferred?

---

## Stage 6: Expressive Control

**Goal:** Design detailed behavior for every performance gesture the instrument should support, from velocity and pitch bend through per-note expression and microtuning.

**Deliverables:**

- Velocity specification: response curve, scaling, destinations, and how velocity interacts with modulation.
- Pitch bend specification: range (semitones), curve, per-voice versus global behavior.
- Pressure and aftertouch specification: channel pressure and polyphonic aftertouch as modulation sources.
- Mod wheel and assignable controls: default routings, user-configurable destinations.
- Sustain pedal behavior: interaction with voice allocation, half-pedal support.
- Per-note expression specification: MPE or similar per-note pitch, pressure, and slide as modulation sources, integration with the modulation matrix from Stage 3.
- Glide and legato specification: glide time, glide curve, legato trigger behavior, portamento modes (constant time versus constant rate).
- Retrigger and note priority rules: last-note, lowest-note, highest-note priority in monophonic mode, retrigger versus legato envelope behavior.
- Microtuning support: alternative tuning tables, scale selection, interaction with pitch bend and glide.

**Dependencies:** Stage 2 complete (voice behavior to control). Stage 3 complete (modulation matrix to route expression sources).

**Completion criteria:**

- Every performance gesture listed in the project README has a defined behavior.
- Each gesture has a clear modulation routing through the matrix or a hardwired path with documented rationale.
- Glide, legato, retrigger, and note priority modes are fully specified for monophonic operation.
- Microtuning is designed as an optional layer that does not complicate standard equal-temperament use.

**Open questions:**

- How should per-note expression interact with voice stealing? If a voice is stolen, does the new note inherit the previous expression state?
- Should microtuning be a global setting or a per-preset parameter?
- How many assignable continuous controllers should the design support?

---

## Stage 7: Advanced Synthesis Expansion

**Goal:** Design additional synthesis engines that reuse the voice, modulation, filter, effects, and preset architecture established in earlier stages.

**Deliverables:**

- Wavetable engine specification: wavetable scanning, frame interpolation, wavetable source management, integration with existing oscillator and filter architecture. The knowledge base recommends wavetable as the second synthesis method because it reuses much of the subtractive architecture and adds modern motion-heavy timbres.
- FM engine specification: operator topology, feedback, ratio and offset tuning, envelope-per-operator design, integration with the modulation matrix. The knowledge base notes that many digital FM implementations actually use phase modulation internally.
- Granular engine specification: grain generation, density, position, size, pitch, and randomization parameters, integration with sample sources.
- Sample-based and physical modeling concepts: design outlines showing how these methods would fit the existing architecture, without full specifications unless needed.
- Integration specification for each engine: how a new engine shares voice allocation, modulation routing, filter processing, effects chain, preset storage, and visualization with the subtractive core.

**Dependencies:** Stages 1 through 6 complete. The core architecture must be stable before extending it with additional engines.

**Completion criteria:**

- Each new engine has a complete design document.
- Each engine document demonstrates how it integrates with voice allocation, modulation, presets, and effects without requiring changes to those subsystems.
- The wavetable and FM engines have full parameter specifications.
- Granular, sample-based, and physical modeling engines have design outlines with identified open questions.

**Open questions:**

- Should engines be selectable per-voice (hybrid patches) or per-instrument?
- How should presets handle engine-specific parameters when switching between engines?
- What is the minimum viable wavetable engine that justifies the design effort?

---

## Stage 8: Productization Decisions

**Goal:** Choose the technical stack, deployment target, and distribution format based on the complete design produced in Stages 0 through 7.

**Deliverables:**

- Stack selection rationale: language, audio backend, UI framework, and plugin format choices with documented tradeoffs.
- Platform constraints: which operating systems, which plugin hosts, standalone versus plugin versus browser-based versus embedded.
- Performance budgets: CPU limits per voice, maximum latency, memory constraints, and how these budgets relate to the voice count, effects chain, and quality modes designed in earlier stages.
- Distribution format: plugin formats (VST3, AU, CLAP), standalone builds, installer requirements, update strategy.
- Implementation plan: how the design documents translate into development work, which subsystems to build first, and how to validate that the implementation matches the design.

**Dependencies:** Stage 7 complete. The full architecture must be designed before constraining it to a specific technology.

**Completion criteria:**

- A clear, justified technical stack is chosen.
- Performance budgets are defined for realistic musical use cases (polyphonic pad, monophonic bass, multi-engine layering).
- The implementation plan references specific design documents from earlier stages.
- Platform and distribution decisions are documented with rationale.

**Open questions:**

- Should the first implementation target a single platform or multiple platforms simultaneously?
- Is a browser-based prototype valuable as a validation step before native implementation?
- How should the project handle platform-specific audio backend differences?

---

## Cross-Stage Concerns

Some design concerns span multiple stages and must be maintained consistently throughout the roadmap.

**Anti-aliasing:** Required from Stage 1 onward. Every oscillator waveform and nonlinear process must account for aliasing. The anti-aliasing strategy defined in Stage 1 applies to all subsequent oscillator and effect work, including the advanced synthesis engines in Stage 7. The knowledge base includes a dedicated advanced anti-aliasing document that should inform decisions at every stage.

**Gain staging:** Defined in Stage 1 for the single-voice path, extended in Stage 2 for polyphonic mixing, and maintained through Stages 5 (effects) and 7 (additional engines). Every new signal processing stage must preserve headroom and avoid clipping. Metering points should be defined wherever gain changes.

**Parameter specification:** Every new parameter introduced in any stage must follow the parameter design guidance established in Stage 0. This includes range, default value, scaling curve, unit, smoothing requirement, and whether the parameter is per-voice or global. Consistency across stages prevents design drift.

**Visualization:** Grows with each stage. Stage 1 introduces waveform and envelope visualization. Stage 2 adds filter response display. Stage 3 adds modulation activity and routing visualization. Stage 5 adds effect metering. Stage 7 adds engine-specific displays (wavetable position, FM operator levels, grain activity). Visualization always reflects the current state of the audio engine accurately.

**Testing strategy:** Each stage should define testable behaviors alongside its specifications. Voice allocation rules, envelope timing, modulation routing, preset loading, and effect gain behavior should all be described precisely enough that correctness can be verified against the design. This is a design concern, not an implementation concern: the goal is specifications clear enough to test against.

**Forward compatibility:** Decisions made in early stages should not block later stages. The preset format must accommodate new parameters. The modulation matrix must accept new sources and destinations. The effects chain must allow new effects. Each stage should note where its design leaves room for future expansion.
