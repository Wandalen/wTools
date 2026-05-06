# Feature: Layered Module Interface

### Scope

- **Purpose**: Organize module items into five visibility layers with controlled propagation through the module hierarchy.
- **Responsibility**: Navigational hub for all source, test, doc, and task artifacts of the mod_interface! macro.
- **In Scope**: Five-layer namespace model, propagation semantics, directive forms, layer composition, and all associated artifacts.
- **Out of Scope**: Runtime behavior; this macro operates at compile time only. Cross-crate module organization is out of scope.

### Design

Modules in complex library hierarchies need more than two visibility levels. The layered module interface pattern introduces five predefined visibility layers that form a cascade, giving library authors precise control over which callers at different positions in the hierarchy can access which items.

The five layers from most to least restrictive are:

- **Private**: Implementation namespace written by the developer; not generated, never propagated.
- **Own**: Items accessible only within this module; not propagated to any parent.
- **Orphan**: Items propagated to the immediate parent module (parent's own and root namespaces).
- **Exposed**: Items propagated to all ancestor modules up the hierarchy.
- **Prelude**: Items propagated to all ancestors and intended for glob import at the call site.

The macro generates four named namespace modules (own, orphan, exposed, prelude) that form a cascade: each namespace includes all items from the one below it, so a prelude item is accessible from all four generated namespaces.

**Directive forms** let the developer assign items to layers, wire child modules into the cascade, or load external modules directly into a named layer. The `#![debug]` directive prints the generated expansion during compilation for inspection.

**Bootstrap constraint**: the mod_interface crate itself cannot invoke its own macro — macro implementation crates cannot use the macro being implemented. It uses conventional module organization instead. The macro implementation lives in the companion `mod_interface_meta` crate, which is re-exported by the user-facing `mod_interface` facade.

**No-std compatibility**: the generated code uses no standard library runtime features; the macro is compatible with no_std crates.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Re-export point for the mod_interface! macro |
| source | `../mod_interface_meta/src/lib.rs` | Proc-macro entry point |
| source | `../mod_interface_meta/src/impls.rs` | Core code generation: directive handlers and namespace module generation |
| source | `../mod_interface_meta/src/record.rs` | AST record types for parsed directives |
| source | `../mod_interface_meta/src/visibility.rs` | Layer enumeration and clause-kind mapping |
| source | `../mod_interface_meta/src/use_tree.rs` | Path prefix detection for directive expansion |
| test | `tests/smoke_test.rs` | Basic importability check |
| test | `tests/tests.rs` | Core propagation and layer composition tests |
| test | `../mod_interface_meta/tests/integration_test.rs` | Four-layer namespace integration tests |
| test | `../mod_interface_meta/tests/propagation_bug_test.rs` | Cascade propagation correctness |
| test | `../mod_interface_meta/tests/corner_cases_test.rs` | Corner case coverage for all directive forms |
| doc | `docs/invariant/001_propagation_rules.md` | Propagation invariants for the cascade |
| doc | `docs/api/001_mod_interface_macro.md` | DSL directive API specification |
| doc | `docs/pattern/001_exposure_level_cascade.md` | Five-layer cascade architectural pattern |
| doc | `docs/pattern/002_absorption_pattern.md` | Meta/runtime crate split pattern |
| task | `task/completed/001_fix_use_layer_reexports.md` | Fix for layer-aware propagation in record_use_implicit |
