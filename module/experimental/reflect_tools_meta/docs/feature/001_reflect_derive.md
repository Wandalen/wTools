# Feature: Reflect Derive

### Scope

- **Purpose**: Enable struct types to opt into the Reflect trait via a single derive attribute.
- **Responsibility**: Document the reflect_derive feature scope, design decisions, and all artifact locations.
- **In Scope**: Derive availability, supported struct forms, `#[debug]` attribute, feature-gate requirements.
- **Out of Scope**: Runtime reflection behavior (→ `reflect_tools` frontend); generated trait definition (→ external).

### Design

The `reflect_derive` feature gates the `#[derive(Reflect)]` macro. Both `enabled` and `reflect_derive` must be active for the macro to be compiled into the crate.

Accepts all struct forms — see `api/001_reflect_derive.md` Operations section for the complete list of supported input varieties.

An optional `#[debug]` attribute on the struct triggers diagnostic output during code generation, reporting the struct name and derive context without altering the generated result.

Current implementation is a stub — returns empty code generation output for all inputs.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Proc-macro export and feature-gating |
| source | `src/implementation/reflect.rs` | Derive logic and debug attribute handling |
| test | `tests/smoke_test.rs` | Crate compilation and linkage health |
| test | `tests/reflect_derive_test.rs` | Compilation tests for all supported struct forms |
| test | `tests/corner_cases_test.rs` | Edge case tests — generics, bounds, attributes, visibility |
| doc | `docs/api/001_reflect_derive.md` | API contract, error conditions, compatibility |
| doc | `docs/invariant/001_struct_only_input.md` | Struct-only input constraint |
| config | `Cargo.toml` | Feature: `reflect_derive` |
