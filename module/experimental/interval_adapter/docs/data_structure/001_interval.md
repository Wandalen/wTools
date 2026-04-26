# Data Structure: Interval

### Scope

- **Purpose**: Define the canonical interval representation that stores any bounded or unbounded interval as a pair of bound values.
- **Responsibility**: Single source of truth for `Interval` — field layout, construction, implemented capabilities, and iteration contract.
- **In Scope**: Struct fields, construction API, capabilities, and iteration contract.
- **Out of Scope**: Iterator internals (→ `data_structure/002`); trait query API (→ `api/001`); conversion design rationale (→ `pattern/002`).

### Abstract

`Interval` is the canonical concrete type that stores any interval as a pair of bound values (left and right). All standard range types and user-defined intervals convert to this type via `IntoInterval`. Iteration is closed-inclusive — both endpoints are included in the output sequence.

### Structure

| Field | Notes |
|-------|-------|
| `_left` | Left endpoint stored as a bound value; may be Included, Excluded, or Unbounded |
| `_right` | Right endpoint stored as a bound value; may be Included, Excluded, or Unbounded |

Both fields are private. Access is via the trait methods on `NonIterableInterval` (`left`, `right`, `closed_left`, `closed_right`, `canonical`, etc.).

**Type parameter:** `T` defaults to `isize`. Must satisfy `EndPointTrait` — integer-like types with arithmetic support.

### Operations

`Interval` is constructed in two ways:

- **`new(left, right)`** — Explicit construction from two bound values.
- **Via conversion** — Any type implementing `NonIterableInterval` converts via `into_interval` or `canonical`.

No validation is performed on construction — the caller is responsible for ensuring `left ≤ right` if a non-empty iteration is required.

`Interval` supports the following capabilities:

| Capability | Notes |
|-----------|-------|
| `NonIterableInterval` | All bound-query methods (left, right, bounds, closed, canonical, etc.) |
| `IterableInterval` | Bounded — also provides iteration via the iteration protocol |
| From half-open range | Half-open range; excluded right adjusted to closed |
| From closed range | Closed range converted directly |
| Equality | Equality by both bound values |
| Debug, Clone, Copy | All derived |

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [data_structure/002_interval_iterator.md](002_interval_iterator.md) | Iterator produced by into_iter |
| doc | [api/001_interval_traits.md](../api/001_interval_traits.md) | Traits Interval implements |
| doc | [api/002_conversion_traits.md](../api/002_conversion_traits.md) | IntoInterval trait used for construction |
| doc | [feature/001_generic_interval_parameter.md](../feature/001_generic_interval_parameter.md) | Storage type for converted intervals |
| doc | [feature/003_no_std_support.md](../feature/003_no_std_support.md) | Stack-allocated canonical type in no_std context |
| doc | [invariant/002_no_validation.md](../invariant/002_no_validation.md) | No validation on construction |
| doc | [pattern/002_canonical_interval_type.md](../pattern/002_canonical_interval_type.md) | Design rationale |
