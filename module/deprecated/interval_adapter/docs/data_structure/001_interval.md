# Data Structure: Interval

### Scope

- **Purpose**: Define the canonical interval representation that stores any bounded or unbounded interval as a pair of bound values.
- **Responsibility**: Single source of truth for the canonical interval type — field layout, construction, implemented capabilities, and iteration contract.
- **In Scope**: Struct fields, construction API, capabilities, and iteration contract.
- **Out of Scope**: Iterator internals (→ `data_structure/002`); trait query API (→ `api/001`); conversion design rationale (→ `pattern/002`).

### Abstract

The canonical interval type stores any interval as a pair of bound values (left and right). All standard range types and user-defined intervals convert to this type via the conversion trait. Iteration is closed-inclusive — both endpoints are included in the output sequence.

### Structure

| Field | Notes |
|-------|-------|
| `_left` | Left endpoint stored as a bound value; may be Included, Excluded, or Unbounded |
| `_right` | Right endpoint stored as a bound value; may be Included, Excluded, or Unbounded |

Both fields are private. Access is via the trait methods from the non-iterable-interval trait (left, right, closed_left, closed_right, canonical, etc.).

**Type parameter:** defaults to the platform-native signed integer type. Must satisfy the endpoint constraint trait — integer-like types with arithmetic support.

### Operations

The canonical interval type is constructed in two ways:

- **Explicit construction** — from two bound values directly.
- **Via conversion** — any type implementing the non-iterable-interval trait converts via the conversion method.

No validation is performed on construction — the caller is responsible for ensuring the left endpoint does not exceed the right if a non-empty iteration is required.

The canonical interval type supports the following capabilities:

| Capability | Notes |
|-----------|-------|
| Bound queries | All bound-query methods (left, right, bounds, closed, canonical, etc.) |
| Iteration | Bounded — also provides iteration via the iteration protocol |
| From half-open range | Half-open range; excluded right adjusted to closed |
| From closed range | Closed range converted directly |
| Equality | Equality by both bound values |
| Derived traits | Debug, Clone, Copy — all derived |

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [data_structure/002_interval_iterator.md](002_interval_iterator.md) | Iterator produced by into_iter |
| doc | [api/001_interval_traits.md](../api/001_interval_traits.md) | Traits the canonical interval implements |
| doc | [api/002_conversion_traits.md](../api/002_conversion_traits.md) | Conversion trait used for construction |
| doc | [feature/001_generic_interval_parameter.md](../feature/001_generic_interval_parameter.md) | Storage type for converted intervals |
| doc | [feature/003_no_std_support.md](../feature/003_no_std_support.md) | Stack-allocated canonical type in no-standard-library context |
| doc | [invariant/002_no_validation.md](../invariant/002_no_validation.md) | No validation on construction |
| doc | [pattern/002_canonical_interval_type.md](../pattern/002_canonical_interval_type.md) | Design rationale |
