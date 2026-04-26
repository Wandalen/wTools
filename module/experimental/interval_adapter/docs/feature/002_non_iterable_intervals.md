# Feature: Non-Iterable Intervals

### Scope

- **Purpose**: Enable querying bounds of unbounded intervals (`RangeFrom`, `RangeFull`, `RangeTo`) without attempting iteration.
- **Responsibility**: Documents `NonIterableInterval` usage for unbounded ranges — what methods are available and how closed-form conversion behaves at unbounded ends.
- **In Scope**: `NonIterableInterval` with unbounded types, `left`/`right` semantics, `closed_left`/`closed_right` semantics for `Unbounded`, and design rationale.
- **Out of Scope**: Bounded interval iteration (→ `feature/001`); `BoundExt` internals (→ `api/002`); iteration step behavior (→ `data_structure/002`).

### Abstract

Rust's `RangeFrom` (`0..`), `RangeFull` (`..`), and `RangeTo` (`..10`) types are inherently unbounded on one or both ends. `NonIterableInterval` provides uniform bound-querying access to these types without requiring iteration capability. The closed-form conversion (`closed_left`, `closed_right`) maps `Unbounded` to `isize::MIN` and `isize::MAX` respectively.

### Design

`NonIterableInterval` is implemented for all standard range types. Unbounded ranges return `Bound::Unbounded` from `left()` or `right()`. The closed-form methods fall back to `isize::MIN`/`isize::MAX` for `Unbounded`.

#### Usage Example

```rust
use interval_adapter::{ NonIterableInterval, Bound };

fn describe( interval : impl NonIterableInterval ) {
  println!( "Left:  {:?}", interval.left() );
  println!( "Right: {:?}", interval.right() );
}

describe( 0.. );    // Left: Included(0),  Right: Unbounded
describe( .. );     // Left: Unbounded,    Right: Unbounded
describe( ..10 );   // Left: Unbounded,    Right: Excluded(10)
describe( ..=9 );   // Left: Unbounded,    Right: Included(9)
```

#### Closed Conversion for Unbounded Ends

```rust
use interval_adapter::NonIterableInterval;

let interval = 0..;              // RangeFrom — right is Unbounded
let left  = interval.closed_left();    // 0
let right = interval.closed_right();   // isize::MAX (fallback)
```

### Why Unbounded Cannot Be Iterable

Unbounded intervals implement `NonIterableInterval` but not `IterableInterval`. The compiler enforces this at the trait level: `IterableInterval` requires `IntoIterator`, which `RangeFull`, `RangeTo`, and `RangeToInclusive` do not implement. This prevents infinite loops from appearing as ordinary `for` loops.

### Constraints

- Closed-form values for `Unbounded` are sentinel approximations (`isize::MIN`/`isize::MAX`), not true infinities.
- `closed_len()` on a partially or fully unbounded interval produces a value using these sentinels — callers should guard against this.
- `canonical()` on an unbounded range produces an `Interval<T>` with `Bound::Unbounded` — which is `NonIterableInterval` only.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [api/001_interval_traits.md](../api/001_interval_traits.md) | Trait split between iterable and non-iterable |
| doc | [api/002_conversion_traits.md](../api/002_conversion_traits.md) | BoundExt closed-form conversion and Unbounded sentinel values |
| doc | [pattern/001_two_trait_hierarchy.md](../pattern/001_two_trait_hierarchy.md) | Design rationale for the split |
| doc | [feature/001_generic_interval_parameter.md](001_generic_interval_parameter.md) | Bounded interval iteration |
| doc | [invariant/003_no_set_operations.md](../invariant/003_no_set_operations.md) | Set operations absent from the interval interface |
