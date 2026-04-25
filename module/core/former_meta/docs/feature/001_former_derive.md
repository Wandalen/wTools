# Feature: Former Derive Macro

### Scope

- **Purpose**: Describe the builder pattern generation capability that `#[derive(Former)]` provides.
- **Responsibility**: Document the macro's behavioral scope — what it generates and how it dispatches.
- **In Scope**: Struct former generation, enum former generation, and field-level attribute control.
- **Out of Scope**: The public macro interface and attribute names — see `api/001_derive_api.md`.

### Design

`former_meta` implements the `#[derive(Former)]` procedural macro, which generates a
complete builder pattern infrastructure for annotated types. The macro produces a former
type, a storage type, and associated trait implementations that allow consumers to
construct values through a fluent interface.

The macro handles two top-level cases: struct types and enum types. For structs, it
generates a former with field setters and a `form()` method that constructs the final
value. For enums, it dispatches to per-variant handlers that generate variant-specific
constructors and, where applicable, sub-formers for variant fields.

Field behavior is controlled through field-level attributes. Scalar setters, subform
scalar setters, subform collection setters, and subform entry setters each generate
different setter methods with different subforming semantics. The `#[former_ignore]`
attribute suppresses setter generation for a field entirely.

The `derive_former` feature flag gates this entire capability. When disabled, the macro
entry point is absent and no builder code is generated.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `../api/001_derive_api.md` | Public macro interface — entry point and all attributes |
| doc | `../invariant/001_proc_macro_separation.md` | Why impl lives here not in former |
| doc | `../invariant/002_feature_flag_gating.md` | derive_former feature flag contract |
| doc | `../integration/003_former.md` | Consumer facade that surfaces this macro |
