# Feature: Compile-Time Check Toggle

### Scope

- **Purpose**: Allow callers to opt into fast development builds by bypassing compile-time SQL validation without changing query call sites.
- **Responsibility**: Documents the `sqlx_compiletime_checks` feature flag, its dispatch effect, and the intended dev/production workflow.
- **In Scope**: Feature flag semantics, dispatch modes, intended consumer workflow, naming rationale.
- **Out of Scope**: SQL validation internals (-> `sqlx` crate), database connection and pooling setup.

### Design

SQLx compile-time query checking validates SQL syntax and schema against a live database at build time. This is accurate but costly: it requires `DATABASE_URL` and a running database in every compilation environment, including local dev and CI.

`sqlx_query` solves this with a feature-controlled dispatch toggle. By default, the macros expand to compile-time checked sqlx macros — full SQL validation, database required at build. When the consumer enables `sqlx_compiletime_checks`, the macros switch to runtime sqlx functions — no database at build time, fast iteration.

**Naming note:** The feature name `sqlx_compiletime_checks` is intentionally inverted from its activation effect. Enabling the feature **disables** compile-time checks by switching to the runtime path. The name describes what the feature controls, not what it activates. This is a known naming asymmetry documented here to prevent misreading.

**Dispatch table:**

| `sqlx_compiletime_checks` | Expansion target | DB at build | SQL errors surface |
|---|---|---|---|
| Not enabled (default) | Compile-time sqlx macros | Required | Compile time |
| Enabled | Runtime sqlx functions | Not required | Runtime |

**Intended workflow:**
- Development: enable `sqlx_compiletime_checks` in consuming crate features for fast builds, no database needed
- CI / pre-release: disable `sqlx_compiletime_checks` with `DATABASE_URL` set for full SQL validation
- Call sites: identical in both modes, zero code changes required when switching

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | `#[cfg(feature = "sqlx_compiletime_checks")]` dispatch in macro arms |
| config | `Cargo.toml` | Feature declaration and default configuration |
| test | `tests/feature_flag_test.rs` | Verify feature flag detection and mode selection |
| doc | `docs/api/001_query_macros.md` | Query macro API implementing this toggle |
| doc | `docs/invariant/001_query_dispatch_invariant.md` | Invariant governing dispatch correctness |
| doc | `docs/invariant/002_consumer_integration_contract.md` | Consumer dependency and feature wiring requirements |

### Sources

| File | Notes |
|------|-------|
| `spec.md` (deleted) | Original combined spec; recoverable from `git show c13cf485~1:module/experimental/sqlx_query/spec.md`; Overview, Scope, Design Rationale, Usage Patterns sections migrated here |
