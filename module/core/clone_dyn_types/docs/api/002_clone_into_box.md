# API: Clone Into Box

### Scope

- **Purpose**: Provide the safe public API for cloning DSTs and trait objects.
- **Responsibility**: Specify the function signatures, usage patterns, error conditions, and stability guarantees.
- **In Scope**: `clone_into_box` for unsized types; `clone` convenience function for sized types.
- **Out of Scope**: `CloneDyn` trait internals; direct `__clone_dyn` invocation.

### Abstract

`clone_into_box` clones any `CloneDyn` type into a correctly-typed `Box<T>`, encapsulating all unsafe fat-pointer operations. `clone` is a convenience wrapper around standard `Clone::clone` for sized types.

### Operations

**`clone_into_box`** — accepts any `T: ?Sized + CloneDyn` by shared reference and returns `Box<T>`. Internally performs fat pointer surgery to produce the cloned box. Primary use case: implementing `Clone for Box<dyn Trait>`.

**`clone`** — accepts any `T: Clone` by shared reference and returns `T`. Thin wrapper over standard `Clone::clone` for use in generic contexts.

**Usage Patterns**

- Trait object cloning: implement `Clone for Box<dyn Trait>` with a body that calls `clone_into_box(&**self)`.
- Slice cloning: pass a double-reference `&&[T]` coerced to `&dyn CloneDyn` to handle the DST coercion requirement.
- Sized type cloning: use `clone(&value)` as an ergonomic alternative to `value.clone()`.

### Error Handling

All errors are compile-time. Passing a type that does not implement `CloneDyn` produces a trait bound error. Passing a single-reference DST (`&[T]` rather than `&&[T]`) produces E0277 at the call site.

### Compatibility Guarantees

Stable since `clone_dyn_types` v0.48.0. The function signatures of `clone_into_box` and `clone` are semver-stable.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `../algorithm/001_fat_pointer_surgery.md` | Implementation of `clone_into_box` |
| doc | `001_clone_dyn_trait.md` | `CloneDyn` trait this function operates on |
| doc | `../invariant/003_usage_constraints.md` | Double-reference requirement for DSTs |
| source | `../../src/lib.rs` | Function implementations |
| test | `../../tests/smoke_test.rs` | Function usage verification |
