# Invariant: No Set Operations

### Scope

- **Purpose**: Document that `interval_adapter` provides no set operations — no union, intersection, containment check, or interval arithmetic.
- **Responsibility**: States the no-set-operations invariant, rationale for the focused scope, and guidance on alternatives.
- **In Scope**: Explicit list of absent operations, rationale, and pointer to alternatives.
- **Out of Scope**: No-validation boundary (→ `invariant/002`); integer endpoint constraint (→ `invariant/001`); feature specifications (→ `feature/`).

### Abstract

`interval_adapter` is a type-unification abstraction, not a computational geometry library. It provides a unified trait interface and canonical storage type for standard range types, but deliberately excludes all operations that treat intervals as mathematical sets.

### Invariant Statement

The following operations are not provided and will never be added to the core crate scope:

| Absent Operation | Category |
|-----------------|----------|
| Containment check | Set membership |
| Overlap test | Set relation |
| Union | Set algebra |
| Intersection | Set algebra |
| Difference | Set algebra |
| Interval + scalar | Interval arithmetic |
| Interval × scalar | Interval arithmetic |
| Interval + interval | Interval arithmetic |

### Rationale

Each absent operation belongs to a different problem domain — computational geometry, interval arithmetic, or set theory — that is wider than the crate's scope of providing a unified type interface for range types. Adding them would:
1. Significantly expand the API surface and maintenance burden.
2. Require design decisions (e.g., how to represent union of disjoint intervals) that belong in specialized crates.
3. Conflict with the zero-dependency, no-standard-library mandate for allocation-free environments.

### Enforcement Mechanism

This invariant is structural: the absent methods do not exist. No runtime enforcement is needed.

### Violation Consequences

This invariant cannot be violated — the absent methods cannot be called. Callers needing set operations should use a dedicated intervals crate (e.g., `ranges`, `interval`, `gcollections`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/001_generic_interval_parameter.md](../feature/001_generic_interval_parameter.md) | What the crate does provide |
| doc | [feature/002_non_iterable_intervals.md](../feature/002_non_iterable_intervals.md) | Bound-query operations that are in scope |
| doc | [api/001_interval_traits.md](../api/001_interval_traits.md) | Complete method table — scope of API |
