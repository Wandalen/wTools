# Feature: Former Derive Macro

### Scope

- **Purpose**: Describe the builder pattern generation capability of the Former derive macro.
- **Responsibility**: Document the macro's behavioral scope — what it generates and how it dispatches.
- **In Scope**: Struct former generation, enum former generation, and field-level attribute control.
- **Out of Scope**: The public macro interface and attribute names — see `api/001_derive_api.md`.

### Design

This crate implements a procedural derive macro that generates a complete builder pattern
infrastructure for annotated types. The macro produces a former type, a storage type, and
associated trait implementations that allow consumers to construct values through a fluent
interface.

The macro handles two top-level cases: struct types and enum types. For structs, it
generates a former with field setters and a terminal construction method. For enums, it
dispatches to per-variant handlers that generate variant-specific constructors and, where
applicable, sub-formers for variant fields.

Field behavior is controlled through field-level attributes. Four setter strategies exist:
scalar setters, subform scalar setters, subform collection setters, and subform entry
setters — each generating methods with different subforming semantics. A dedicated
suppress attribute omits setter generation for a field entirely.

This entire capability is gated behind a dedicated feature flag. When the flag is
disabled, no macro entry point is registered and no builder code is generated.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `../../src/lib.rs` | Proc-macro crate entry point with feature-gated registration |
| source | `../../src/derive_former.rs` | Primary derive implementation dispatching struct and enum paths |
| test | `../../tests/smoke_test.rs` | Integration smoke tests for the derive macro |
| doc | `../api/001_derive_api.md` | Public macro interface — entry point and all attributes |
| doc | `../invariant/001_proc_macro_separation.md` | Why impl lives here not in former |
| doc | `../invariant/002_feature_flag_gating.md` | derive_former feature flag contract |
| doc | `../integration/003_former.md` | Consumer facade that surfaces this macro |
