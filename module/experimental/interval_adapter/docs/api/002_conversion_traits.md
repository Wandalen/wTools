# API: Conversion Traits

### Scope

- **Purpose**: Document the three helper traits that enable bound conversion, endpoint constraint bundling, and canonical interval creation.
- **Responsibility**: Canonical reference for `BoundExt`, `EndPointTrait`, and `IntoInterval` — operations and contracts.
- **In Scope**: Trait operations, bound conversion semantics, endpoint type requirements, and the canonical conversion contract.
- **Out of Scope**: Interval query traits (→ `api/001`); canonical type layout (→ `data_structure/001`); design rationale (→ `pattern/`).

### Abstract

Three auxiliary traits supporting the core interval abstraction. `EndPointTrait` bundles the constraints all interval endpoint types must satisfy. `BoundExt` extends the standard `Bound` type with closed-form conversion methods. `IntoInterval` converts any interval type to the canonical `Interval` representation.

### Operations

#### EndPointTrait

Constraint bundle for interval endpoint types. Blanket-implemented for all types satisfying all of: partial ordering, subtraction and addition (both returning `T`), `Clone`, `Copy`, and `Sized`. No methods — exists solely as a combined bound to avoid repeating five constraints everywhere.

#### BoundExt

Extension methods on the standard `Bound` type, enabling conversion to closed-interval endpoint values. Provides the left/right asymmetric conversion logic required for closed-interval arithmetic. Implemented by providing the trait because `Bound` is a foreign type that cannot receive new methods directly.

| Method | Returns | Notes |
|--------|---------|-------|
| `into_left_closed` | left closed endpoint value | Convert bound to a left endpoint of a closed interval |
| `into_right_closed` | right closed endpoint value | Convert bound to a right endpoint of a closed interval |

**Bound conversion:**

| Input Bound | `into_left_closed` | `into_right_closed` |
|-------------|---------------------|----------------------|
| `Included(x)` | `x` | `x` |
| `Excluded(x)` | `x + 1` (next value) | `x - 1` (previous value) |
| `Unbounded` | minimum sentinel value | maximum sentinel value |

#### IntoInterval

Conversion trait for producing a canonical interval from any interval type. Blanket-implemented for all types implementing `NonIterableInterval`.

| Method | Returns | Notes |
|--------|---------|-------|
| `into_interval` | canonical interval | Consume self and produce canonical interval |

### Re-exports

`Bound` and `RangeBounds` from the standard library are re-exported at the crate root. Callers do not need to import from `core::ops` directly when using `interval_adapter`.

### Error Handling

All conversion methods are infallible — `into_left_closed`, `into_right_closed`, and `into_interval` return values unconditionally. For `Unbounded` bounds, sentinel values (`isize::MIN` / `isize::MAX`) are returned rather than an error.

### Compatibility Guarantees

- The sentinel values for `Unbounded` bounds are fixed at `isize::MIN` and `isize::MAX` and will not change across minor releases.
- `EndPointTrait` is blanket-implemented and callers do not implement it directly; additions to its constraints require a major version bump.
- `IntoInterval` is blanket-implemented for all `NonIterableInterval` types; new implementations are non-breaking additions.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [api/001_interval_traits.md](001_interval_traits.md) | NonIterableInterval and IterableInterval using these traits |
| doc | [data_structure/001_interval.md](../data_structure/001_interval.md) | Canonical type constructed via IntoInterval |
| doc | [feature/001_generic_interval_parameter.md](../feature/001_generic_interval_parameter.md) | Generic parameter pattern using IntoInterval for storage |
| doc | [feature/002_non_iterable_intervals.md](../feature/002_non_iterable_intervals.md) | BoundExt sentinel values for Unbounded ends |
| doc | [invariant/001_integer_endpoints_only.md](../invariant/001_integer_endpoints_only.md) | EndPointTrait constraint excludes floats |
| doc | [pattern/002_canonical_interval_type.md](../pattern/002_canonical_interval_type.md) | IntoInterval as the conversion entry point |
