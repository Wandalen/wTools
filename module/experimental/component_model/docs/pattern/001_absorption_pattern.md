# Pattern: Absorption Pattern

### Scope

- **Purpose**: Explain why the component model feature requires three separate crates and how their roles are separated.
- **Responsibility**: Documents the absorption pattern as applied in this ecosystem — the problem it solves, how the three crates are structured, and the trade-offs involved.
- **In Scope**: The three-crate split, dependency directions, and circular dependency prevention.
- **Out of Scope**: The feature itself (→ `feature/001_component_model.md`); derive macro internals (→ `component_model_meta/docs/`).

### Problem

A procedural macro crate and a runtime crate often need to share type definitions. A naive two-crate split creates a circular dependency:

- The runtime crate depends on the macro crate (to get derive macros)
- The macro crate wants to depend on the runtime crate (to get shared type definitions)

Rust forbids circular crate dependencies at compile time. This forces a structural solution.

### Solution

Extract the shared type definitions into a third, independent crate. The dependency graph becomes a directed acyclic graph:

- **Types crate** (`component_model_types`) — defines only types and traits; no dependency on either other crate
- **Macro crate** (`component_model_meta`) — depends on the types crate for trait definitions used in generated code
- **Runtime crate** (`component_model`) — depends on both; re-exports everything under a single public API

The runtime crate "absorbs" the other two — it adds no implementation of its own, only re-exports. Users import from the runtime crate and see a unified API without being aware of the three-crate structure beneath.

### Applicability

Apply this pattern when:
- A proc-macro crate and a runtime crate need to share types (traits, enums, structs)
- Both crates have external consumers who should use the same type definitions
- You want to present a single-crate API to users despite the internal split

Do not apply when:
- A simple two-crate split suffices (proc-macro + runtime, with no shared types needed in generated code)
- The shared types can be placed entirely in the macro crate (which is unusual)

### Consequences

**Benefits:**
- No circular dependencies — types crate has no upstream dependencies within the ecosystem
- Single import for users — one `use component_model::*` gives everything
- Consistent versioning — all three crates are versioned together
- Clear responsibility boundaries — types vs macros vs aggregation

**Trade-offs:**
- Extra crate in the workspace — three crates instead of two
- Users who want only types (no derive macros) must depend on the types crate directly
- Versioning discipline required — breaking changes to types crate break all three crates simultaneously

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | The absorption crate — re-exports only, no implementation |
| source | `component_model_meta/src/lib.rs` | Macro crate depending on types crate |
| source | `component_model_types/src/component.rs` | Types crate — shared trait definitions |
| doc | [feature/001_component_model.md](../feature/001_component_model.md) | User-facing feature enabled by this pattern |
