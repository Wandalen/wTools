# Data Structure: IntervalIterator

### Scope

- **Purpose**: Define the iterator type that yields consecutive integer values across a closed interval.
- **Responsibility**: Single source of truth for the interval iterator — field layout, iteration semantics, and the closed-inclusive contract.
- **In Scope**: Struct fields, iteration semantics, and termination contract.
- **Out of Scope**: The canonical interval type that produces it (→ `data_structure/001`); trait signatures (→ `api/001`).

### Abstract

The interval iterator is produced by calling the iteration method on the canonical interval type. It yields consecutive values starting at the closed left endpoint up to and including the closed right endpoint, stepping by one. Reverse iteration and custom step sizes are not supported.

### Structure

| Field | Notes |
|-------|-------|
| `current` | Next value to yield; starts at the closed left endpoint of the source interval |
| `right` | Inclusive upper bound; iteration stops after this value is yielded |

Both fields are private. The iterator is produced only via the canonical interval type's iteration method.

### Operations

- Iteration is **closed-inclusive** on both endpoints: the first value yielded is the closed left endpoint, the last is the closed right endpoint.
- Each step increments by exactly one.
- Returns no more values once the current value exceeds the right bound — for reversed intervals (left greater than right), the iterator immediately produces nothing.

**Example output:**

An inclusive-bounded interval from 0 to 3 yields values 0, 1, 2, 3. A half-open interval from 0 up-to-but-not-including 4 normalizes the excluded right endpoint to 3 and yields the same sequence.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [data_structure/001_interval.md](001_interval.md) | Produces this iterator via the iteration method |
| doc | [api/001_interval_traits.md](../api/001_interval_traits.md) | Iterable-interval constraint requiring iteration |
| doc | [feature/003_no_std_support.md](../feature/003_no_std_support.md) | Stack-allocated iterator in no-standard-library context |
| doc | [invariant/001_integer_endpoints_only.md](../invariant/001_integer_endpoints_only.md) | Endpoint constraint trait constrains element type |
| doc | [invariant/002_no_validation.md](../invariant/002_no_validation.md) | Immediate empty result for reversed intervals |
