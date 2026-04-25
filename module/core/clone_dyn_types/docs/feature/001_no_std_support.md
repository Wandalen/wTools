# Feature: No-Std Support

### Scope

**Purpose**: Enable `clone_dyn_types` in no_std + alloc environments.
**In Scope**: `#![no_std]` with `extern crate alloc`; all CloneDyn primitives under no_std.
**Out of Scope**: std-only environments (these still work, but are not the target constraint).

### Statement

`clone_dyn_types` MUST compile and function correctly in `#![no_std]` environments
that provide the `alloc` crate, requiring only `extern crate alloc` for heap allocation.

### Acceptance Criteria

- AC-1: `#![no_std]` at crate root compiles without error under stable Rust
- AC-2: `extern crate alloc` is the only required allocation declaration
- AC-3: All CloneDyn trait implementations compile under no_std + alloc
- AC-4: `clone_into_box` produces a correctly-typed `Box<T>` from `alloc::boxed`

### Cross-References

- `invariant/001_zero_dependencies.md` — zero production deps enable no_std
- `api/001_clone_dyn_trait.md` — CloneDyn trait definition compatible with no_std
