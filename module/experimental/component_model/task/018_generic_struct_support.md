# Task 018 — Generic Struct Support

## Problem

The five derive macros (`ComponentFrom`, `Assign`, `ComponentsAssign`, `FromComponents`, `ComponentModel`) do not support generic structs. Attempting to derive any of these traits on a struct with type parameters produces a compiler error because the generated `impl` block lacks the required generic bounds.

The issue was tracked with a `qqq:` marker in `tests/inc/components_tests/composite.rs` at the `Options1` struct definition:

```
// qqq: make these traits working for generic struct, use `split_for_impl`
```

## Location

- Macro expansion logic: `module/experimental/component_model_meta/src/`
- Relevant test struct: `tests/inc/components_tests/composite.rs` — `Options1`

## Acceptance Criteria

1. All five derive macros produce correct `impl<T>` blocks for structs with one or more type parameters.
2. `split_for_impl` (from `syn`) is used to split generics into `<T>`, `<T: Bound>`, and `where T: Bound` parts.
3. A new test in `tests/inc/components_tests/composite.rs` (or a new file) verifies derive on a generic struct `struct GenericOptions<T> { field: T }`.
4. Existing non-generic tests continue to pass (no regression).

## Dependencies

- None (pure proc-macro work in `component_model_meta`).

## Status

📋 Planned
