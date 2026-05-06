# Integration: macro_tools

### Scope

- **Purpose**: Document the dependency relationship with `macro_tools`.
- **Responsibility**: Explain what this crate uses from `macro_tools` and why it is a required dependency.
- **In Scope**: Syntax parsing utilities, integration points, and compatibility requirements.
- **Out of Scope**: The `macro_tools` crate itself and its own features.

### System Description

`macro_tools` is a procedural macro development utility crate in the same
workspace. It provides helpers for parsing and manipulating Rust syntax trees,
reducing the boilerplate required when implementing procedural macros.

### Integration Points

This crate depends on `macro_tools` for syntax parsing utilities used during
macro expansion. Specifically, the implementations use its helpers to extract
struct field information, match field patterns, and produce output tokens.

The dependency is compile-time only. No runtime component is involved.

### Error Handling

Errors originating from `macro_tools` parsing utilities surface as
compile-time diagnostics at the macro call site. These are indistinguishable
from errors produced by this crate's own validation.

### Compatibility Requirements

Changes to `macro_tools`'s API may require updates to macro implementations
in this crate. Any `macro_tools` version upgrade must be validated against
all macros provided by this crate.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `../api/001_derive_api.md` | Macros implemented using macro_tools |
| doc | `../feature/001_derive_macros.md` | Behavioral description of the macros |
