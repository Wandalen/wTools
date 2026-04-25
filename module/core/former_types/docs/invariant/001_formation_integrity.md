# Invariant: Formation Integrity

### Scope

- **Purpose**: Guarantee that any sequence of builder accumulation calls produces a valid formed entity.
- **Responsibility**: Define what valid means for a formed entity and how the framework enforces it at compile time.
- **In Scope**: Storage default-constructibility, mutation-before-completion ordering, formed-entity type consistency.
- **Out of Scope**: Collection ordering, entry uniqueness, application-level validation logic.

### Invariant Statement

For every formation sequence — regardless of which fields are set, how many times they are set, or in what order — formation completion produces a value of the declared formed-entity type. Formation never silently discards accumulated state or produces a value of an unexpected type. Specifically: storage is always initializable before accumulation begins, mutation always runs after the last accumulation write and before completion, and completion always receives the full accumulated storage.

### Enforcement Mechanism

Three compile-time constraints enforce this invariant jointly.

Storage default-constructibility: the storage type is bounded to require default construction. Formation cannot begin without this bound satisfied; there is no code path that initializes a builder with uninitialized or borrowed storage.

Mutation ordering: the mutation hook is called inside the completion method, sharing the same storage reference, in a single non-interruptible call chain. There is no code path that invokes the completion handler before the mutation hook, and no code path that allows the storage to be observed between mutation and completion.

Formed-entity type consistency: the formed-entity type is fixed as an associated type at definition time. The completion handler's return type must match this associated type. A completion handler returning a different type fails to satisfy the bound and does not compile.

### Violation Consequences

Absent storage default-construction bound: formation could begin with uninitialized memory, producing undefined or invalid field values in the formed entity.

Absent mutation-before-completion ordering: formation could complete before the mutation hook runs, causing field defaults, validation, or transformations to be silently skipped.

Formed-entity type mismatch: callers would receive a value whose type does not match the declared formed-entity type, breaking type safety at the call site.

### Sources

| File | Notes |
|------|-------|
| [../../spec.md](../../spec.md) | Combined source; design rationale and architecture sections extracted here |

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | api/001_former_definition.md | Definition traits where storage and formed-entity bounds are declared |
| doc | api/003_storage.md | Storage trait enforcing default-constructibility |
| doc | algorithm/001_formation_lifecycle.md | Lifecycle procedure this invariant governs |
