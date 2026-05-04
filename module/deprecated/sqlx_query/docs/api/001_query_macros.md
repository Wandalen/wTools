# API: Query Macros

### Scope

- **Purpose**: Provide SQL query execution macros that transparently toggle between compile-time and runtime dispatch.
- **Responsibility**: Documents the `query!` and `query_as!` macro interfaces, invocation forms, and dispatch behavior per feature flag state.
- **In Scope**: Macro argument forms, bind parameter passing, feature-flag-controlled dispatch target, error surfaces.
- **Out of Scope**: Database connection management and result type definitions (-> `sqlx` crate).

### Abstract

Two macros wrapping sqlx query execution: `query!` for untyped results and `query_as!` for typed results. Both dispatch transparently to either compile-time checked sqlx macros or runtime sqlx functions depending on the `sqlx_compiletime_checks` feature flag. Call sites are identical in both modes; no code changes are required when switching.

The crate declares no explicit dependency on sqlx — the consuming crate must provide it. See `docs/invariant/002_consumer_integration_contract.md`.

### Operations

`query!( sql )` — Execute a SQL query without bind parameters, returning untyped rows.

`query!( sql, bind... )` — Execute a SQL query with one or more positional bind parameters.

`query_as!( Type, sql )` — Execute a SQL query, deserializing each row into `Type`.

`query_as!( Type, sql, bind... )` — Typed query with bind parameters.

Dispatch target per operation:
- `sqlx_compiletime_checks` **not** enabled (default): `sqlx::query!` / `sqlx::query_as!` — compile-time SQL validation, database required at build time.
- `sqlx_compiletime_checks` **enabled**: `sqlx::query()` / `sqlx::query_as()` — runtime execution, no database required at build time.

### Error Handling

In default mode, SQL syntax and schema errors surface as build errors during compilation. In `sqlx_compiletime_checks` mode, SQL errors surface as runtime error values from executor methods chained after macro expansion — errors are only caught at runtime.

### Compatibility Guarantees

Macro argument forms mirror the sqlx `query!` / `query_as!` interface exactly. Enabling or disabling `sqlx_compiletime_checks` never requires changes at call sites. Callers are insulated from the underlying dispatch mechanism.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Macro definitions and `#[cfg]` dispatch logic |
| config | `Cargo.toml` | `sqlx_compiletime_checks` feature declaration |
| test | `tests/macro_expansion_test.rs` | Verify macro syntax expansion in both modes |
| doc | `docs/feature/001_compiletime_check_toggle.md` | Feature describing the toggle mechanism and workflow |
| doc | `docs/invariant/001_query_dispatch_invariant.md` | Invariant governing dispatch correctness |
| doc | `docs/invariant/002_consumer_integration_contract.md` | Invariant governing consumer dependency requirements |

### Sources

| File | Notes |
|------|-------|
| `spec.md` (deleted) | Original combined spec; recoverable from `git show c13cf485~1:module/experimental/sqlx_query/spec.md`; Public API and Feature Flags sections migrated here |
