# Invariant: No Validation

### Scope

- **Purpose**: Document that `interval_adapter` does not validate that left ≤ right and does not prevent construction of reversed intervals.
- **Responsibility**: States the no-validation invariant, the rationale for trusting callers, and the consequences of passing a reversed interval.
- **In Scope**: The absence of left ≤ right checks in all construction paths, rationale, and caller responsibility.
- **Out of Scope**: Integer endpoint restriction (→ `invariant/001`); set-operations boundary (→ `invariant/003`).

### Abstract

`interval_adapter` is a zero-overhead abstraction. No runtime validation is performed at construction time or during iteration. If a caller constructs an interval where the left endpoint is greater than the right, behavior is caller-defined: the iterator produces zero elements immediately, but no panic or error is raised.

### Invariant Statement

All construction paths — explicit construction, conversion from any interval type, and any trait implementation — never validate that the closed left endpoint is less than or equal to the closed right endpoint. The crate trusts callers to supply valid intervals. A reversed interval is accepted without error.

### Rationale

Validation adds runtime overhead (comparison on every construction) and is redundant when callers derive intervals from ranges they already know are valid. Library-level validation is appropriate at system boundaries, not in low-level type abstractions. Callers that require validation should enforce it at their own boundary before passing to `interval_adapter`.

### Enforcement Mechanism

No enforcement is in place — this invariant is a design choice, not a compile-time constraint. The absence of assertion calls in the constructor is deliberate.

### Violation Consequences

If a caller passes a reversed interval (left greater than right):
- The iterator immediately produces no elements — the first call to advance yields nothing.
- The closed-interval length method may return a negative or underflowing value for integer endpoint types.
- No panic occurs; no error is reported.

Callers that depend on non-empty iteration must validate before constructing an interval.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [data_structure/001_interval.md](../data_structure/001_interval.md) | No validation in the canonical interval constructor |
| doc | [data_structure/002_interval_iterator.md](../data_structure/002_interval_iterator.md) | Immediate empty result for reversed intervals |
| doc | [invariant/001_integer_endpoints_only.md](001_integer_endpoints_only.md) | Type constraint (orthogonal invariant) |
