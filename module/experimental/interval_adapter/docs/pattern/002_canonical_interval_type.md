# Pattern: Canonical Interval Type

### Scope

- **Purpose**: Document why `Interval<T>` exists as a concrete type that unifies all interval representations into a single storable form.
- **Responsibility**: Problem statement, solution structure, applicability, and consequences of the canonical type approach.
- **In Scope**: The rationale for a concrete canonical type over trait objects, the uniformity benefit, and the conversion tradeoff.
- **Out of Scope**: Trait hierarchy design (→ `pattern/001`); `Interval<T>` struct details (→ `data_structure/001`); `IntoInterval` signatures (→ `api/002`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| data_structure/001 | [Interval](../data_structure/001_interval.md) | `Interval<T>` struct definition and field layout |
| api/002 | [Conversion Traits](../api/002_conversion_traits.md) | `IntoInterval` trait enabling conversion |
| pattern/001 | [Two-Trait Hierarchy](001_two_trait_hierarchy.md) | Complementary pattern governing the trait layer |

### Problem

Generic code working with intervals may need to store an interval in a struct field, return it from a function, or collect multiple intervals into a `Vec`. Trait objects (`Box<dyn NonIterableInterval>`) require heap allocation and dynamic dispatch, defeating the zero-dependency and `no_std` goals. Without a concrete canonical type, every storage point must be monomorphized to a specific range type, losing generality.

### Solution

Provide a single concrete type, `Interval<T>`, that stores any interval as a `(Bound<T>, Bound<T>)` pair:

```rust
pub struct Interval< T = isize >
{
  _left  : Bound< T >,
  _right : Bound< T >,
}
```

All interval types implement `IntoInterval<T>`, converting to `Interval<T>` at the point of storage. Once stored as `Interval<T>`, the value implements both `NonIterableInterval<T>` and `IterableInterval<T>` (for bounded cases), so all interval operations remain available without any further conversion.

**Converting to canonical form:**

```rust
use interval_adapter::{ IntoInterval, Interval };

// At struct field:
struct Window { range : Interval< i32 > }

// At call site — any interval type works:
let w1 = Window { range : ( 0..10 ).into_interval() };
let w2 = Window { range : ( 0..=9 ).into_interval() };
let w3 = Window { range : ( 0, 9 ).into_interval() };
```

### Applicability

Apply this pattern when a library must accept multiple concrete types implementing a trait and needs a single storage representation that:
- Avoids heap allocation (`Box<dyn Trait>`).
- Works in `no_std` environments.
- Retains access to all trait methods after conversion.
- Is `Copy` and `Clone` for ergonomic passing.

### Consequences

**Benefits:**
- `Interval<T>` is `Copy` — passes by value without cloning overhead.
- Single iterator implementation (`IntervalIterator<T>`) covers all interval types after conversion.
- Struct fields, return types, and collections use a single concrete type.

**Tradeoff:**
- Conversion from source type to `Interval<T>` incurs a small construction overhead — two `Bound<T>` values are constructed rather than borrowing the original range.
- Callers that work exclusively with a single range type (e.g., always `Range<i32>`) pay an unnecessary conversion cost.
