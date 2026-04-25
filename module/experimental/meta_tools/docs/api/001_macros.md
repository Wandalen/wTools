# API: Macros

### Scope

- **Purpose**: Expose all compile-time meta-programming macros provided by `meta_tools` as its sole public interface.
- **Responsibility**: Define every re-exported macro, its feature gate, and the access paths available to consumers.
- **In Scope**: All macros accessible via `use meta_tools::*` or explicit paths, feature gate conditions, the `dependency` sub-module, and the `meta` sub-module.
- **Out of Scope**: Proc-macro implementation details, serde integrations, runtime APIs, internal private modules.

### Abstract

`meta_tools` exposes no types or functions — only macros. The public API is controlled by five feature flags, all on by default: `enabled` propagates activation to sub-crate deps (and activates their optional dependencies via the `crate/enabled` feature syntax), while the four capability flags (`meta_for_each`, `meta_impls_index`, `mod_interface`, `meta_idents_concat`) gate the actual macro re-exports. Disabling a capability flag removes its crate from the dependency tree entirely.

All macros are accessible via `use meta_tools::*`. The `meta_tools::dependency` sub-module provides explicit per-dependency namespace access for disambiguation when glob imports conflict.

### Operations

#### Macro Iteration (`meta_for_each` feature, default on)

`for_each!(macro_name, elem1, elem2, ...)` — Invokes `macro_name!` once per element in the comma-delimited list. Elements may be literals, identifiers, or expressions. Both function-style and map-style invocations are supported. Compile-time only.

#### Trait Implementation Generation (`meta_impls_index` feature, default on)

`impls! { impl ... }` — Generates trait implementations for multiple tuple arities. Alias for `impls3!`. Procedural macro; most capable level.

`impls1! { ... }` — Level-1 declarative impl generation; simplest and fastest to compile.

`impls2! { ... }` — Level-2 declarative impl generation; intermediate capabilities.

`impls3! { ... }` — Level-3 procedural impl generation; same capability as `impls!`.

`impls_optional! { impl ... }` — Generates implementations only when the named trait is in scope at the call site.

`tests_impls! { fn name() { ... } }` — Wraps test functions for use with the impls test framework.

`tests_index! { name, ... }` — Registers test names with the impls test index.

`index! { ... }` — Generates `Index` trait implementations for tuple types.

`fn_name!(name)` — Returns the string name of the given function identifier at compile time.

`fn_rename!(old => new)` — Renames a function at compile time.

`fns! { ... }` — Function generation helper.

`fns2! { ... }` — Function generation helper (variant 2).

#### Module Interface Pattern (always available)

`mod_interface! { layer name; ... }` — Generates the four standard namespace layers (`own`, `orphan`, `exposed`, `prelude`) for the enclosing module. Always available regardless of feature flags because `mod_interface_meta` is a mandatory dependency.

#### Identifier Concatenation (`meta_idents_concat` feature, default on)

`meta_idents_concat! { ... [< tokens >] ... }` — Pastes tokens together inside `[< ... >]` brackets to construct a new identifier at compile time. Re-export of `paste::paste` under the workspace naming convention.

#### Dependency Sub-module

`meta_tools::dependency` — Sub-module providing direct namespace access per dependency. Use when a macro name conflicts with another import or when an explicit path is preferred over a glob import.

### Error Handling

All macros produce compile-time errors only — there are no runtime error conditions. Macro invocation errors are reported as compiler diagnostics at the call site. Procedural macros (`impls3!`, `mod_interface!`) produce span-annotated error messages pointing to the offending token.

### Compatibility Guarantees

Semantic versioning. The macro names `for_each!`, `impls!`, `mod_interface!`, and `meta_idents_concat!` are stable within a major version. The leveled aliases `impls1!` / `impls2!` / `impls3!` are stable but `impls!` is preferred for everyday use. The `dependency` sub-module paths are stable. Internal module paths (`meta_tools::meta::*`, `meta_tools::own::*`, etc.) are not part of the public API contract.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| Source | `src/lib.rs` | Feature-gated glob re-exports of all namespaces |
| Source | `src/dependency.rs` | All macro imports and explicit re-exports |
| Source | `src/exposed.rs` | Exposed namespace re-exports |
| Test | `tests/meta_tools_tests.rs` | Main macro test harness |
| Test | `tests/corner_cases_comprehensive.rs` | Edge-case coverage for `for_each!` and `meta_idents_concat!` |
| Test | `tests/inc/indents_concat_test.rs` | Identifier concatenation tests |
| Doc | `docs/feature/001_macro_iteration.md` | Scope of the `for_each!` feature |
| Doc | `docs/feature/002_trait_impl_generation.md` | Scope of the impl generation features |
| Doc | `docs/feature/003_module_interface.md` | Scope of the `mod_interface!` feature |
| Doc | `docs/feature/004_identifier_concatenation.md` | Scope of the `meta_idents_concat!` feature |
