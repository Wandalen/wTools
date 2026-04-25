# Feature: Clone Dyn Macro

The `#[clone_dyn]` procedural attribute macro that enables `Box<dyn Trait>` cloning.

### Scope

- **Purpose:** Provide compile-time code generation that makes annotated trait definitions cloneable as boxed trait objects.
- **Responsibility:** Inject `Self: clone_dyn::CloneDyn` into the trait's where clause and emit four `Clone` impl blocks for `Box<dyn Trait>` across `Send`/`Sync` marker combinations.
- **In Scope:** Trait definitions annotated with `#[clone_dyn]`, generic trait parameters, where clause extension, optional `debug` property for expansion introspection.
- **Out of Scope:** Runtime cloning behavior, non-trait items (structs/enums/functions), user-facing API (route through `clone_dyn` facade), direct end-user use.

### Design

Rust's compilation model requires proc-macro crates to be separate from library crates. This crate is the proc-macro implementation; the `clone_dyn` facade crate re-exports the macro with a more ergonomic API. Direct use of this crate by end-users is not intended.

### Feature Flags

| Feature | Default | Governs |
|---------|---------|---------|
| `enabled` | off | Activates macro implementation and all deps |
| `full` | off | Alias for `enabled` |

### Attribute Properties

| Property | Form | Default | Effect |
|----------|------|---------|--------|
| `debug` | `#[clone_dyn(debug)]` | `false` | Prints macro-expanded output via `diag::report_print` to aid macro development |

### Dependency Boundaries

- **Upstream:** `macro_tools` — syntax parsing (syn, quote, proc-macro2 wrappers); `component_model_types` — `Assign` trait for attribute property dispatch
- **Downstream:** `clone_dyn` — re-exports this crate's macro and provides `CloneDyn` + `clone_into_box` runtime

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `../algorithm/001_macro_expansion.md` | Step-by-step code generation logic |
| doc | `../api/001_clone_dyn_attr.md` | Invocation contract and output specification |
| source | `../../src/clone_dyn.rs` | Canonical implementation |
| test | `../../tests/smoke_test.rs` | Feature smoke test |
