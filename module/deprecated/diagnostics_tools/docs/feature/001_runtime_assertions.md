# Feature: Runtime Assertions

### Scope

- **Purpose**: Provide always-active runtime assertion macros for defensive programming and test validation.
- **Responsibility**: Documents the runtime assertions feature — its macro set, behavioral contracts, and governing invariants.
- **In Scope**: The a_true, a_false, a_id, a_not_id macros and their debug-only a_dbg_* variants.
- **Out of Scope**: Compile-time assertions, memory layout validation, assertion formatting internals.

### Design

The runtime assertions feature provides a unified set of macros for expressing program invariants that are checked at runtime. All macros invoke panic when their condition is not met, matching standard Rust assertion semantics.

The feature includes two categories of macros. Always-active assertions (a_true, a_false, a_id, a_not_id) run in both debug and release builds. Debug-only variants (a_dbg_true, a_dbg_false, a_dbg_id, a_dbg_not_id) are active only when debug assertions are enabled and compile to no-ops in release builds.

Equality assertions (a_id, a_not_id) delegate to pretty_assertions for colored diff output, making assertion failures substantially easier to diagnose.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [api/001_runtime_assertion_macros.md](../api/001_runtime_assertion_macros.md) | Public runtime assertion macro set |
| doc | [invariant/001_debug_variants_release_noop.md](../invariant/001_debug_variants_release_noop.md) | Debug variants compile to no-ops in release builds |
| doc | [invariant/002_pretty_diff_output.md](../invariant/002_pretty_diff_output.md) | Equality assertions produce colored diff output |
| source | [src/diag/rta.rs](../../src/diag/rta.rs) | Runtime assertion macro implementations |
| test | [tests/inc/rta_test.rs](../../tests/inc/rta_test.rs) | Behavioral tests for runtime assertion macros |
