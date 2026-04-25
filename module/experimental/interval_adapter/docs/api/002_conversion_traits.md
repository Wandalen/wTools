# API: Conversion Traits

### Scope

- **Purpose**: Document the three helper traits that enable bound conversion, endpoint constraint bundling, and canonical interval creation.
- **Responsibility**: Canonical reference for `BoundExt`, `EndPointTrait`, and `IntoInterval` — operations and contracts.
- **In Scope**: Trait operations, bound conversion semantics, endpoint type requirements, and the canonical conversion contract.
- **Out of Scope**: Interval query traits (→ `api/001`); canonical type layout (→ `data_structure/001`); design rationale (→ `pattern/`).

### Abstract

Three auxiliary traits supporting the core interval abstraction. `EndPointTrait<T>` bundles the constraints all interval endpoint types must satisfy. `BoundExt<T>` extends `core::ops::Bound` with closed-form conversion methods. `IntoInterval<T>` converts any interval type to the canonical `Interval<T>` representation.

### EndPointTrait

Constraint bundle for interval endpoint types. Blanket-implemented for all types satisfying all of: partial ordering, subtraction and addition (both returning `T`), `Clone`, `Copy`, and `Sized`. No methods — exists solely as a combined bound to avoid repeating five constraints everywhere.

### BoundExt

Extension methods on the standard `Bound` type, enabling conversion to closed-interval endpoint values. Provides the left/right asymmetric conversion logic required for closed-interval arithmetic. Implemented by providing the trait because `Bound` is a foreign type that cannot receive new methods directly.

| Method | Returns | Notes |
|--------|---------|-------|
| `into_left_closed` | `T` | Convert bound to a left endpoint of a closed interval |
| `into_right_closed` | `T` | Convert bound to a right endpoint of a closed interval |

#### Bound conversion table

| Input Bound | `into_left_closed` | `into_right_closed` |
|-------------|---------------------|----------------------|
| `Included(x)` | `x` | `x` |
| `Excluded(x)` | `x + 1` (next value) | `x - 1` (previous value) |
| `Unbounded` | minimum sentinel value | maximum sentinel value |

### IntoInterval

Conversion trait for producing a canonical `Interval<T>` from any interval type. Blanket-implemented for all types implementing `NonIterableInterval<T>`.

| Method | Returns | Notes |
|--------|---------|-------|
| `into_interval` | `Interval<T>` | Consume self and produce canonical interval |

### Re-exports

`Bound` and `RangeBounds` from the standard library are re-exported at the crate root. Callers do not need to import from `core::ops` directly when using `interval_adapter`.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| api/001 | [Interval Traits](001_interval_traits.md) | `NonIterableInterval` and `IterableInterval` using these traits |
| data_structure/001 | [Interval](../data_structure/001_interval.md) | Canonical type constructed via `IntoInterval` |
| invariant/001 | [Integer Endpoints Only](../invariant/001_integer_endpoints_only.md) | `EndPointTrait` constraint excludes floats |
