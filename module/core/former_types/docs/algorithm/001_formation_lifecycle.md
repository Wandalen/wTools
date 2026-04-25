# Algorithm: Formation Lifecycle

### Scope

- **Purpose**: Describe the four-step procedure by which a builder accumulates state and produces a formed entity.
- **Responsibility**: Define the ordered sequence of phases, their inputs and outputs, and the ordering guarantees between them.
- **In Scope**: Begin phase, accumulation phase, mutation phase, completion phase.
- **Out of Scope**: Trait signatures, storage internals, collection-specific behavior.

### Abstract

Formation is the process by which a builder converts an empty storage into a fully constructed entity. It proceeds through four sequential phases. Phases execute in strict order; no phase may begin before its predecessor completes. The lifecycle applies equally to top-level struct formation and to nested sub-former formation — both follow the same four steps.

### Algorithm

**Phase 1 — Begin**

A builder instance is initialized with three inputs: an optional initial storage (defaulting to empty if absent), an optional context (carries parent-builder state for nested formation), and a completion handler (determines what happens when formation ends). The output is a ready-to-accumulate builder instance.

**Phase 2 — Accumulation**

Builder methods are called in any order and any number of times. Each call writes one or more field values into the storage container. No ordering constraint exists between individual field writes. At the end of accumulation the storage holds all values that will participate in formation.

**Phase 3 — Mutation**

Immediately before completion, the mutator hook receives mutable access to both the storage and the context. It may apply defaults to absent fields, validate contents, or transform values. The default implementation is a no-op; entity-specific definitions override it as needed. Mutation is guaranteed to execute exactly once, after the last accumulation write and before the completion handler.

**Phase 4 — Completion**

The completion handler receives the fully accumulated storage and the optional context. It returns the final formed entity. Common implementations: call preform on the storage and return the result; return the storage itself as the formed value; invoke a closure for custom conversion.

**Ordering guarantee**: Mutation always runs after the last accumulation write. Completion always runs after mutation. The ordering between any two accumulation writes is determined solely by the call order at the use site.

### Sources

| File | Notes |
|------|-------|
| [../../spec.md](../../spec.md) | Combined source; architecture and formation lifecycle sections extracted here |

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | ../../src/lib.rs | Crate root wiring all formation modules |
| doc | api/001_former_definition.md | Definition traits providing the type set for each phase |
| doc | api/002_formation_process.md | FormingEnd, FormerMutator, FormerBegin traits for phases 1, 3, 4 |
| doc | api/003_storage.md | Storage container used in phases 2, 3, and 4 |
| doc | invariant/001_formation_integrity.md | Correctness property this lifecycle must maintain |
| doc | feature/001_builder_trait_infrastructure.md | End-to-end capability this algorithm powers |
