# Feature: Manual Clone Implementation

### Scope

- **Purpose**: Enable `Clone` for `Box<dyn Trait>` without the `#[clone_dyn]` macro, via direct dependency on `clone_dyn_types`.
- **Responsibility**: Document the low-level pattern for users needing explicit control or wishing to avoid proc-macro compilation.
- **In Scope**: Manual `impl Clone for Box<dyn Trait>`, direct `CloneDyn` supertrait usage, `clone_into_box` call.
- **Out of Scope**: Macro-generated code, any runtime behavior differences from the macro path.

### Design

The manual pattern requires two additions: `clone_dyn::CloneDyn` as a supertrait in the trait definition, and an explicit `Clone` implementation for `Box<dyn Trait>` that delegates to `clone_dyn::clone_into_box`. This is runtime-equivalent to the macro path — the macro produces exactly this code — but trades ergonomics for explicit visibility and avoids proc-macro compilation overhead.

This pattern is useful for: understanding what the macro generates, conditional compilation scenarios, and custom clone behavior (the impl body can be modified, unlike the macro-generated one).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `001_macro_usage.md` | Macro-based alternative |
| doc | `../api/001_facade_api.md` | `clone_into_box` and `CloneDyn` re-exports |
| source | `../../src/lib.rs` | Facade re-exports wiring |
| test | `../../tests/inc/basic_manual.rs` | Manual implementation tests |
