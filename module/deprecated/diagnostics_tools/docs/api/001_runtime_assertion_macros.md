# API: Runtime Assertion Macros

### Scope

- **Purpose**: Define the public runtime assertion macro set exposed by the diagnostics_runtime_assertions feature.
- **Responsibility**: Documents the interface contract for all runtime assertion macros — their invocation forms and failure semantics.
- **In Scope**: a_true, a_false, a_id, a_not_id, a_dbg_true, a_dbg_false, a_dbg_id, a_dbg_not_id.
- **Out of Scope**: Internal delegation to pretty_assertions, debug_assert expansion internals.

### Abstract

A macro set for expressing program invariants that are checked at runtime. Available when the diagnostics_runtime_assertions feature is enabled. Provides always-active and debug-only variants, covering boolean checks and equality comparisons with enhanced diff output.

### Operations

**Always-active macros** — panic in both debug and release builds:

- a_true — asserts a boolean expression is true; accepts an optional panic message.
- a_false — asserts a boolean expression is false; accepts an optional panic message.
- a_id — asserts two expressions are equal using value equality; produces a colored diff on failure.
- a_not_id — asserts two expressions are not equal using value equality; produces a colored diff on failure.

**Debug-only variants** — compile to no-ops in release builds:

- a_dbg_true — debug-build variant of a_true.
- a_dbg_false — debug-build variant of a_false.
- a_dbg_id — debug-build variant of a_id; delegates to a_id when debug assertions are active.
- a_dbg_not_id — debug-build variant of a_not_id; delegates to a_not_id when debug assertions are active.

All macros are exported at the crate root and re-exported as assert_eq / assert_ne aliases in the orphan namespace.

### Error Handling

All macros invoke panic on assertion failure, consistent with standard Rust assertion semantics. The panic message includes the expression text; equality assertion failures additionally include a colored side-by-side diff. No macro can fail for reasons other than the asserted condition being false.

### Compatibility Guarantees

The always-active macro set (a_true, a_false, a_id, a_not_id) is stable across minor versions. The debug-only variants (a_dbg_*) are stable across minor versions and preserve their no-op-in-release guarantee. The assert_eq / assert_ne re-export aliases are stable. The diagnostics_runtime_assertions feature name is permanent.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/001_runtime_assertions.md](../feature/001_runtime_assertions.md) | Runtime assertions feature context |
| source | [src/diag/rta.rs](../../src/diag/rta.rs) | Runtime assertion macro implementations |
| test | [tests/inc/rta_test.rs](../../tests/inc/rta_test.rs) | Behavioral tests for runtime assertion macros |
