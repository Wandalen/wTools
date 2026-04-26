# Pattern: Canonical Interval Type

### Scope

- **Purpose**: Document why `Interval` exists as a concrete type that unifies all interval representations into a single storable form.
- **Responsibility**: Problem statement, solution structure, applicability, and consequences of the canonical type approach.
- **In Scope**: The rationale for a concrete canonical type over trait objects, the uniformity benefit, and the conversion tradeoff.
- **Out of Scope**: Trait hierarchy design (→ `pattern/001`); `Interval` struct details (→ `data_structure/001`); `IntoInterval` signatures (→ `api/002`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [data_structure/001_interval.md](../data_structure/001_interval.md) | Interval struct definition and field layout |
| doc | [api/002_conversion_traits.md](../api/002_conversion_traits.md) | IntoInterval trait enabling conversion |
| doc | [pattern/001_two_trait_hierarchy.md](001_two_trait_hierarchy.md) | Complementary pattern governing the trait layer |

### Problem

Generic code working with intervals may need to store an interval in a struct field, return it from a function, or collect multiple intervals into a `Vec`. Trait objects (`Box<dyn NonIterableInterval>`) require heap allocation and dynamic dispatch, defeating the zero-dependency and `no_std` goals. Without a concrete canonical type, every storage point must be monomorphized to a specific range type, losing generality.

### Solution

Provide a single concrete type, `Interval`, that stores any interval as a pair of bound values — one for the left endpoint and one for the right. Both fields are private; all access goes through the trait methods.

All interval types implement `IntoInterval`, converting to `Interval` at the point of storage. Once stored as `Interval`, the value implements both `NonIterableInterval` and `IterableInterval` (for bounded cases), so all interval operations remain available without any further conversion.

**Converting to canonical form:**

When storing an interval in a struct field, declare the field using `Interval` and convert at the call site using `into_interval`. Any interval type — half-open ranges, closed ranges, tuples, and arrays — converts to `Interval` via this method, so the struct field accepts all of them uniformly.

### Applicability

Apply this pattern when a library must accept multiple concrete types implementing a trait and needs a single storage representation that:
- Avoids heap allocation (`Box<dyn Trait>`).
- Works in `no_std` environments.
- Retains access to all trait methods after conversion.
- Is `Copy` and `Clone` for ergonomic passing.

### Consequences

**Benefits:**
- `Interval` is `Copy` — passes by value without cloning overhead.
- A single iterator implementation (`IntervalIterator`) covers all interval types after conversion.
- Struct fields, return types, and collections use a single concrete type.

**Tradeoff:**
- Conversion from source type to `Interval` incurs a small construction overhead — two bound values are constructed rather than borrowing the original range.
- Callers that work exclusively with a single concrete range type pay an unnecessary conversion cost.
