# Invariant: Integer Endpoints Only

### Scope

- **Purpose**: Guarantee that interval endpoints must satisfy `EndPointTrait<T>`, which excludes floating-point types (`f32`, `f64`).
- **Responsibility**: Documents the constraint, its compile-time enforcement, and the rationale for excluding floats.
- **In Scope**: The `EndPointTrait<T>` constraint, types that satisfy it, types excluded (floats), and the compile-time enforcement mechanism.
- **Out of Scope**: No-validation behavior (→ `invariant/002`); no-set-operations boundary (→ `invariant/003`).

### Abstract

All interval endpoint types must implement `EndPointTrait<T>`, which requires `PartialOrd + Sub<Output = T> + Add<Output = T> + Clone + Copy + Sized`. While `f32` and `f64` technically satisfy these bounds, the integer-compatibility constraint required by `BoundExt`, `NonIterableInterval`, and `IntervalIterator` excludes them in practice, since there is no integer-to-float coercion in the required direction. Integer types (`i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`) all satisfy both constraints.

### Invariant Statement

`interval_adapter` does not provide iteration or closed-form conversion for floating-point endpoint types. `f32` and `f64` do not satisfy the integer-compatibility constraint required by `NonIterableInterval`, `IterableInterval`, and `BoundExt`. Attempts to use float endpoints produce a compile-time error.

### Rationale

Iteration over floating-point intervals is ambiguous: there is no canonical "next" value after `0.1f32`, and step size would require external specification. Closed-form conversion (`Excluded(x)` → `x ± 1`) requires integer arithmetic. Restricting endpoints to integer-like types gives iteration a clear and unambiguous semantics.

### Enforcement Mechanism

The integer-compatibility constraint on `NonIterableInterval`, `IterableInterval`, `BoundExt`, and `IntervalIterator` is evaluated at compile time. Any attempt to instantiate these traits with a float type fails with a trait bound error. No runtime check is required.

### Violation Consequences

This invariant cannot be violated at runtime — it is enforced entirely at compile time by the type system. No defensive code is needed.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [api/001_interval_traits.md](../api/001_interval_traits.md) | Traits with EndPointTrait and integer-compatibility constraint |
| doc | [api/002_conversion_traits.md](../api/002_conversion_traits.md) | EndPointTrait definition and BoundExt constraint |
| doc | [data_structure/002_interval_iterator.md](../data_structure/002_interval_iterator.md) | Iterator with EndPointTrait constraint |
| doc | [invariant/002_no_validation.md](002_no_validation.md) | Orthogonal invariant on construction validation |
| doc | [pattern/001_two_trait_hierarchy.md](../pattern/001_two_trait_hierarchy.md) | Trait split where integer constraint is enforced |
