# Data Structure: OptionalCow

### Scope

- **Purpose**: Provide a transparent wrapper for optional borrowed-or-owned field values with marker semantics.
- **Responsibility**: Document the OptionalCow structure, layout decisions, and phantom marker purpose.
- **In Scope**: Memory layout, phantom type parameter, relationship to field value forms.
- **Out of Scope**: Fields API usage patterns (→ `api/002_fields_api.md`); Fields feature design (→ `feature/002_fields_iteration.md`).

### Abstract

A newtype wrapper over an optional borrowed-or-owned value, carrying an additional phantom type parameter for value form discrimination. Used by the fields iteration subsystem to distinguish reference, owned, and optional access modes at the type level without runtime overhead.

### Structure

The wrapper is declared with transparent representation, ensuring it has the same memory layout as its inner optional borrowed-or-owned value. The phantom type parameter uses a function pointer phantom to avoid imposing additional trait bounds (specifically auto-trait bounds) on the wrapper. This means the wrapper's ability to cross thread boundaries is determined solely by the inner value, not by the marker type.

Two additional wrapper types exist in the same module (aref, maybe_as) but are not declared in the module tree — they are orphan files retained for potential future use.

### Operations

Construction from an inner value. Conversion to and from the underlying optional borrowed-or-owned representation. Deref to the inner value when present.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/reflect/wrapper/optional_cow.rs` | OptionalCow definition and trait implementations |
| source | `src/reflect/wrapper.rs` | Wrapper module declaration |
| test | `tests/inc/fundamental/fields_test.rs` | Exercises OptionalCow through Fields trait |
| doc | [`docs/api/002_fields_api.md`](../api/002_fields_api.md) | Fields operations using OptionalCow |
| doc | [`docs/feature/002_fields_iteration.md`](../feature/002_fields_iteration.md) | Fields iteration feature scope |
