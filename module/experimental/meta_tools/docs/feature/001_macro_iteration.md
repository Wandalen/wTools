# Feature: Macro Iteration

### Scope

- **Purpose**: Allow a caller to apply a single macro to each element in a comma-delimited list without repeating the macro name at every call site.
- **Responsibility**: Document the scope, constraints, and cross-references for the `for_each!` macro re-exported by `meta_tools`.
- **In Scope**: `for_each!` in function-style and map-style invocations, compile-time expansion, feature flag `meta_for_each`.
- **Out of Scope**: Runtime iteration, custom iteration order, `for_each` implementation internals (see the `for_each` crate).

### Design

`for_each!` accepts a macro name as its first argument followed by elements separated by commas. It expands to one invocation of the named macro per element, in list order. Two syntactic forms are supported: function-style (first argument is the macro name) and map-style (elements mapped to a macro). The macro is compile-time only; no runtime dispatch occurs.

The feature is controlled by the `meta_for_each` flag. Disabling the flag removes the dependency on the `for_each` crate entirely.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| Source | `src/dependency.rs` | `for_each::*` re-export under `meta_for_each` feature |
| Source | `src/exposed.rs` | Exposed namespace re-export |
| Test | `tests/inc/mod.rs` | Cross-crate inclusion of `for_each` test suite |
| Test | `tests/corner_cases_comprehensive.rs` | Edge cases: single element, unicode, mixed literals, nesting |
| Doc | `docs/api/001_macros.md` | `for_each!` macro signature and usage |
