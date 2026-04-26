# Feature: Field Formatting Macros

### Scope

- **Purpose**: Extract and format individual struct fields with automatic key naming and configurable fallback strategies.
- **Responsibility**: Documents the field formatting macro capability — its key extraction behavior, fallback configuration, and all associated artifacts.
- **In Scope**: Field key extraction from expression paths, primary and fallback formatter configuration, custom key name override.
- **Out of Scope**: Fallback conversion mechanism internals (→ feature/001), table layout (→ feature/002), text wrapping (→ feature/004).

### Design

The field formatting macros eliminate repetitive field extraction and formatting code in structs. A single macro invocation extracts both the field value and its name from the expression path, then formats the value using a configurable strategy chain.

Key naming is automatic: the macro derives the display key from the last segment of the field expression path. For example, referencing `person.name` produces `"name"` as the key. When the automatic name is not suitable, a custom key name can be provided explicitly.

Each macro accepts a strategy chain identical to the fallback conversion mechanism: a primary formatter and one or two fallbacks. This ensures every field produces a string regardless of which formatting interfaces the field type satisfies.

The macros are declarative (not procedural), keeping compile times low and avoiding heavyweight macro-expansion infrastructure.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/format.rs` | Primary field formatting macro definitions |
| test | `tests/inc/to_string_with_fallback_test.rs` | Field macro test suite |
| test | `tests/inc/to_string_example.rs` | Field macro usage examples |
| doc | `docs/api/002_field_macros_api.md` | Public API for field formatting macros |
| doc | `docs/feature/001_fallback_string_conversion.md` | Fallback chain used by field macros |
| doc | `docs/invariant/003_synchronous_only.md` | Synchronous execution constraint |
| doc | `docs/invariant/004_declarative_macros_only.md` | Declarative macro constraint |
