# Data Structure: Interval

### Scope

- **Purpose**: Define the canonical interval representation that stores any bounded or unbounded interval as a pair of bound values.
- **Responsibility**: Single source of truth for `Interval<T>` — field layout, construction, implemented traits, and iteration contract.
- **In Scope**: Struct fields, construction API, trait implementations, and iteration contract.
- **Out of Scope**: Iterator internals (→ `data_structure/002`); trait query API (→ `api/001`); conversion design rationale (→ `pattern/002`).

### Abstract

`Interval<T>` is the canonical concrete type that stores any interval as a pair of `Bound<T>` values (left and right). All standard range types and user-defined intervals convert to this type via `IntoInterval<T>`. Iteration is closed-inclusive — both endpoints are included in the output sequence.

### Fields

| Field | Type | Notes |
|-------|------|-------|
| `_left` | `Bound<T>` | Left endpoint; may be `Included`, `Excluded`, or `Unbounded` |
| `_right` | `Bound<T>` | Right endpoint; may be `Included`, `Excluded`, or `Unbounded` |

Both fields are private. Access is via the trait methods on `NonIterableInterval` (`left`, `right`, `closed_left`, `closed_right`, `canonical`, etc.).

**Type parameter:** `T` defaults to `isize`. Must satisfy `EndPointTrait<T>` — integer-like types with arithmetic support.

### Construction

`Interval<T>` is constructed in two ways:

- **`new(left, right)`** — Explicit construction from two `Bound<T>` values.
- **Via `IntoInterval`** — Any type implementing `NonIterableInterval<T>` converts via `into_interval()` or `canonical()`.

No validation is performed on construction — the caller is responsible for ensuring `left ≤ right` if a non-empty iteration is required.

### Trait Implementations

| Trait | Notes |
|-------|-------|
| `NonIterableInterval<T>` | All bound-query methods |
| `IterableInterval<T>` | Bounded — also implements `IntoIterator` |
| `IntoIterator` | Produces `IntervalIterator<T>` via `into_iter` |
| `From<Range<T>>` | Half-open range; excluded right adjusted to closed |
| `From<RangeInclusive<T>>` | Closed range converted directly |
| `PartialEq`, `Eq` | Equality by both bound values |
| `Debug`, `Clone`, `Copy` | All derived |

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| data_structure/002 | [IntervalIterator](002_interval_iterator.md) | Iterator produced by `into_iter` |
| api/001 | [Interval Traits](../api/001_interval_traits.md) | Traits `Interval<T>` implements |
| api/002 | [Conversion Traits](../api/002_conversion_traits.md) | `IntoInterval` trait used for construction |
| pattern/002 | [Canonical Interval Type](../pattern/002_canonical_interval_type.md) | Design rationale |
