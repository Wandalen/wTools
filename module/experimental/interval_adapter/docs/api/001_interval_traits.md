# API: Interval Traits

### Scope

- **Purpose**: Document the public interval trait hierarchy — the two traits that unify all Rust range types under a single interface.
- **Responsibility**: Canonical reference for `NonIterableInterval` and `IterableInterval` — operations, type parameter requirements, and the complete implementor table.
- **In Scope**: Trait operations, return types, type parameter requirements, and the complete implementor table.
- **Out of Scope**: Conversion traits (→ `api/002`); canonical type layout (→ `data_structure/001`); design rationale (→ `pattern/001`).

### Abstract

Two-trait hierarchy providing a unified interface for all Rust range types. `NonIterableInterval` covers all intervals including unbounded ones; `IterableInterval` extends it to require iteration capability, restricting to bounded ranges only. All standard `std::ops` range types and several additional representations implement one or both traits.

### Operations

#### NonIterableInterval

Universal query interface for all intervals, including unbounded ones. Default type parameter is `isize`. Implemented for every range type — bounded and unbounded.

| Method | Returns | Notes |
|--------|---------|-------|
| `left` | left bound | May be Unbounded |
| `right` | right bound | May be Unbounded |
| `bounds` | both bounds as a pair | Tuple of left and right bounds |
| `closed_left` | left endpoint as a closed value | Converted from bound form |
| `closed_right` | right endpoint as a closed value | Converted from bound form |
| `closed_len` | closed-form length | `closed_right - closed_left + 1` |
| `closed` | both endpoints as closed values | Pair of left and right in closed form |
| `canonical` | canonical interval | Converted to canonical storage type |

#### IterableInterval

Extends `NonIterableInterval` with iteration capability, restricting to bounded intervals only. The compiler enforces this boundary — unbounded types never implement `IterableInterval`. No additional methods beyond those inherited from `NonIterableInterval` and the iteration protocol.

#### Implementors

| Type | `NonIterableInterval` | `IterableInterval` | Notes |
|------|-----------------------|--------------------|-------|
| `Range<T>` (`0..4`) | ✅ | ✅ | Half-open |
| `RangeInclusive<T>` (`0..=3`) | ✅ | ✅ | Closed |
| `(T, T)` | ✅ | ✅ | Both endpoints included |
| `[T; 2]` | ✅ | ✅ | Both endpoints included |
| `(Bound<T>, Bound<T>)` | ✅ | ✅ | Custom bounds |
| `[Bound<T>; 2]` | ✅ | ✅ | Custom bounds |
| `Interval<T>` | ✅ | ✅ | Canonical type |
| `RangeTo<T>` (`..4`) | ✅ | ❌ | Unbounded left |
| `RangeToInclusive<T>` (`..=3`) | ✅ | ❌ | Unbounded left |
| `RangeFrom<T>` (`0..`) | ✅ | ❌ | Unbounded right |
| `RangeFull` (`..`) | ✅ | ❌ | Fully unbounded |

### Error Handling

Neither `NonIterableInterval` nor `IterableInterval` defines fallible operations. All bound-query methods return values unconditionally. Callers are responsible for ensuring endpoints are valid before performing derived arithmetic (see `invariant/002`).

### Compatibility Guarantees

- Trait implementations for all listed `std::ops` range types are stable across patch and minor releases.
- The default type parameter (`isize`) is stable; changing it requires a major version bump.
- New range types may implement these traits in future minor releases without breaking existing callers.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [api/002_conversion_traits.md](002_conversion_traits.md) | Conversion traits: BoundExt, EndPointTrait, IntoInterval |
| doc | [data_structure/001_interval.md](../data_structure/001_interval.md) | Canonical type implementing both traits |
| doc | [data_structure/002_interval_iterator.md](../data_structure/002_interval_iterator.md) | Iterator type produced by IterableInterval |
| doc | [feature/001_generic_interval_parameter.md](../feature/001_generic_interval_parameter.md) | Accepting any interval type as a function parameter |
| doc | [feature/002_non_iterable_intervals.md](../feature/002_non_iterable_intervals.md) | Non-iterable interval support and NonIterableInterval usage |
| doc | [invariant/001_integer_endpoints_only.md](../invariant/001_integer_endpoints_only.md) | Integer-compatibility constraint on endpoint types |
| doc | [invariant/003_no_set_operations.md](../invariant/003_no_set_operations.md) | No set-algebraic operations in the trait surface |
| doc | [pattern/001_two_trait_hierarchy.md](../pattern/001_two_trait_hierarchy.md) | Design rationale for the trait split |
