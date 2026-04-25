# API: External Derives

### Scope

- **Purpose**: Document the derive macros re-exported from external packages.
- **Responsibility**: Reference for macros from `derive_more`, `strum`, and `parse-display`.
- **In Scope**: Arithmetic, conversion, enum variant, enum string utilities, and display/parsing macros.
- **Out of Scope**: Workspace-internal derives — see `api/001_workspace_derives.md`.

### Abstract

External package derives cover general-purpose patterns: arithmetic operations on
numeric wrappers, enum variant utilities, display formatting, and string parsing.
Each package is an optional dependency activated by the corresponding feature flag group.

### Operations

**Arithmetic macros** — generate operator implementations for numeric wrapper types.
Covers addition, subtraction, multiplication, division, and their compound assignment forms,
plus iterator summation.

**Conversion macros** — generate type conversion implementations. Covers value conversion
to a target type, fallible conversion, iterator conversion, and struct construction.

**Enum variant macros** — generate variant inspection methods and safe variant extraction
for enum types.

**Enum string utilities** — generate string representations and string-based construction
for enums. Covers reference-to-string conversion, display formatting, iterator over variants,
and perfect-hash-based lookups.

**Display and parsing macros** — generate display formatting and string parsing implementations
driven by a format pattern string. Applicable to both structs and enums.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `../feature/001_aggregate_facade.md` | Aggregate facade context for these macros |
| doc | `001_workspace_derives.md` | Workspace-internal derives available in the same facade |

### Sources

| File | Notes |
|------|-------|
| [../../spec.md](../../spec.md) | Public API: External Derives sections; spec.md has been deleted — Sources entry retained as migration record. |
