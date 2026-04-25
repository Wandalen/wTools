# Feature: Identifier Concatenation

### Scope

- **Purpose**: Enable compile-time construction of identifiers by concatenating tokens, allowing callers to generate method names, type names, or any identifier from parts.
- **Responsibility**: Document the scope and cross-references for the `meta_idents_concat!` macro provided by `meta_tools`.
- **In Scope**: `meta_idents_concat!` macro, `[< ... >]` bracket syntax for identifier construction, the `meta_idents_concat` feature flag.
- **Out of Scope**: `paste` crate internals, macro hygiene implementation details.

### Design

`meta_idents_concat!` is `paste::paste` re-exported under a workspace-consistent name. The `[< tokens >]` bracket syntax inside the macro body assembles a new identifier from adjacent token fragments. The rename provides naming consistency: all meta-programming macros in the workspace use descriptive names rather than external crate names.

The feature is controlled by `meta_idents_concat`. Disabling it removes the `paste` crate dependency entirely.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| Source | `src/dependency.rs` | `paste::paste as meta_idents_concat` re-export |
| Source | `src/exposed.rs` | Exposed namespace re-export |
| Test | `tests/inc/indents_concat_test.rs` | Basic identifier construction test |
| Test | `tests/corner_cases_comprehensive.rs` | Edge cases: paste basic, multiple macros in same scope |
| Doc | `docs/api/001_macros.md` | `meta_idents_concat!` macro signature and syntax |
