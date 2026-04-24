# API: mod_interface Macro DSL

### Scope

- **Purpose**: Define the DSL operations accepted by mod_interface! and their namespace-assignment semantics.
- **Responsibility**: Specifies all valid directive forms, their parameters, and error conditions for callers.
- **In Scope**: Namespace directives, layer directive, rename syntax, supported path forms, and error conditions.
- **Out of Scope**: Internal expansion mechanics; those are implementation details of src/impls.rs.

### Abstract

The `mod_interface!` proc-macro accepts a DSL body declaring which items from a module's private namespace should be re-exported into which visibility layers. It expands at compile time to generate four named namespace modules.

### Operations

**Namespace directives** — assign an item to a layer and all layers above it:
- `own use <path>` — item available from `own` and all layers above
- `orphan use <path>` — item available from `orphan` and all layers above
- `exposed use <path>` — item available from `exposed` and all layers above
- `prelude use <path>` — item available from all four layers

Rename syntax `<path> as <alias>` is supported on any namespace directive; the alias becomes the exported identifier in all target layers.

**Layer directive** — integrate a submodule into the parent's cascade:
- `layer <submodule>` — wires the submodule's `orphan`, `exposed`, and `prelude` sub-namespaces into the parent namespace hierarchy

Multiple directives of any kind may appear in a single `mod_interface!` invocation.

### Error Handling

Malformed DSL produces a compile error emitted via `syn::Error`. Unsupported forms: bare `use <path>` without a layer prefix, and group use expressions `use m::{ a, b }`. Missing filesystem files for `layer` directives produce standard Rust `mod` resolution errors.

### Compatibility Guarantees

The DSL is the public interface of the `mod_interface_meta` proc-macro companion crate. Breaking DSL changes require a major version bump in both `mod_interface_meta` and the `mod_interface` facade crate. Minor additions (new directive forms) are backward-compatible.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/impls.rs` | Directive parsing and expansion handlers |
| source | `src/record.rs` | AST record types: ElementType, Record, Records |
| source | `src/visibility.rs` | ClauseKind enum; VALID_VISIBILITY_LIST_STR |
| test | `tests/corner_cases_test.rs` | Corner case coverage for all DSL directive forms |
| doc | `docs/feature/001_mod_interface.md` | Feature hub for the mod_interface! macro |
| doc | `docs/invariant/001_namespace_cascade.md` | Cascade invariant governing layer propagation |

### Sources

| File | Notes |
|------|-------|
| [../../spec.md](../../spec.md) | Combined source; Public API, Namespace Directives, and Feature Flags sections contributed to this doc instance |
