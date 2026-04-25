# Feature: Macro-Based Cloning

### Scope

- **Purpose**: Enable `Clone` for `Box<dyn Trait>` via a single `#[clone_dyn]` attribute on a trait definition.
- **Responsibility**: Generate all four `impl Clone for Box<dyn Trait[+Send][+Sync]>` blocks automatically.
- **In Scope**: Any trait annotated with `#[clone_dyn]`, all `Send`/`Sync` auto-trait combinations, traits with generics and where-clauses.
- **Out of Scope**: Non-Box smart pointers (`Rc`, `Arc`), custom clone behavior, traits whose implementors do not implement `Clone`.

### Design

`Clone::clone() -> Self` is not object-safe — `Self` requires compile-time size knowledge. This crate solves it via two steps:

1. The `#[clone_dyn]` macro adds `where Self: CloneDyn` as a supertrait bound. `CloneDyn` provides a `__clone_dyn` method returning a type-erased raw heap pointer.
2. The macro emits four `impl Clone for Box<dyn Trait + 'c>` blocks (base, `+Send`, `+Sync`, `+Send+Sync`), each calling `clone_dyn_types::clone_into_box(&**self)`.

Usage:

```rust
#[ clone_dyn ]
pub trait MyTrait
{
  fn method( &self );
}
// Box<dyn MyTrait> is now Clone
```

The one-liner form is the primary ergonomics goal: no boilerplate on trait implementors, no manual `impl Clone` blocks.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| feature | `002_manual_impl.md` | Alternative pattern without macro |
| invariant | `../invariant/001_box_only.md` | Box-only restriction on generated impls |
| api | `../api/001_facade_api.md` | Re-export surface and feature flags |
