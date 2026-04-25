# Invariant: Usage Constraints

### Scope

- **Purpose**: Document required usage patterns the type system cannot fully enforce.
- **Responsibility**: Specify caller obligations for correct use of `clone_into_box` and `CloneDyn`.
- **In Scope**: Caller obligations for `clone_into_box` and `CloneDyn`-bound traits.
- **Out of Scope**: Internal unsafe block invariants (see `invariant/002_memory_safety.md`).

### Invariant Statement

The following constraints MUST be satisfied by all callers:
1. **DST coercion**: Slices (`[T]`) and `str` MUST be passed via double-reference
   (`&&[T]`, `&&str`) to coerce to `&dyn CloneDyn`; single `&` produces a compile error.
2. **Box Clone impl**: Users who add `CloneDyn` as a bound on a trait MUST manually
   implement `Clone for Box<dyn Trait>`; no automatic derivation exists.
3. **Tuple arity**: `CloneDyn` is not implemented for tuples with arity > 12 because
   Rust std does not implement `Clone` for them.

### Enforcement Mechanism

- Constraint 1: compile-time type error (E0277) — self-enforcing.
- Constraint 2: missing-impl compile error — self-enforcing.
- Constraint 3: compile-time trait bound failure — self-enforcing.

All three constraints are enforced by the Rust compiler. No runtime checks required.

### Violation Consequences

- Constraint 1 violation: compile error E0277 (`the trait CloneDyn is not implemented`).
- Constraint 2 violation: missing trait impl compile error on `Box<dyn Trait>: Clone`.
- Constraint 3 violation: compile error — `CloneDyn` not satisfied for tuples of arity > 12.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `../api/002_clone_into_box.md` | Function where constraint 1 applies |
| doc | `../feature/003_type_coverage.md` | Type support scope including the arity limit |
