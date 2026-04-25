# API: Interval Traits

### Scope

- **Purpose**: Document the public interval trait hierarchy — the two traits that unify all Rust range types under a single interface.
- **Responsibility**: Canonical reference for `NonIterableInterval` and `IterableInterval` — operations, type parameter requirements, and the complete implementor table.
- **In Scope**: Trait operations, return types, type parameter requirements, and the complete implementor table.
- **Out of Scope**: Conversion traits (→ `api/002`); canonical type layout (→ `data_structure/001`); design rationale (→ `pattern/001`).

### Abstract

Two-trait hierarchy providing a unified interface for all Rust range types. `NonIterableInterval<T>` covers all intervals including unbounded ones; `IterableInterval<T>` extends it to require iteration capability, restricting to bounded ranges only. All standard `std::ops` range types and several additional representations implement one or both traits.

### NonIterableInterval

Universal query interface for all intervals, including unbounded ones. Default type parameter is `isize`. Implemented for every range type — bounded and unbounded.

| Method | Returns | Notes |
|--------|---------|-------|
| `left` | `Bound<T>` | Left endpoint as stored; may be `Unbounded` |
| `right` | `Bound<T>` | Right endpoint as stored; may be `Unbounded` |
| `bounds` | `(Bound<T>, Bound<T>)` | Both bounds as a tuple |
| `closed_left` | `T` | Left converted to a closed-interval value |
| `closed_right` | `T` | Right converted to a closed-interval value |
| `closed_len` | `T` | Closed-form length: `closed_right - closed_left + 1` |
| `closed` | `(T, T)` | Both endpoints in closed form |
| `canonical` | `Interval<T>` | Convert to the canonical storage type |

### IterableInterval

Extends `NonIterableInterval<T>` with `IntoIterator`, restricting to bounded intervals only. The compiler enforces this boundary — unbounded types never implement `IterableInterval`. No additional methods beyond those inherited from `NonIterableInterval` and `IntoIterator`.

### Implementors

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

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| api/002 | [Conversion Traits](002_conversion_traits.md) | `BoundExt`, `EndPointTrait`, `IntoInterval` |
| data_structure/001 | [Interval](../data_structure/001_interval.md) | Canonical type implementing both traits |
| pattern/001 | [Two-Trait Hierarchy](../pattern/001_two_trait_hierarchy.md) | Design rationale for the split |
| invariant/001 | [Integer Endpoints Only](../invariant/001_integer_endpoints_only.md) | Constraint on valid endpoint types |
