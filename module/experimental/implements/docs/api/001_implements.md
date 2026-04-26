# API: implements

### Scope

- **Purpose**: Provide a call-site trait implementation check — determine whether the type of a given expression satisfies one or more trait bounds, returning a bool without consuming the expression.
- **Responsibility**: Documents the implements macro — its accepted input form, return contract, error surface, and compatibility.
- **In Scope**: Single-expression trait bound evaluation returning bool.
- **Out of Scope**: The instance_of alias (→ api/002_instance_of.md), compile-time branching on trait bounds, associated type constraints.

### Abstract

A macro that accepts one expression and one or more trait bound names, evaluates whether the expression's type satisfies those bounds, and returns a bool. The expression is captured by a temporary non-consuming reference so the caller retains full ownership after the call. The result is determined entirely at compile time; the returned bool is a compile-time constant emitted as a runtime expression. No allocation, no side effects.

### Operations

**Check single expression against trait bounds**: accepts any well-typed expression followed by `=>` and one or more trait bound names; creates a temporary phantom reference to the expression; resolves method dispatch via autoref specialization; returns true if the type satisfies the given bounds, false otherwise. The expression text is evaluated once to obtain the value; the value itself is referenced but not moved.

Compound bounds (multiple traits in conjunction) are supported using the same syntax as Rust trait bound clauses. Trait paths with path separators are accepted.

### Error Handling

No runtime errors. Any invalid expression or malformed trait bound produces a compile-time error. The macro cannot panic. The bool return value always represents a definitive answer — there is no partial or indeterminate state.

### Compatibility Guarantees

No feature flags required. No external runtime dependencies (see invariant/002_zero_runtime_dependencies.md). The expression is never consumed (see invariant/001_value_not_consumed.md). The macro is available whenever the `enabled` feature is active.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Public macro export and prelude re-export |
| source | `src/implements_impl.rs` | Internal _implements! macro — autoref mechanism |
| test | `tests/inc/test_cases.rs` | Full coverage: primitives, Box, closures, Fn/FnMut/FnOnce, path syntax |
| doc | `docs/feature/001_trait_implementation_check.md` | End-to-end feature context |
| doc | `docs/api/002_instance_of.md` | Alias macro with identical semantics |
| doc | `docs/invariant/001_value_not_consumed.md` | Non-consuming evaluation guarantee |
