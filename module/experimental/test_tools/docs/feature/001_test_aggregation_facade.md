# Feature: Test Aggregation Facade

### Scope

- **Purpose**: Provides a single dependency giving access to all testing utilities from constituent crates.
- **Responsibility**: Documents the re-export aggregation layer, namespace module structure, and feature cascading.
- **In Scope**: Test aggregation, namespace layers, macro re-exports, standalone build mode.
- **Out of Scope**: Individual constituent crate APIs; see their own documentation.

### Design

The crate aggregates testing utilities from multiple ecosystem crates into a unified namespace hierarchy. Four namespace layers (`own`, `orphan`, `exposed`, `prelude`) provide controlled API surfaces at different visibility levels.

Collection constructor macros require explicit re-export because macros marked with the macro export attribute do not propagate through module re-exports. They are placed in the `exposed` layer to avoid root-level ambiguity with the standard library `vec!` macro.

In standalone mode, transient dependency sources are included via path attributes, bypassing the Cargo dependency graph to break circular dependencies.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Root aggregation module with namespace layers |
| source | `src/standalone.rs` | Standalone mode implementation |
| test | `tests/smoke_test.rs` | Comprehensive aggregation smoke test |
| test | `tests/api_stability_facade_tests.rs` | API stability facade verification |
| test | `tests/mod_interface_aggregation_tests.rs` | Namespace aggregation tests |
| doc | `docs/invariant/001_no_cfg_gate_on_namespace.md` | Invariant protecting namespace visibility |
| doc | `docs/invariant/002_no_vec_macro_reexport.md` | Invariant preventing macro ambiguity |
| doc | `docs/pattern/001_namespace_layers.md` | Pattern governing module layer hierarchy |
