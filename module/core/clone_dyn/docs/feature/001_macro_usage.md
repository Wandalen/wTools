# Feature: Macro-Based Cloning

### Scope

- **Purpose**: Enable `Clone` for `Box<dyn Trait>` via a single `#[clone_dyn]` attribute on a trait definition.
- **Responsibility**: Generate all four `impl Clone for Box<dyn Trait[+Send][+Sync]>` blocks automatically.
- **In Scope**: Any trait annotated with `#[clone_dyn]`, all `Send`/`Sync` auto-trait combinations, traits with generics and where-clauses.
- **Out of Scope**: Non-Box smart pointers (`Rc`, `Arc`), custom clone behavior, traits whose implementors do not implement `Clone`.

### Design

Annotating a trait definition with `#[clone_dyn]` is the only change required to make `Box<dyn Trait>` cloneable. The macro injects `where Self: CloneDyn` into the trait's where clause (providing an object-safe clone indirection method) and emits four `Clone` impl blocks for `Box<dyn Trait + 'c>`, covering the base, `+Send`, `+Sync`, and `+Send+Sync` variants. No changes are required on trait implementors or at call sites.

The one-liner form is the primary ergonomics goal: zero boilerplate for users beyond the attribute itself.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `002_manual_impl.md` | Alternative pattern without macro |
| doc | `../invariant/001_box_only.md` | Box-only restriction on generated impls |
| doc | `../api/001_facade_api.md` | Re-export surface and feature flags |
| source | `../../src/lib.rs` | Facade re-exports wiring |
| test | `../../tests/inc/basic.rs` | Macro-based cloning tests |
