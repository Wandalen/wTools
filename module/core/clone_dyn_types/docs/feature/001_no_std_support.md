# Feature: No-Std Support

### Scope

- **Purpose**: Enable `clone_dyn_types` in no_std + alloc environments.
- **Responsibility**: Declare `#![no_std]` at crate root and ensure all primitives work with only `alloc`.
- **In Scope**: `#![no_std]` with `extern crate alloc`; all CloneDyn primitives under no_std.
- **Out of Scope**: std-only environments (these still work, but are not the target constraint).

### Design

`clone_dyn_types` uses `#![no_std]` at crate root with a conditional `extern crate alloc` declaration for heap allocation. All `CloneDyn` implementations, blanket impls, and `clone_into_box` rely only on `alloc::boxed::Box` — no `std` features are needed. The crate compiles under both `std` and `no_std + alloc` environments without any conditional compilation in application code.

The zero-dependency invariant is the structural precondition that makes no_std possible: every production dependency would risk pulling in `std`.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `../invariant/001_zero_dependencies.md` | Zero production deps enable no_std |
| doc | `../api/001_clone_dyn_trait.md` | CloneDyn trait definition compatible with no_std |
| source | `../../src/lib.rs` | no_std declaration and alloc usage |
| test | `../../tests/smoke_test.rs` | no_std compilation verification |
