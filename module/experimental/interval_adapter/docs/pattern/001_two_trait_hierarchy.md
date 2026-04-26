# Pattern: Two-Trait Hierarchy

### Scope

- **Purpose**: Document why `interval_adapter` splits the interval interface into two traits — `NonIterableInterval` for all intervals and `IterableInterval` for bounded ones.
- **Responsibility**: Problem statement, solution structure, applicability criteria, and consequences of the split.
- **In Scope**: Trait hierarchy design, the type-safety argument, and the tradeoff accepted.
- **Out of Scope**: Canonical type design (→ `pattern/002`); trait signatures (→ `api/001`); unbounded interval feature (→ `feature/002`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [api/001_interval_traits.md](../api/001_interval_traits.md) | Trait signatures for both layers |
| doc | [feature/002_non_iterable_intervals.md](../feature/002_non_iterable_intervals.md) | Feature built on NonIterableInterval |
| doc | [invariant/001_integer_endpoints_only.md](../invariant/001_integer_endpoints_only.md) | Additional constraint enforced at the trait level |
| doc | [pattern/002_canonical_interval_type.md](002_canonical_interval_type.md) | Complementary pattern governing the concrete type layer |

### Problem

Rust's standard range types include both bounded (`Range`, `RangeInclusive`) and unbounded (`RangeFull`, `RangeFrom`, `RangeTo`) variants. A generic function that accepts any range type must either:

1. Accept only bounded ranges (losing the ability to describe unbounded intervals), or
2. Accept all ranges — including unbounded ones — and risk creating infinite iteration loops.

A single trait covering all range types cannot expose iteration without silently allowing infinite loops over `RangeFull` or `RangeFrom`.

### Solution

Split the interface into two traits in a strict hierarchy: a base trait covers all interval types including unbounded ones and exposes only bound-query operations; an extended trait adds an iteration requirement and is implemented only by types that also support the iteration protocol — bounded ranges only.

- `NonIterableInterval` is implemented for all range types, including unbounded ones.
- `IterableInterval` is implemented only for types that also support the iteration protocol — bounded ranges only.
- Unbounded types (`RangeFull`, `RangeFrom`, `RangeTo`, `RangeToInclusive`) never implement `IterableInterval`.

The compiler enforces the split: a function taking `impl IterableInterval` cannot be called with a `RangeFull` argument. The error appears at the call site, not inside the function body.

### Applicability

Apply this pattern when designing an abstraction over a heterogeneous collection of types where:
- Some members can participate in an operation (iteration) and some cannot.
- Using a single trait with a blanket implementation would silently allow unsafe or infinite behavior.
- Compile-time type safety is preferred over runtime guards.

### Consequences

**Benefits:**
- Compiler prevents passing unbounded intervals to functions that iterate — no infinite loops at runtime.
- Functions that only query bounds can accept both bounded and unbounded intervals uniformly.
- The hierarchy is extensible: a third trait (e.g., `BidirectionalInterval`) could extend `IterableInterval`.

**Tradeoff:**
- Callers must choose between `NonIterableInterval` and `IterableInterval` when writing generic functions.
- The split adds conceptual complexity for users unfamiliar with the rationale.
