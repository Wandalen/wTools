# Feature: mod_interface Macro

### Scope

- **Purpose**: Organize module items into four visibility layers via a compile-time DSL, enabling precise propagation control through module hierarchies.
- **Responsibility**: Navigational hub for all source, test, and doc artifacts of the mod_interface! macro.
- **In Scope**: Namespace directive semantics, layer composition, cascade structure, and all associated artifacts.
- **Out of Scope**: Runtime behavior; this macro generates code at compile time only.

### Design

The macro generates a four-layer cascade of namespace modules at compile time, automating visibility propagation that would otherwise require manually written re-exports at every module boundary. Each directive maps an item to a named visibility layer; the generated cascade handles propagation to parent modules automatically.

This crate (`mod_interface_meta`) is the proc-macro companion; use the `mod_interface` facade crate for direct access.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Proc-macro entry point; routes token stream to impls |
| source | `src/impls.rs` | Core expansion; directive handlers; namespace module generation |
| source | `src/record.rs` | Parsed directive record types |
| source | `src/visibility.rs` | Layer kind classification; layer-to-namespace mapping |
| source | `src/use_tree.rs` | Path tree parsing; path prefix logic |
| test | `tests/smoke_test.rs` | Compile-time importability check |
| test | `tests/integration_test.rs` | Four-layer namespace integration tests |
| test | `tests/corner_cases_test.rs` | Corner case coverage for all directive forms |
| test | `tests/propagation_bug_test.rs` | Cascade propagation correctness verification |
| doc | `docs/api/001_mod_interface_macro.md` | DSL operations, directives, and error conditions |
| doc | `docs/invariant/001_namespace_cascade.md` | Four-layer cascade propagation invariant |

