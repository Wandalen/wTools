# Integration: macro_tools

### Scope

- **Purpose**: Document the compile-time dependency on `macro_tools` for syntax parsing.
- **Responsibility**: Describe the integration points, error handling, and compatibility requirements for `macro_tools`.
- **In Scope**: What this crate uses from `macro_tools` and why it is needed.
- **Out of Scope**: The `macro_tools` crate itself — see that crate's own documentation.

### System Description

`macro_tools` is a procedural macro development utility crate in the same workspace. It
provides helpers for parsing and manipulating Rust syntax trees, reducing the boilerplate
required when implementing procedural macros.

### Integration Points

This crate depends on `macro_tools` for syntax parsing utilities used during macro
expansion. The implementations use its helpers to extract struct field information, enum
variant structure, match attribute patterns, and produce output token streams.

The dependency is compile-time only. No runtime component is involved. `macro_tools` is
listed as an `enabled`-feature dependency in `Cargo.toml`.

### Error Handling

Errors originating from `macro_tools` parsing utilities surface as compile-time
diagnostics at the macro call site. These are indistinguishable from errors produced by
this crate's own validation logic.

### Compatibility Requirements

Changes to `macro_tools`'s API may require updates to macro implementations in this
crate. Any `macro_tools` version upgrade must be validated against all macros provided
by this crate.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `../api/001_derive_api.md` | Macros implemented using macro_tools |
| doc | `../feature/001_former_derive.md` | Behavioral description of the macros |
| config | `../../Cargo.toml` | Declares macro_tools as an enabled-feature dependency |
