# Invariant: No Validation

### Scope

- **Purpose**: Document that `interval_adapter` does not validate that left ≤ right and does not prevent construction of reversed intervals.
- **Responsibility**: States the no-validation invariant, the rationale for trusting callers, and the consequences of passing a reversed interval.
- **In Scope**: The absence of `left ≤ right` checks in all construction paths, rationale, and caller responsibility.
- **Out of Scope**: Integer endpoint restriction (→ `invariant/001`); set-operations boundary (→ `invariant/003`).

### Abstract

`interval_adapter` is a zero-overhead abstraction. No runtime validation is performed at construction time or during iteration. If a caller constructs an interval where `left > right`, behavior is caller-defined: `IntervalIterator` will produce zero elements (immediate `None`), but no panic or error is raised.

### Invariant Statement

Construction via `Interval::new()`, `IntoInterval`, or any `NonIterableInterval` implementation never validates that `closed_left ≤ closed_right`. The crate trusts callers to supply valid intervals. A reversed interval (`left > right`) is accepted without error.

### Rationale

Validation adds runtime overhead (comparison on every construction) and is redundant when callers derive intervals from ranges they already know are valid. Library-level validation is appropriate at system boundaries, not in low-level type abstractions. Callers that require validation should enforce it at their own boundary before passing to `interval_adapter`.

### Enforcement Mechanism

No enforcement is in place — this invariant is a design choice, not a compile-time constraint. The absence of a `debug_assert!(left <= right)` or `assert!` call in `Interval::new()` is deliberate.

### Violation Consequences

If a caller passes a reversed interval (`left > right`):
- `IntervalIterator::next()` immediately returns `None` — iteration produces no elements.
- `closed_len()` may return a negative or underflowing value for integer types.
- No panic occurs; no error is reported.

Callers that depend on non-empty iteration must validate before constructing an interval.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| data_structure/001 | [Interval](../data_structure/001_interval.md) | `Interval::new()` — no validation in constructor |
| data_structure/002 | [IntervalIterator](../data_structure/002_interval_iterator.md) | Immediate `None` for reversed intervals |
| invariant/001 | [Integer Endpoints Only](001_integer_endpoints_only.md) | Type constraint (orthogonal invariant) |
