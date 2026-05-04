# Invariant: Query Dispatch Mode Consistency

### Scope

- **Purpose**: Ensure `query!` and `query_as!` always delegate to the correct sqlx variant for the active feature configuration.
- **Responsibility**: Documents the behavioral contract between feature flag state and macro dispatch target.
- **In Scope**: Dispatch mode selection rule, both modes' expected delegation targets, mutual exclusivity guarantee.
- **Out of Scope**: Runtime error handling (-> `sqlx` crate), schema validation mechanics (-> `sqlx` crate).

### Invariant Statement

For every invocation of `query!` or `query_as!`: when `sqlx_compiletime_checks` is **not** enabled, the macro MUST expand to the compile-time sqlx macro form; when `sqlx_compiletime_checks` IS enabled, the macro MUST expand to the runtime sqlx function form. No invocation may silently mix modes within a single build. In the `sqlx_compiletime_checks`-enabled mode, no compile-time SQL validation occurs — SQL errors are deferred to runtime.

### Enforcement Mechanism

Enforced by mutually exclusive `#[cfg(feature = "sqlx_compiletime_checks")]` / `#[cfg(not(feature = "sqlx_compiletime_checks"))]` branches within each macro expansion arm in `src/lib.rs`. The compiler selects exactly one branch per feature configuration — the two dispatch paths cannot both be active in the same build.

### Violation Consequences

A violation (both branches active, neither active, or wrong branch selected for the feature state) would silently route SQL queries to the wrong execution path: either bypassing expected compile-time SQL validation in production builds, or requiring a database at build time when the fast-build runtime path was expected. In the runtime path, SQL syntax errors and schema mismatches that would have been caught at build time are only detected at runtime, potentially in production.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Mutually exclusive `#[cfg]` dispatch in macro arms |
| config | `Cargo.toml` | `sqlx_compiletime_checks` feature declaration governing dispatch |
| test | `tests/feature_flag_test.rs` | Verify feature flag detection and mode selection |
| doc | `docs/api/001_query_macros.md` | API governed by this invariant |
| doc | `docs/feature/001_compiletime_check_toggle.md` | Feature that activates the dispatch toggle |

### Sources

| File | Notes |
|------|-------|
| `spec.md` (deleted) | Macro Expansion diagram and Known Limitations migrated here; recoverable from `git show c13cf485~1:module/experimental/sqlx_query/spec.md` |
