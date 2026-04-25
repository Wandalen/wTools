# Feature: mod_interface Macro

### Scope

- **Purpose**: Organize module items into four visibility layers via a compile-time DSL, enabling precise propagation control through module hierarchies.
- **Responsibility**: Navigational hub for all source, test, and doc artifacts of the mod_interface! macro.
- **In Scope**: Namespace directive semantics, layer composition, cascade structure, and all associated artifacts.
- **Out of Scope**: Runtime behavior; this macro generates code at compile time only.

### Design

The macro accepts a body of namespace directives and generates four named namespace modules: own, orphan, exposed, prelude. Each module re-exports the one below it, forming a cascade where items declared at a lower layer are accessible from all higher layers.

Namespace directives assign items to layers based on their intended propagation:
- Own: item accessible only within this module; not propagated to parent modules.
- Orphan: item propagated to the immediate parent module.
- Exposed: item propagated to all ancestor modules.
- Prelude: item propagated to all ancestors and intended for glob import.

The layer directive integrates an existing Rust submodule into the parent's cascade. The submodule's orphan, exposed, and prelude namespaces are wired into the parent's corresponding namespaces following the propagation invariant.

This crate (`mod_interface_meta`) is the proc-macro companion and should not be used directly. The `mod_interface` facade crate re-exports the macro with documentation.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Proc-macro entry point; routes token stream to impls |
| source | `src/impls.rs` | Core expansion; directive handlers; namespace module generation |
| source | `src/record.rs` | AST record types for parsed DSL directives |
| source | `src/visibility.rs` | ClauseKind enum; layer-to-namespace mapping |
| source | `src/use_tree.rs` | UseTree parsing; path prefix logic |
| test | `tests/smoke_test.rs` | Compile-time importability check |
| test | `tests/integration_test.rs` | Four-layer namespace integration tests |
| test | `tests/corner_cases_test.rs` | Corner case coverage for all directive forms |
| doc | `docs/api/001_mod_interface_macro.md` | DSL operations, directives, and error conditions |
| doc | `docs/invariant/001_namespace_cascade.md` | Four-layer cascade propagation invariant |

### Sources

| File | Notes |
|------|-------|
| [../../spec.md](../../spec.md) | Combined source; Overview, Scope, Architecture, Design Rationale, and Related Crates sections contributed to this doc instance. spec.md has been deleted — Sources entry retained as migration record. |
