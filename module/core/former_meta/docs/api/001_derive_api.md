# API: Derive API

### Scope

- **Purpose**: Define the public macro interface — the entry point and all supported attribute names.
- **Responsibility**: Document every proc-macro entry point and field attribute contract in this crate.
- **In Scope**: `Former` derive applicability, struct-level attributes, and all field-level attributes.
- **Out of Scope**: Generated output structure and behavioral rationale — see `feature/001_former_derive.md`.

### Abstract

`former_meta` exposes one proc-macro entry point: `Former`. It is applied via
`#[derive(Former)]` on struct or enum items. All other names in this crate are
implementation details not intended for direct use.

### Operations

**Derive entry point**

`Former` — applicable to structs with named fields and to enums. Requires the
`derive_former` feature flag to be active.

**Struct-level attributes** (placed on the annotated item or inside `#[former(...)]`)

- `debug` — emit debug output during macro expansion
- `perform` — configure the terminal method name (default: `form`)
- `storage_fields` — additional fields added to the storage type only
- `mutator` — attach a custom mutator closure to the former

**Field-level attributes** (placed on individual struct fields)

- `#[scalar]` / `#[former(scalar)]` — generate a scalar setter for this field
- `#[subform_scalar]` — generate a subform scalar setter (delegates to a nested former)
- `#[subform_collection]` — generate a subform collection setter (delegates to a collection former)
- `#[subform_entry]` — generate a subform entry setter (delegates to an entry former)
- `#[former_ignore]` — suppress all setter generation for this field
- `#[arg_for_constructor]` — mark field as a required constructor argument

**Enum variant attributes** (placed on enum variants)

Variant dispatch is automatic. Unit variants, tuple single-field variants, and named-field
variants each route to specialized handlers. No explicit attribute is required to select
a handler.

### Error Handling

All errors surface as compile-time diagnostics at the macro call site. Span information
points to the annotated item or offending attribute. Errors from dependency crates
(`macro_tools`, `former_types`) are indistinguishable from errors produced by this crate.

### Compatibility Guarantees

The macro names and attribute names listed above constitute the public contract. Changes
to these names or to their semantics are breaking changes from the perspective of
`former` consumers and must be coordinated with a version bump in `former`.

`standalone_constructors` is a supported attribute (controls which constructors are
emitted for standalone use). Its contract is stable once `derive_former` is enabled.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `../feature/001_former_derive.md` | Behavioral description of what the macro generates |
| doc | `../invariant/002_feature_flag_gating.md` | derive_former feature flag gates this API |
| doc | `../integration/003_former.md` | Consumer facade that re-exports this macro |
