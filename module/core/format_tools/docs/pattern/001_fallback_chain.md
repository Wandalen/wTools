# Pattern: Fallback Chain

### Scope

- **Purpose**: Provide a compile-time mechanism for trying a preferred operation and falling back through alternatives when the preferred one is unavailable.
- **Responsibility**: Documents the fallback chain pattern — the problem it solves, the solution structure, when to apply it, and its trade-offs.
- **In Scope**: Type-based dispatch with ordered fallback levels, compile-time strategy selection, zero-runtime-cost abstraction.
- **Out of Scope**: Runtime error handling, dynamic dispatch (those use different patterns).

### Problem

A caller wants to format a value using the most informative formatter available (display), but not all types implement display. A fallback to a less informative but universally available formatter (debug) is needed. Without a pattern, callers must write conditional code at every call site, with the condition duplicated across the codebase.

### Solution

Use zero-size marker types as strategy tokens. The primary marker directs a generic function to try the preferred approach. If the type does not satisfy the required interface, the compiler selects a blanket implementation that delegates to the next marker in the chain — the first fallback. The same mechanism applies recursively for a second fallback.

The chain is fully resolved at compile time through trait resolution. No runtime branching, no dynamic dispatch, no overhead. The marker types contain no data and occupy no memory.

In format_tools: wrapper types serve as strategy markers. The fallback conversion macro constructs the appropriate wrapper chain and invokes the generic dispatch function, which resolves to the first applicable strategy.

### Applicability

Apply this pattern when:
- Multiple formatting strategies exist with a clear preference ordering.
- The preferred strategy may not be available for all types.
- Compile-time guarantees are required that at least one strategy applies.
- Runtime overhead must be zero.

### Consequences

**Benefits**: Zero runtime cost; failures caught at compile time; call sites remain clean with no conditional logic; new fallback levels can be added by extending the chain.

**Trade-offs**: Compile error messages for unsatisfied chains can be complex; the chain depth is fixed at compile time (not dynamically adjustable); understanding the dispatch requires familiarity with trait resolution.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/format/to_string_with_fallback.rs` | Core implementation of the fallback chain |
| source | `src/format/wrapper/aref.rs` | Strategy marker implementations |
| test | `tests/inc/to_string_with_fallback_test.rs` | Pattern behavior tests |
| test | `tests/inc/to_string_with_fallback_corner_cases_test.rs` | Edge case coverage |
| doc | `docs/feature/001_fallback_string_conversion.md` | Feature that applies this pattern |
| doc | `docs/api/001_fallback_conversion_api.md` | API built on this pattern |
| doc | `docs/api/004_wrapper_types_api.md` | Strategy markers used in this pattern |
