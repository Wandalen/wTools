# API: Clone Dyn Attribute

Public API contract for the `#[clone_dyn]` attribute macro.

### Scope

- **Purpose:** Define the caller-visible contract for `#[clone_dyn]`: valid forms, accepted properties, input requirements, and guaranteed outputs.
- **Responsibility:** Be the authoritative specification that callers and the `clone_dyn` facade rely on.
- **In Scope:** Valid invocation forms, attribute properties, trait item constraints, generated impl guarantees.
- **Out of Scope:** Internal parsing logic (`algorithm/001_macro_expansion.md`), end-user ergonomics (handled by `clone_dyn` facade).

### Abstract

`#[clone_dyn]` is an outer attribute applied to a trait definition. It injects a `CloneDyn` supertrait bound and emits four `Clone` impl blocks for `Box<dyn Trait>` across `Send`/`Sync` marker combinations. The macro is consumed through the `clone_dyn` facade crate, which re-exports it.

### Operations

Add `clone_dyn` to `[dependencies]` and import the macro by name. Apply it as an outer attribute directly before the `trait` keyword.

**Attribute Properties**

| Property | Form | Default | Effect |
|----------|------|---------|--------|
| _(none)_ | `#[clone_dyn]` | — | Standard expansion: supertrait injection + 4 `Clone` impls |
| `debug` | `#[clone_dyn(debug)]` | `false` | Same as above, plus prints expanded tokens to stdout |

**Input Requirements**

- Input MUST be a trait item. Applying to structs, enums, or functions is a compile error.
- All generic parameters of the input trait are preserved verbatim in generated impls.
- Existing `where` clauses are extended (not replaced) with `Self: clone_dyn::CloneDyn`.

**Generated Output**

For a generic trait `Foo<T>`, the macro emits: the original trait with `Self: CloneDyn` added to its where clause, and four `Clone` impl blocks covering `Box<dyn Foo<T> + 'c>` plus the `+Send`, `+Sync`, and `+Send+Sync` variants, each delegating to `clone_into_box`.

### Error Handling

All errors are compile-time only. Non-trait input produces a descriptive compile error. An unknown attribute property produces a compile error listing `"debug"` as the only known keyword.

### Compatibility Guarantees

Stable since `clone_dyn_meta` v0.58.0 and `clone_dyn` v0.62.0. The `debug` property is informational-only and not semver-stable — it may be removed in a future release.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `../feature/001_clone_dyn_macro.md` | Feature rationale and attribute configuration |
| doc | `../algorithm/001_macro_expansion.md` | Implementation of the expansion steps |
| source | `../../src/clone_dyn.rs` | Canonical implementation |
| test | `../../tests/smoke_test.rs` | Attribute macro smoke test |
