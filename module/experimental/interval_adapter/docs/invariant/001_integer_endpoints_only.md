# Invariant: Integer Endpoints Only

### Scope

- **Purpose**: Guarantee that interval endpoints must satisfy the endpoint constraint trait, which excludes floating-point types.
- **Responsibility**: Documents the constraint, its compile-time enforcement, and the rationale for excluding floats.
- **In Scope**: The endpoint constraint trait, types that satisfy it, types excluded (floating-point), and the compile-time enforcement mechanism.
- **Out of Scope**: No-validation behavior (→ `invariant/002`); no-set-operations boundary (→ `invariant/003`).

### Abstract

All interval endpoint types must implement the endpoint constraint trait, which bundles partial ordering, subtraction, addition, cloneability, copyability, and fixed size. While floating-point types technically satisfy the arithmetic bounds, the integer-compatibility constraint required by the bound extension, non-iterable-interval, and iterator components excludes them in practice. All standard signed and unsigned integer sizes satisfy both constraints.

### Invariant Statement

`interval_adapter` does not provide iteration or closed-form conversion for floating-point endpoint types. Floating-point types do not satisfy the integer-compatibility constraint required by the non-iterable-interval trait, the iterable-interval trait, and the bound extension trait. Attempts to use floating-point endpoints produce a compile-time error.

### Rationale

Iteration over floating-point intervals is ambiguous: there is no canonical "next" value after an arbitrary float, and step size would require external specification. Closed-form conversion from an excluded endpoint requires integer increment or decrement. Restricting endpoints to integer-like types gives iteration a clear and unambiguous semantics.

### Enforcement Mechanism

The integer-compatibility constraint on the non-iterable-interval trait, iterable-interval trait, bound extension trait, and iterator type is evaluated at compile time. Any attempt to instantiate these with a floating-point type fails with a trait bound error. No runtime check is required.

### Violation Consequences

This invariant cannot be violated at runtime — it is enforced entirely at compile time by the type system. No defensive code is needed.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [api/001_interval_traits.md](../api/001_interval_traits.md) | Traits with the endpoint constraint and integer-compatibility requirement |
| doc | [api/002_conversion_traits.md](../api/002_conversion_traits.md) | Endpoint constraint trait definition and bound extension requirement |
| doc | [data_structure/002_interval_iterator.md](../data_structure/002_interval_iterator.md) | Iterator with endpoint constraint on element type |
| doc | [invariant/002_no_validation.md](002_no_validation.md) | Orthogonal invariant on construction validation |
| doc | [pattern/001_two_trait_hierarchy.md](../pattern/001_two_trait_hierarchy.md) | Trait split where integer constraint is enforced |
