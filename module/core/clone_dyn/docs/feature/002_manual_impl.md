# Feature: Manual Clone Implementation

### Scope

- **Purpose**: Enable `Clone` for `Box<dyn Trait>` without the `#[clone_dyn]` macro, via direct dependency on `clone_dyn_types`.
- **Responsibility**: Document the low-level pattern for users needing explicit control or wishing to avoid proc-macro compilation.
- **In Scope**: Manual `impl Clone for Box<dyn Trait>`, direct `CloneDyn` supertrait usage, `clone_into_box` call.
- **Out of Scope**: Macro-generated code, any runtime behavior differences from the macro path.

### Design

Depend on `clone_dyn` with only the `clone_dyn_types` feature (omit `derive_clone_dyn`):

```rust
trait MyTrait: clone_dyn::CloneDyn
{
  fn method( &self );
}

impl Clone for Box< dyn MyTrait >
{
  fn clone( &self ) -> Self
  {
    clone_dyn::clone_into_box( &**self )
  }
}
```

This pattern is runtime-equivalent to the macro path. It exists for:
- Users who want to see exactly what the macro generates.
- Conditional compilation scenarios.
- Custom `Clone` behavior (the impl body can be modified).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| feature | `001_macro_usage.md` | Macro-based alternative |
| api | `../api/001_facade_api.md` | `clone_into_box` and `CloneDyn` re-exports |
