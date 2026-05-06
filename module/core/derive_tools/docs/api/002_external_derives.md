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

### Error Handling

All macros in this group fail at compile time; no runtime errors are produced.

**Type constraint violation** — arithmetic and conversion macros require the target type to contain an inner type implementing the relevant standard traits. Applying them to types that do not satisfy those bounds causes a trait-bound compile error at the derive site.

**Enum-only restriction** — enum variant and string utility macros must be applied to enum types. Applying them to structs or unions produces a compile error.

**Format pattern mismatch** — display and parsing macros derive from a user-supplied format pattern attribute. A pattern referencing field names that do not exist in the type causes a compile error.

**Feature gate absent** — each external package is an optional dependency gated by a dedicated feature flag. Using a macro without its flag activated causes a missing-dependency compile error.

### Compatibility Guarantees

Re-export names are stable across patch and minor versions. If an upstream package renames a macro, a major version bump is required in this crate to avoid silent breakage for consumers.

Each external package dependency is independently versioned. Activating one package's feature flag does not affect the others.

Version constraints for each external package are declared in `Cargo.toml`. Consumers receive the version chosen by the workspace and do not control external package versions directly.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `../feature/001_aggregate_facade.md` | Aggregate facade context for these macros |
| doc | `001_workspace_derives.md` | Workspace-internal derives available in the same facade |
