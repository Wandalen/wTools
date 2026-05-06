# Feature: Generic Interval Parameter

### Scope

- **Purpose**: Enable writing generic functions that accept any interval type as a parameter without committing to a concrete type.
- **Responsibility**: Documents the pattern of using the iterable-interval and non-iterable-interval bounds as generic function parameters, including all supported input types and conversion examples.
- **In Scope**: Iterable-interval and non-iterable-interval function parameters, supported input types, and canonical conversion examples.
- **Out of Scope**: Iterator step behavior (→ `data_structure/002`); unbounded intervals (→ `feature/002`); no_std context (→ `feature/003`).

### Abstract

Instead of accepting a specific bounded range type, callers declare a generic function parameter bounded by the iterable-interval trait. The function then accepts any bounded interval type — including standard ranges, tuples, arrays, and the canonical interval type — without any change to the call site.

### Design

| Use case | Bound to use | Notes |
|----------|--------------|-------|
| Iterate over values | Iterable-interval bound | Enables iteration in the function body |
| Query bounds only | Non-iterable-interval bound | Accepts both bounded and unbounded intervals |
| Store interval generically | Canonical interval type | Convert at the call site via the conversion method |

#### Usage Example

Declare the function parameter with the iterable-interval bound. Callers may pass any bounded interval representation — half-open ranges, closed ranges, tuples, or two-element arrays. The function body iterates over the values using the standard iteration protocol; all four representations produce identical output.

#### Converting to Canonical Form

When a function must store or return an interval, declare the parameter with the conversion trait bound and return the canonical interval type. Any interval type converts at the call site via the conversion method.

### Constraints

- Generic parameters using the iterable-interval bound are monomorphized — each concrete type produces a separate instantiation.
- Trait objects over the iterable-interval bound are not supported because the iteration protocol is not object-safe; use the canonical interval type for dynamic dispatch scenarios.
- Tuples and arrays as intervals always treat both endpoints as included (closed on both sides).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [api/001_interval_traits.md](../api/001_interval_traits.md) | Iterable-interval and non-iterable-interval trait signatures |
| doc | [api/002_conversion_traits.md](../api/002_conversion_traits.md) | Conversion trait for canonical interval creation |
| doc | [data_structure/001_interval.md](../data_structure/001_interval.md) | Storage type for converted intervals |
| doc | [feature/002_non_iterable_intervals.md](002_non_iterable_intervals.md) | Accepting unbounded intervals via the non-iterable-interval bound |
| doc | [invariant/003_no_set_operations.md](../invariant/003_no_set_operations.md) | Set operations absent from the interval interface |
