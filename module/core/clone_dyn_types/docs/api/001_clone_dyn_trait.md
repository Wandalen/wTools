# API: CloneDyn Trait

### Scope

- **Purpose**: Define the public `CloneDyn` trait contract for DST cloning.
- **Responsibility**: Specify the trait interface, blanket implementations, sealing constraints, and stability guarantees.
- **In Scope**: Trait definition, blanket implementations, sealing mechanism.
- **Out of Scope**: The `clone_into_box` function; procedural macro generation.

### Abstract

`CloneDyn` is a sealed, object-safe trait that enables type-erased cloning of trait objects and DSTs without requiring `Clone` as a supertrait (which would violate object safety).

### Operations

`CloneDyn` exposes a single method `__clone_dyn` — a type-erased clone that returns a raw heap pointer. The trait is sealed via a private `Sealed` supertrait and a `DontCallMe` marker parameter that prevents both external implementation and direct invocation.

**Trait Items**

- `__clone_dyn(&self, _: DontCallMe) -> *mut ()` — object-safe clone indirection; returns heap pointer to cloned value

**Blanket Implementations**

`CloneDyn` is implemented for:
- All `T: Clone` (sized types) — via blanket impl
- `[T]` where `T: Clone` — explicit DST impl for slices
- `str` — explicit DST impl for string slices

### Error Handling

All API errors are compile-time. Attempting to implement `CloneDyn` outside this crate produces a compile error because `Sealed` is private. Calling `__clone_dyn` directly is prevented by the `DontCallMe` marker parameter.

### Compatibility Guarantees

Stable since `clone_dyn_types` v0.48.0. `CloneDyn` and the blanket impls are semver-stable. The `__clone_dyn` method signature is an implementation detail — not part of the public API contract.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `../pattern/001_sealed_trait.md` | Sealing mechanism used by this trait |
| doc | `002_clone_into_box.md` | Public function that invokes `__clone_dyn` |
| doc | `../invariant/002_memory_safety.md` | `__clone_dyn` return value contract |
| source | `../../src/lib.rs` | Trait definition and blanket impls |
| test | `../../tests/smoke_test.rs` | Trait usage verification |
