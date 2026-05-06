# Feature: Non-Iterable Intervals

### Scope

- **Purpose**: Enable querying bounds of unbounded intervals — open-ended, fully-open, and upper-bounded ranges — without attempting iteration.
- **Responsibility**: Documents the non-iterable-interval trait usage for unbounded ranges — what methods are available and how closed-form conversion behaves at unbounded ends.
- **In Scope**: Non-iterable-interval trait usage with unbounded types, left/right bound semantics, closed-form conversion for unbounded ends, and design rationale.
- **Out of Scope**: Bounded interval iteration (→ `feature/001`); bound extension internals (→ `api/002`); iteration step behavior (→ `data_structure/002`).

### Abstract

Some standard range types are inherently unbounded on one or both ends. The non-iterable-interval trait provides uniform bound-querying access to these types without requiring iteration capability. The closed-form conversion maps an unbounded end to the minimum or maximum sentinel value for the endpoint type, respectively.

### Design

The non-iterable-interval trait is implemented for all standard range types. Unbounded ranges return an unbounded variant from the left or right bound methods. The closed-form methods fall back to the minimum or maximum sentinel value when a bound is unbounded.

#### Usage Example

Declare the function parameter with the non-iterable-interval bound. The function can then query the left and right bounds of any range — bounded or unbounded — using the bound-query methods. Passing an open-ended range returns an unbounded value for the right bound; passing a fully-open range returns unbounded for both.

#### Closed Conversion for Unbounded Ends

When the closed-form left or right endpoint method is called on a range with an unbounded end, the result is the minimum or maximum sentinel value for the endpoint type. This is a bounded approximation, not a true infinity.

### Why Unbounded Cannot Be Iterable

Unbounded intervals implement the non-iterable-interval trait but not the iterable-interval trait. The compiler enforces this at the trait level: the iterable-interval trait requires the iteration protocol, which fully-open, upper-bounded, and upper-inclusive-bounded range types do not implement. This prevents infinite loops from appearing as ordinary iteration.

### Constraints

- Closed-form values for unbounded ends are sentinel approximations — minimum or maximum for the endpoint type — not true infinities.
- The closed-interval length method on a partially or fully unbounded interval produces a value using these sentinels — callers should guard against this.
- Converting an unbounded range to canonical form produces an interval with an unbounded bound — which is non-iterable-interval only.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [api/001_interval_traits.md](../api/001_interval_traits.md) | Trait split between iterable and non-iterable |
| doc | [api/002_conversion_traits.md](../api/002_conversion_traits.md) | Bound extension closed-form conversion and unbounded sentinel values |
| doc | [pattern/001_two_trait_hierarchy.md](../pattern/001_two_trait_hierarchy.md) | Design rationale for the split |
| doc | [feature/001_generic_interval_parameter.md](001_generic_interval_parameter.md) | Bounded interval iteration |
| doc | [invariant/003_no_set_operations.md](../invariant/003_no_set_operations.md) | Set operations absent from the interval interface |
