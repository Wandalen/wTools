# API: is_slice

### Scope

- **Purpose**: Provide a call-site slice detection check — determine whether the type of a given expression is a reference to an unsized contiguous sequence, returning a bool without consuming the expression.
- **Responsibility**: Documents the is_slice macro — its accepted input form, return contract, discrimination rules, error surface, and compatibility.
- **In Scope**: Single-expression slice reference detection returning bool.
- **Out of Scope**: String slice detection, mutable vs shared distinction, slice length or content inspection.

### Abstract

A macro that accepts any single expression, determines at the point of call whether that expression is a reference to an unsized contiguous sequence, and returns a bool. The expression is captured by a temporary non-consuming reference so the caller retains full ownership. The result is determined at compile time via autoref specialization — a reference to an unsized slice produces true; all other types produce false.

### Operations

**Detect whether expression is a slice reference**: accepts any well-typed expression; wraps it in a non-consuming temporary reference; resolves method dispatch using phantom type autoref specialization; returns true if and only if the expression's type is a reference to an unsized contiguous sequence.

Discrimination rules:
- A variable of slice reference type — true
- A slice literal followed by a full-range index (producing an unsized reference) — true
- A reference to a sized array — false (thin pointer, different phantom type)
- A Vec or other owning collection — false
- A primitive, Box, or any non-reference type — false
- A string slice — false (out of scope by design)

When a variable of slice reference type is passed, the extra indirection from the temporary reference is handled correctly — the result reflects the type of the binding, not the reference wrapper.

### Error Handling

No runtime errors. Any invalid expression produces a compile-time error. The macro cannot panic. The bool return always gives a definitive answer.

### Compatibility Guarantees

No feature flags required. No standard library required — only core (see invariant/001_no_std.md). The expression is never consumed (see invariant/002_value_not_consumed.md). Available whenever the `enabled` feature is active.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Macro definition — is_slice autoref specialization |
| test | `tests/inc/slice_tests.rs` | 13 test cases across all discrimination categories |
| doc | `docs/feature/001_slice_detection.md` | End-to-end feature context |
| doc | `docs/invariant/001_no_std.md` | No standard library requirement |
| doc | `docs/invariant/002_value_not_consumed.md` | Non-consuming evaluation guarantee |
