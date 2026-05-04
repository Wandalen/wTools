# Feature: Trait Implementation Generation

### Scope

- **Purpose**: Generate repetitive trait implementations for tuple types at multiple arities, eliminating boilerplate for patterns such as `Index` and custom multi-element traits.
- **Responsibility**: Document the scope and cross-references for the `impls!` family, `impls_optional!`, test generation macros, and function utility macros re-exported by `meta_tools`.
- **In Scope**: All macros from `impls_index` and `impls_index_meta`, the `impls` alias for `impls3`, and the `meta_impls_index` feature flag.
- **Out of Scope**: `impls_index` and `impls_index_meta` implementation internals, proc-macro compilation details.

### Design

Three levels of impl macros are provided with increasing power and compile complexity: `impls1!` (declarative, simplest), `impls2!` (declarative, intermediate), `impls3!` / `impls!` (procedural, most capable). The `impls` name is an alias for `impls3` for everyday use. `impls_optional!` generates implementations only when the named trait is in scope.

`tests_impls!` and `tests_index!` generate test functions using the same expansion mechanism, integrating with the `test_tools` harness.

`impls_index_meta` (procedural macro crate) is always linked regardless of the `meta_impls_index` flag because `mod_interface!` depends on it internally. Only the declarative `impls_index` crate is feature-optional.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/dependency.rs` | `impls_index::*` and `impls_index_meta::*` re-exports |
| source | `src/exposed.rs` | Exposed namespace re-exports |
| test | `tests/inc/mod.rs` | Cross-crate inclusion of `impls_index` test suite |
| test | `tests/meta_tools_tests.rs` | Main test harness; aggregates `tests_impls!` tests |
| doc | `docs/api/001_macros.md` | `impls!` family and test generation macro signatures |
