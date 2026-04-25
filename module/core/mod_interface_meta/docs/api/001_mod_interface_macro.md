# API: mod_interface Macro DSL

### Scope

- **Purpose**: Define the DSL operations accepted by mod_interface! and their namespace-assignment semantics.
- **Responsibility**: Specifies all valid directive categories, their parameters, and error conditions for callers.
- **In Scope**: Namespace directives, layer directive, rename support, supported path forms, and error conditions.
- **Out of Scope**: Internal expansion mechanics; those are implementation details of src/impls.rs.

### Abstract

The mod_interface! proc-macro accepts a DSL body declaring which items from a module's private namespace should be re-exported into which visibility layers. It expands at compile time to generate four named namespace modules.

### Operations

**Namespace directives** — assign an item to a named visibility layer and all layers of higher visibility:
- Assigning to own: item accessible only from the own layer; not propagated to parent modules.
- Assigning to orphan: item accessible from orphan and own layers; propagated to immediate parent's own namespace.
- Assigning to exposed: item accessible from exposed, orphan, and own layers; propagated to all ancestors' exposed namespaces.
- Assigning to prelude: item accessible from all four layers; propagated to all ancestors' prelude namespaces and intended for glob import.

An optional rename form is supported on any namespace directive; the alias becomes the exported identifier in all target layers.

**Layer directive** — integrates a submodule into the parent cascade: wires the submodule's orphan, exposed, and prelude namespaces into the parent's own, exposed, and prelude namespaces respectively.

**Micro-module directive** — loads an external module file directly into a named layer namespace. Requires a corresponding module file to exist on the filesystem (standard module resolution applies).

**Debug directive** — when placed inside the invocation body, emits the full generated expansion as a compiler message during compilation.

Multiple directives of any kind may appear in a single invocation body.

### Error Handling

Malformed DSL produces a compile error emitted via the macro's parser. Unsupported forms: bare item paths without a layer prefix, and grouped path expressions. Missing filesystem files for layer or micro-module directives produce standard module resolution errors.

### Compatibility Guarantees

The DSL is the public interface of the mod_interface_meta proc-macro companion crate. Breaking DSL changes require a major version bump in both mod_interface_meta and the mod_interface facade crate. Minor additions (new directive forms) are backward-compatible.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/impls.rs` | Directive parsing and expansion handlers |
| source | `src/record.rs` | Parsed directive record types for the DSL abstract syntax tree |
| source | `src/visibility.rs` | Layer kind classification; valid directive name list |
| test | `tests/corner_cases_test.rs` | Corner case coverage for all DSL directive forms |
| doc | `docs/feature/001_mod_interface.md` | Feature hub for the mod_interface! macro |
| doc | `docs/invariant/001_namespace_cascade.md` | Cascade invariant governing layer propagation |

