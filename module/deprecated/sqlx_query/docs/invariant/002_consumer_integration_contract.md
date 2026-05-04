# Invariant: Consumer Integration Contract

### Scope

- **Purpose**: Ensure consumers correctly wire `sqlx_query` into their own project dependency and feature declarations.
- **Responsibility**: Documents the two external requirements a consumer must satisfy for the crate to function correctly.
- **In Scope**: Consumer sqlx dependency requirement, consumer feature flag wiring, sqlx version coupling.
- **Out of Scope**: Database connection setup (-> `sqlx` crate), query execution mechanics (-> `api/001_query_macros.md`).

### Invariant Statement

Two conditions MUST both hold for `sqlx_query` to function in a consuming crate:

1. **sqlx dependency**: The consuming crate MUST declare `sqlx` as a direct dependency in its own `Cargo.toml`. `sqlx_query` macros reference `::sqlx::*` as an external crate path — `sqlx_query` itself declares no `sqlx` dependency and provides no re-export.

2. **Feature wiring**: The `sqlx_compiletime_checks` feature MUST be declared in the consuming crate `Cargo.toml` (either as `sqlx_query = { features = ["sqlx_compiletime_checks"] }` or via a local feature re-export) to activate runtime mode. Enabling compile-time mode requires no feature — it is the default.

### Enforcement Mechanism

Neither condition is enforced by the compiler automatically. Condition 1 failure produces an "unresolved import" error at macro expansion time on `::sqlx::*` references. Condition 2 failure is silent — the feature simply remains disabled (compile-time mode stays active), which may or may not be the consumer intent.

### Violation Consequences

Condition 1 violation: macro expansion fails to compile with an unresolved path error on `::sqlx::*` references.

Condition 2 violation (silent): consumer intends fast dev builds but `sqlx_compiletime_checks` is not wired — compile-time mode stays active — build requires database at compile time — unexpected build failures in environments without database connectivity.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | `::sqlx::*` external path references that require consumer sqlx dependency |
| config | `Cargo.toml` | Feature declarations; absence of explicit sqlx dependency |
| doc | `docs/api/001_query_macros.md` | API whose call sites require both conditions to be met |
| doc | `docs/feature/001_compiletime_check_toggle.md` | Feature that requires condition 2 to activate |

### Sources

| File | Notes |
|------|-------|
| `spec.md` (deleted) | Consumer Requirements and Known Limitations migrated here; recoverable from `git show c13cf485~1:module/experimental/sqlx_query/spec.md` |
