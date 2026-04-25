# Feature: DST Cloning

### Scope

**Purpose**: Enable cloning of dynamically sized types and boxed trait objects.
**In Scope**: Trait objects (`dyn Trait`), slices (`[T]`), string slices (`str`).
**Out of Scope**: Procedural macro generation (belongs to `clone_dyn` crate); non-Clone types.

### Statement

`clone_dyn_types` MUST enable cloning of any type implementing `CloneDyn` into a
correctly-typed `Box<T>` via `clone_into_box`, including trait objects and DSTs.

### Acceptance Criteria

- AC-1: `Box<dyn Trait>` is `Clone` when `Trait: CloneDyn`
- AC-2: `&[T]` coercible to `&dyn CloneDyn`; `clone_into_box` returns `Box<[T]>`
- AC-3: `&str` coercible to `&dyn CloneDyn`; `clone_into_box` returns `Box<str>`
- AC-4: Cloned value equals original; no aliasing between original and clone

### Cross-References

- `algorithm/001_fat_pointer_surgery.md` — internal DST cloning mechanism
- `api/001_clone_dyn_trait.md` — CloneDyn trait definition
- `api/002_clone_into_box.md` — public entry point for DST cloning
- `pattern/001_sealed_trait.md` — prevents unsound external implementations
