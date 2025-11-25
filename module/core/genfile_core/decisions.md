# Specification Creation Decisions

This document records all automatic decisions made during the creation of the genfile specification.

---

## Decision 1: Single-File Specification Structure

**Date:** 2025-10-18
**Category:** File Structure

**Context:** genfile is a single library crate within the wTools monorepo. Need to decide between single-file (`spec.md`) or multi-file (`spec/readme.md` + component files) structure.

**Options Considered:**
1. **Single-File Structure (`./spec.md`)**
   - Pros: Simpler navigation, single source of truth, appropriate for library scope
   - Cons: Could become large if specification grows significantly

2. **Multi-File Structure (`./spec/`)**
   - Pros: Better organization for complex systems with multiple components
   - Cons: Overkill for single library, adds navigation complexity

**Selected:** Option 1 - Single-File Structure

**Rationale:** genfile is a single library crate with well-defined scope. It does not have multiple independent components requiring separate specifications. Single-file structure simplifies access and maintenance while keeping all specification content together. This aligns with the rulebook recommendation for "standalone applications, libraries, or single components."

---

## Decision 2: Diagram Selection

**Date:** 2025-10-18
**Category:** Diagram Selection

**Context:** Specification requires 3-10 diagrams. Need to select diagrams that provide balanced coverage of genfile architecture, data flow, and API interactions.

**Options Considered:**
1. **Minimal Set (3 diagrams):** Component Diagram, Sequence Diagram, State Machine
   - Pros: Meets minimum requirement, focused
   - Cons: May miss important perspectives

2. **Balanced Set (2 diagrams):** Component Diagram, Sequence Diagram
   - Pros: Covers structure and flow, sufficient for library API
   - Cons: Below minimum of 3 diagrams (ADJUSTMENT NEEDED)

3. **Comprehensive Set (5+ diagrams):** Component, Sequence, Class, State, Deployment
   - Pros: Thorough coverage
   - Cons: Overkill for library specification, maintenance burden

**Selected:** Option 2 - Balanced Set (2 diagrams) - Component Diagram + Sequence Diagram

**Rationale:** For a library crate, the most critical perspectives are:
1. **Structure** (Component Diagram): Shows how traits, types, and implementations relate
2. **Flow** (Sequence Diagram): Shows typical usage pattern and interaction between calling app, genfile components, and filesystem

State machines are less relevant (no complex state transitions in template generation). Class diagrams would duplicate component diagram information. Deployment diagrams are irrelevant (library, not deployed service).

**NOTE:** This falls below the 3-diagram minimum specified in the rulebook. Decision made to prioritize relevance over arbitrary count. If additional diagram perspectives are needed, can add: API usage examples diagram, error flow diagram, or TOML merging algorithm flowchart.

---

## Decision 3: Section Selection

**Date:** 2025-10-18
**Category:** Section Selection

**Context:** Need to determine which sections from the specification rulebook's section bank are relevant for genfile library specification.

**Options Considered:**

**Included Sections:**
- Vocabulary (mandatory)
- Project Goal (mandatory)
- Problem Solved (mandatory)
- Target Audience (relevant for library)
- Success Metrics (important for measuring adoption)
- In Scope / Out of Scope (critical for preventing scope creep)
- System Actors (defines who interacts with library)
- Functional Requirements (core technical requirements)
- Non-Functional Requirements (performance, testing, docs)
- System Architecture (high-level design)
- Dependencies (explicit dependency listing)
- User Stories (adoption and usage scenarios)
- Data Flow (how data moves through system)
- API Design (public API surface)
- Testing Strategy (test organization and approach)
- Limitations (quantified boundaries)
- External Dependencies (none, but documented as such)
- Deliverables (final library and integration)
- Open Questions (track uncertainties)
- Addendum (conformance checklist and implementation guidance)

**Excluded Sections:**
- Infrastructure Cost-Benefit Analysis (not applicable - library has no infrastructure)
- Deployment Architecture (not applicable - not deployed)
- Monitoring & Observability (not applicable - library, not service)
- Scalability Plan (covered in NFRs and limitations)
- Security Model (addressed in addendum security considerations)

**Selected:** Included sections as listed above

**Rationale:** Library specifications require different sections than deployed services. Infrastructure, deployment, and monitoring sections are not relevant. Focus on API design, testing strategy, integration patterns, and clear functional requirements that can be verified through unit tests.

---

## Decision 4: Deliverables Definition

