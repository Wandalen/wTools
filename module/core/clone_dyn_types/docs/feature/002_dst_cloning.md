# Feature: DST Cloning

### Scope

- **Purpose**: Enable cloning of dynamically sized types and boxed trait objects.
- **Responsibility**: Specify which DST kinds are supported and how the clone is produced.
- **In Scope**: Trait objects (`dyn Trait`), slices (`[T]`), string slices (`str`).
- **Out of Scope**: Procedural macro generation (belongs to `clone_dyn` crate); non-Clone types.

### Design

DST cloning requires separating the fat pointer representing a `&dyn CloneDyn` into its two components: the data pointer (pointing to the concrete value on the heap) and the metadata (vtable pointer for trait objects; byte length for slices and strings). `clone_into_box` replaces the data component with a freshly cloned allocation produced by `__clone_dyn`, then re-assembles the fat pointer with the original metadata to yield a correctly typed `Box<T>`.

This mechanism works uniformly for all DSTs — the `CloneDyn` blanket impl covers sized types, while explicit DST impls handle `[T]` and `str`. For trait objects, any trait annotated with `#[clone_dyn]` or manually given a `Clone for Box<dyn Trait>` impl gains this capability.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `../algorithm/001_fat_pointer_surgery.md` | Internal DST cloning mechanism |
| doc | `../api/001_clone_dyn_trait.md` | CloneDyn trait definition |
| doc | `../api/002_clone_into_box.md` | Public entry point for DST cloning |
| doc | `../pattern/001_sealed_trait.md` | Prevents unsound external implementations |
| source | `../../src/lib.rs` | CloneDyn blanket impl and DST impls |
| test | `../../tests/smoke_test.rs` | DST cloning verification |
