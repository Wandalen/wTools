# Data Structure: IntervalIterator

### Scope

- **Purpose**: Define the iterator type that yields consecutive integer values across a closed interval.
- **Responsibility**: Single source of truth for `IntervalIterator<T>` — field layout, iteration semantics, and the closed-inclusive contract.
- **In Scope**: Struct fields, iteration semantics, and termination contract.
- **Out of Scope**: The `Interval<T>` type that produces it (→ `data_structure/001`); trait signatures (→ `api/001`).

### Abstract

`IntervalIterator<T>` is produced by calling `into_iter` on an `Interval<T>`. It yields consecutive values starting at the closed left endpoint up to and including the closed right endpoint, stepping by one. Reverse iteration and custom step sizes are not supported.

### Fields

| Field | Notes |
|-------|-------|
| `current` | Next value to yield; starts at `closed_left` of the source interval |
| `right` | Inclusive upper bound; iteration stops after this value is yielded |

Both fields are private. The iterator is produced only via `Interval<T>::into_iter`.

### Iteration Semantics

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
| data_structure/001 | [Interval](001_interval.md) | Produces this iterator via `into_iter` |
| api/001 | [Interval Traits](../api/001_interval_traits.md) | `IterableInterval` constraint requiring `IntoIterator` |
| invariant/001 | [Integer Endpoints Only](../invariant/001_integer_endpoints_only.md) | `EndPointTrait` constrains element type |
