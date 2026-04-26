# Data Structure: IntervalIterator

### Scope

- **Purpose**: Define the iterator type that yields consecutive integer values across a closed interval.
- **Responsibility**: Single source of truth for `IntervalIterator` — field layout, iteration semantics, and the closed-inclusive contract.
- **In Scope**: Struct fields, iteration semantics, and termination contract.
- **Out of Scope**: The `Interval` type that produces it (→ `data_structure/001`); trait signatures (→ `api/001`).

### Abstract

`IntervalIterator` is produced by calling `into_iter` on an `Interval`. It yields consecutive values starting at the closed left endpoint up to and including the closed right endpoint, stepping by one. Reverse iteration and custom step sizes are not supported.

### Structure

| Field | Notes |
|-------|-------|
| `current` | Next value to yield; starts at `closed_left` of the source interval |
| `right` | Inclusive upper bound; iteration stops after this value is yielded |

Both fields are private. The iterator is produced only via `Interval::into_iter`.

### Operations

- Iteration is **closed-inclusive** on both endpoints: the first value yielded is `closed_left`, the last is `closed_right`.
- Each step increments by exactly one.
- Returns no more values once `current > right` — for reversed intervals (`left > right`), the iterator immediately produces nothing.

**Example output:**

```
Interval from Included(0) to Included(3):
  yields: 0, 1, 2, 3

Interval from Included(0) to Excluded(4):
  yields: 0, 1, 2, 3  (Excluded(4) normalizes to closed right = 3)
```

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [data_structure/001_interval.md](001_interval.md) | Produces this iterator via into_iter |
| doc | [api/001_interval_traits.md](../api/001_interval_traits.md) | IterableInterval constraint requiring iteration |
| doc | [feature/003_no_std_support.md](../feature/003_no_std_support.md) | Stack-allocated iterator in no_std context |
| doc | [invariant/001_integer_endpoints_only.md](../invariant/001_integer_endpoints_only.md) | EndPointTrait constrains element type |
| doc | [invariant/002_no_validation.md](../invariant/002_no_validation.md) | Immediate empty result for reversed intervals |
