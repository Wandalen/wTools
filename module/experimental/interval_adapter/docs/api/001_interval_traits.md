# API: Interval Traits

### Scope

- **Purpose**: Document the public interval trait hierarchy — the two traits that unify all interval types under a single interface.
- **Responsibility**: Canonical reference for the non-iterable-interval and iterable-interval traits — operations, type parameter requirements, and the complete implementor table.
- **In Scope**: Trait operations, return types, type parameter requirements, and the complete implementor table.
- **Out of Scope**: Conversion traits (→ `api/002`); canonical type layout (→ `data_structure/001`); design rationale (→ `pattern/001`).

### Abstract

Two-trait hierarchy providing a unified interface for all interval types. The non-iterable-interval trait covers all intervals including unbounded ones; the iterable-interval trait extends it to require iteration capability, restricting to bounded ranges only. All standard range types and several additional representations implement one or both traits.

### Operations

#### NonIterableInterval

Universal query interface for all intervals, including unbounded ones. The default endpoint type is the platform-native signed integer. Implemented for every range type — bounded and unbounded.

| Method | Returns | Notes |
|--------|---------|-------|
| `left` | left bound | May be Unbounded |
| `right` | right bound | May be Unbounded |
| `bounds` | both bounds as a pair | Tuple of left and right bounds |
| `closed_left` | left endpoint as a closed value | Converted from bound form |
| `closed_right` | right endpoint as a closed value | Converted from bound form |
| `closed_len` | closed-form length | Length of the closed interval |
| `closed` | both endpoints as closed values | Pair of left and right in closed form |
| `canonical` | canonical interval | Converted to canonical storage type |

#### IterableInterval

Extends the non-iterable-interval trait with iteration capability, restricting to bounded intervals only. The compiler enforces this boundary — unbounded types never implement the iterable-interval trait. No additional methods beyond those inherited from the non-iterable-interval trait and the iteration protocol.

#### Implementors

| Interval Kind | Non-iterable | Iterable | Notes |
|---------------|:------------:|:--------:|-------|
| Half-open bounded | ✅ | ✅ | `Range<T>` e.g. `0..4` |
| Closed bounded | ✅ | ✅ | `RangeInclusive<T>` e.g. `0..=3` |
| Pair — both inclusive | ✅ | ✅ | `(T, T)` — both endpoints included |
| Array — both inclusive | ✅ | ✅ | `[T; 2]` — both endpoints included |
| Custom bound pair | ✅ | ✅ | `(Bound<T>, Bound<T>)` — custom bounds |
| Custom bound array | ✅ | ✅ | `[Bound<T>; 2]` — custom bounds |
| Canonical interval | ✅ | ✅ | `Interval<T>` — canonical storage type |
| Upper-bounded half-open | ✅ | ❌ | `RangeTo<T>` e.g. `..4` |
| Upper-inclusive-bounded | ✅ | ❌ | `RangeToInclusive<T>` e.g. `..=3` |
| Open-ended | ✅ | ❌ | `RangeFrom<T>` e.g. `0..` |
| Fully unbounded | ✅ | ❌ | `RangeFull` i.e. `..` |

### Error Handling

Neither trait defines fallible operations. All bound-query methods return values unconditionally. Callers are responsible for ensuring endpoints are valid before performing derived arithmetic (see `invariant/002`).

### Compatibility Guarantees

- Trait implementations for all listed range types are stable across patch and minor releases.
- The default endpoint type is stable; changing it requires a major version bump.
- New range types may implement these traits in future minor releases without breaking existing callers.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [api/002_conversion_traits.md](002_conversion_traits.md) | Conversion traits: bound extension, endpoint constraint, canonical conversion |
| doc | [data_structure/001_interval.md](../data_structure/001_interval.md) | Canonical type implementing both traits |
| doc | [data_structure/002_interval_iterator.md](../data_structure/002_interval_iterator.md) | Iterator type produced by the iterable-interval trait |
| doc | [feature/001_generic_interval_parameter.md](../feature/001_generic_interval_parameter.md) | Accepting any interval type as a function parameter |
| doc | [feature/002_non_iterable_intervals.md](../feature/002_non_iterable_intervals.md) | Non-iterable interval support and trait usage |
| doc | [invariant/001_integer_endpoints_only.md](../invariant/001_integer_endpoints_only.md) | Integer-compatibility constraint on endpoint types |
| doc | [invariant/003_no_set_operations.md](../invariant/003_no_set_operations.md) | No set-algebraic operations in the trait surface |
| doc | [pattern/001_two_trait_hierarchy.md](../pattern/001_two_trait_hierarchy.md) | Design rationale for the trait split |
