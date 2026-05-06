# Invariant: Debug Variants Are No-Ops in Release Builds

### Scope

- **Purpose**: Guarantee that a_dbg_* assertion macros introduce no overhead in release builds.
- **Responsibility**: Documents the behavioral contract for debug-only assertion variants — their release-build behavior and enforcement mechanism.
- **In Scope**: a_dbg_true, a_dbg_false, a_dbg_id, a_dbg_not_id behavior under release compilation.
- **Out of Scope**: Always-active assertion macros (a_true, a_false, a_id, a_not_id) — those run unconditionally.

### Invariant Statement

All a_dbg_* macros expand conditionally on cfg(debug_assertions). In release builds (where debug_assertions is not set), the expansion is a no-op — no code is generated and no runtime cost is incurred.

### Enforcement Mechanism

- a_dbg_true and a_dbg_false expand to debug_assert, which is natively conditional on debug_assertions.
- a_dbg_id and a_dbg_not_id use an if cfg!(debug_assertions) block guarding a call to a_id or a_not_id. The compiler eliminates the dead branch in release builds.

### Violation Consequences

Any change that causes a_dbg_* macros to execute in release builds would silently impose pretty_assertions overhead on release binaries and defeat the purpose of having debug-only variants.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/001_runtime_assertions.md](../feature/001_runtime_assertions.md) | Runtime assertions feature subject to this invariant |
| source | [src/diag/rta.rs](../../src/diag/rta.rs) | a_dbg_* macro implementations that enforce this invariant |
| test | [tests/inc/rta_test.rs](../../tests/inc/rta_test.rs) | Tests exercising debug-variant no-op behavior |
