# Feature: Generic Interval Parameter

### Scope

- **Purpose**: Enable writing generic functions that accept any Rust range or interval type as a parameter without committing to a concrete type.
- **Responsibility**: Documents the pattern of using `IterableInterval` or `NonIterableInterval` as a generic bound, including all supported input types and usage examples.
- **In Scope**: `impl IterableInterval` and `impl NonIterableInterval` function parameters, supported input types, and conversion examples.
- **Out of Scope**: Iterator step behavior (→ `data_structure/002`); unbounded intervals (→ `feature/002`); no_std context (→ `feature/003`).

### Abstract

Instead of accepting `Range<i32>` or `RangeInclusive<i32>` specifically, callers write `impl IterableInterval<i32>`. The function then accepts any bounded interval type — including standard ranges, tuples, arrays, and the canonical `Interval<T>` — without any change to the call site.

### Design

| Use case | Recommended bound | Notes |
|----------|------------------|-------|
| Iterate over values | `impl IterableInterval<T>` | Function body uses `for i in interval { ... }` |
| Query bounds only | `impl NonIterableInterval<T>` | Accepts bounded and unbounded intervals |
| Store interval generically | `Interval<T>` | Convert with `.into_interval()` or `.canonical()` |

#### Usage Example

```rust
use interval_adapter::IterableInterval;

fn print_range( interval : impl IterableInterval ) {
  for i in interval {
    println!( "{i}" );
  }
}

// All four call sites produce identical output:
print_range( 0..4 );           // Range — half-open
print_range( 0..=3 );          // RangeInclusive — closed
print_range( ( 0, 3 ) );       // Tuple — both endpoints included
print_range( [ 0, 3 ] );       // Array — both endpoints included
```

#### Converting to Canonical Form

When a function must store or return an interval, convert to `Interval<T>`:

```rust
use interval_adapter::{ IntoInterval, Interval };

fn clamp_to( range : impl IntoInterval< i32 > ) -> Interval< i32 > {
  range.into_interval()
}
```

### Constraints

- Generic parameters using `impl IterableInterval` are monomorphized — each concrete type produces a separate instantiation.
- Trait objects (`dyn IterableInterval`) are not supported without `IntoIterator` being object-safe; use `Interval<T>` for dynamic dispatch scenarios.
- Tuples and arrays as intervals always treat both endpoints as included (closed on both sides).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| api/001 | [Interval Traits](../api/001_interval_traits.md) | `IterableInterval` and `NonIterableInterval` signatures |
| api/002 | [Conversion Traits](../api/002_conversion_traits.md) | `IntoInterval` for canonical conversion |
| data_structure/001 | [Interval](../data_structure/001_interval.md) | Storage type for converted intervals |
| feature/002 | [Non-Iterable Intervals](002_non_iterable_intervals.md) | Accepting unbounded intervals via `NonIterableInterval` |
