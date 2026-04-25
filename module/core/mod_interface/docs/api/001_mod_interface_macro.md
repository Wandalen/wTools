# API: mod_interface Macro DSL

### Scope

- **Purpose**: Define the DSL accepted by mod_interface! and the namespace-assignment semantics of each directive form.
- **Responsibility**: Specifies all valid directive categories, their parameters, error conditions, and compatibility guarantees for callers.
- **In Scope**: Directive categories, layer assignment semantics, error conditions, and versioning policy.
- **Out of Scope**: Internal expansion mechanics; those are implementation details of mod_interface_meta/src/impls.rs.

### Abstract

The mod_interface! macro accepts a body of namespace directives and expands at compile time to generate four named namespace modules. Callers place the invocation inside any Rust module alongside a private namespace block containing the module's implementation.

### Operations

**Layer assignment directives** — assign an item from the module's private namespace to a named visibility layer. The layer name determines propagation:
- Assigning to own: item is accessible only within this module; not propagated to parent modules.
- Assigning to orphan: item is propagated to the immediate parent module's own and root namespaces.
- Assigning to exposed: item is propagated to all ancestor modules' exposed namespaces and above.
- Assigning to prelude: item is propagated to all ancestor modules' prelude namespaces and intended for glob import.

An optional rename form is supported on any layer assignment directive; the alias becomes the exported identifier in all target namespaces.

**Layer wiring directive** — integrates an existing Rust submodule into the parent's cascade. The submodule's orphan, exposed, and prelude namespaces are wired into the parent's own, exposed, and prelude namespaces respectively, following the propagation rules invariant.

**Micro-module directive** — loads an external module file directly into a named layer namespace. Requires a corresponding module file on the filesystem (standard Rust module resolution applies).

**Debug directive** — when placed inside the invocation body, causes the macro to emit the full generated expansion as a compiler message during compilation. Useful for inspecting the generated namespace structure.

Multiple directives of any kind may appear in a single invocation body.

### Error Handling

Malformed directive syntax produces a compile error from the macro's parser. Unsupported forms include: bare item paths without a layer prefix, and grouped path expressions. Missing filesystem files for layer-wiring or micro-module directives produce standard Rust module resolution errors. All errors are compile-time only; no runtime error handling is required.

### Compatibility Guarantees

The DSL forms described here constitute the public interface of the mod_interface crate. Breaking changes to directive syntax or semantics require a major version bump. New directive forms are backward-compatible minor additions. The `mod_interface` and `mod_interface_meta` crates are versioned in lockstep; breaking changes affect both.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `../mod_interface_meta/src/lib.rs` | Proc-macro entry point; routes token stream to directive handlers |
| source | `../mod_interface_meta/src/impls.rs` | Directive parsing and expansion handlers |
| source | `../mod_interface_meta/src/record.rs` | AST record types: ElementType, Record, Records |
| source | `../mod_interface_meta/src/visibility.rs` | ClauseKind enum; valid layer name list |
| test | `../mod_interface_meta/tests/corner_cases_test.rs` | Corner case coverage for all DSL directive forms |
| test | `../mod_interface_meta/tests/smoke_test.rs` | Basic directive smoke check |
| doc | `docs/feature/001_layered_module_interface.md` | Feature hub for the mod_interface! macro |
| doc | `docs/invariant/001_propagation_rules.md` | Propagation invariants governing layer assignment semantics |

### Sources

| File | Notes |
|------|-------|
| [../../spec.md](../../spec.md) | Combined source; Public API (Macro, Usage Syntax, Required Module Structure) and Scope item 4 (Layer Directives) sections contributed to this doc instance. spec.md has been deleted — Sources entry retained as migration record. |