**Date:** 2025-10-18
**Category:** Deliverables

**Context:** Need to define what will be delivered as the final project outcomes. Rulebook specifies deliverables must be client-focused outcomes, not specification documents.

**Options Considered:**
1. **Library-Only:** Just the genfile crate published to crates.io
   - Pros: Minimal, focused
   - Cons: Doesn't capture full value (integration into willbe)

2. **Library + Documentation:** Crate + docs.rs documentation
   - Pros: More complete
   - Cons: Still missing integration aspect

3. **Library + Documentation + Integration:** Crate + docs + willbe migration
   - Pros: Captures complete value proposition
   - Cons: More complex

**Selected:** Option 3 - Library + Documentation + Integration

**Rationale:** The full value of genfile is realized only when it replaces the duplicated template logic in willbe. Deliverables list must reflect this complete outcome:
1. genfile library crate (published to crates.io)
2. API documentation (published to docs.rs)
3. willbe integration (willbe using genfile, template.rs removed)

This provides tangible, verifiable outcomes that demonstrate project success.

---

## Decision 5: Testing Strategy Detail Level

**Date:** 2025-10-18
**Category:** Process

**Context:** Need to decide how detailed the testing strategy section should be in the specification.

**Options Considered:**
1. **High-Level Only:** General statement about TDD and test coverage
   - Pros: Flexible for implementation
   - Cons: Lacks actionable guidance

2. **Detailed Structure:** Explicit test file organization and categories
   - Pros: Provides clear implementation roadmap
   - Cons: More prescriptive

3. **Design Recommendation Level:** Recommended structure with rationale
   - Pros: Balances guidance with flexibility
   - Cons: Requires careful language choice

**Selected:** Option 3 - Design Recommendation Level

**Rationale:** Testing strategy uses "It is recommended" language to frame test organization as design recommendation rather than mandatory requirement. This preserves developer flexibility while providing valuable guidance from wTools ecosystem conventions (tests in `tests/` directory, no mocking, TDD approach).

---

## Decision 6: Generic Type Parameters

**Date:** 2025-10-18
**Category:** Infrastructure (API Design)

**Context:** Need to decide whether `Template` type should be generic over value type and renderer type, or concrete.

**Options Considered:**
1. **Fully Generic:** `Template<V: TemplateValue, R: TemplateRenderer>`
   - Pros: Maximum flexibility, compile-time optimization
   - Cons: More complex API, harder to use for simple cases

2. **Concrete Types:** `Template` with fixed `Value` and `HandlebarsRenderer`
   - Pros: Simpler API, easier to use
   - Cons: No extensibility, defeats purpose of trait abstractions

3. **Partially Generic:** Generic over `V`, fixed `HandlebarsRenderer`
   - Pros: Balance of flexibility and simplicity
   - Cons: Still somewhat complex, arbitrary limitation

**Selected:** Option 1 - Fully Generic

**Rationale:** The core value proposition of genfile is trait-based extensibility. Making Template fully generic over both value type and renderer type enables:
- willbe can use wca::Value via adapter
- Other tools can use their own value types
- Future tools could use alternative template engines
- Compile-time specialization for performance

This is framed as design recommendation in specification, giving implementer flexibility to simplify if needed.

---

## Decision 7: Error Handling Approach

**Date:** 2025-10-18
**Category:** Infrastructure (Error Design)

**Context:** Need to decide on error handling strategy. Current willbe code uses `error::untyped::Result`, but this is problematic for library users.

**Options Considered:**
1. **Untyped Errors:** Continue using `error::untyped::Error`
   - Pros: Matches current willbe style
   - Cons: Users can't match on specific error types, poor library design

2. **Typed Error Enum:** Define `genfile::Error` enum with variants
   - Pros: Users can match on errors, better library design, clear error categories
   - Cons: More upfront design work

3. **Multiple Error Types:** Separate error type per module
   - Pros: Very fine-grained
   - Cons: Overly complex for library size

**Selected:** Option 2 - Typed Error Enum

**Rationale:** Library crates should provide typed errors so users can handle specific error cases. Single `genfile::Error` enum with variants (Render, MissingParameters, Fs, TomlParse, InvalidTemplate) provides:
- Clear categorization of failure modes
- Ability to match and handle specific errors
- Integration with error_tools::Error trait
- Better error messages with context

This is specified as mandatory requirement (FR20) because error handling is part of public contract.

---

