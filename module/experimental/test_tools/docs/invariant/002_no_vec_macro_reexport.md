# Invariant: No vec! Reexport at Crate Root

### Scope

- **Purpose**: Prevents macro ambiguity between the standard library `vec!` and `collection_tools::vec!` when users do `use test_tools::*`.
- **Responsibility**: Documents the invariant that `vec!` must not be re-exported at the crate root level.
- **In Scope**: Collection constructor macro re-export placement in `src/lib.rs`.
- **Out of Scope**: Explicit `collection_tools::vec!` usage by users, which is always permitted.

### Invariant Statement

The `vec!` macro and other collection constructor macros from `collection_tools` must not be re-exported at the crate root level.

### Enforcement Mechanism

Code review of root-level `pub use` statements in `src/lib.rs`. Test `tests/macro_ambiguity_test.rs` documents ambiguity patterns and verifies resolution strategies.

### Violation Consequences

Re-exporting `vec!` at root causes E0659 ambiguity errors: `vec is ambiguous — could refer to macro from prelude and the macro imported here`. This breaks all code using `use test_tools::*` combined with `vec![...]`.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Root-level export declarations and REGRESSION PREVENTION comments |
| test | `tests/macro_ambiguity_test.rs` | Documents ambiguity patterns and resolution strategies |
| task | `task/completed/002_fix_collection_macro_reexports.md` | Task that fixed 7 E0433 errors |
| doc | `docs/feature/001_test_aggregation_facade.md` | Feature owning the macro re-export strategy |
