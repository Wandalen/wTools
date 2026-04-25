# API: Field Macros API

### Scope

- **Purpose**: Provide macros for extracting and formatting individual struct fields with automatic key naming.
- **Responsibility**: Documents the public interface for field formatting macros — available macros, key extraction behavior, and compatibility policy.
- **In Scope**: The field macro, the custom-key field macro, and the key derivation rules.
- **Out of Scope**: The fallback conversion mechanism used internally (→ api/001), table-level formatting (→ api/003).

### Abstract

The field macros format individual struct fields as key-value pairs, deriving the display key from the field expression path and formatting the value with a configurable strategy chain. Two variants exist: one that derives the key automatically and one that accepts an explicit key name.

### Operations

**Field macro (automatic key)**: Accepts a field expression and a strategy chain (two or three formatters). Derives the display key from the last segment of the expression path. Returns a formatted key-value pair as a string. Example: referencing `record.age` produces key `"age"`.

**Field macro (custom key)**: Accepts a field expression, an explicit key name, and a strategy chain. Uses the provided key name instead of the derived one. Useful when the field name is not descriptive enough or when a different display name is required.

Both macros are declarative and complete at compile time. Strategy chain rules are identical to the fallback conversion API (see api/001).

### Error Handling

No runtime errors. Type errors at compilation if no strategy in the chain is applicable to the field's type.

### Compatibility Guarantees

Both macros are stable. Key derivation from expression paths is stable. Strategy chain interface is shared with api/001 and is stable.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/format.rs` | Field macro implementations: _field!, _field_with_key! |
| test | `tests/inc/to_string_with_fallback_test.rs` | Field macro test suite |
| test | `tests/inc/to_string_example.rs` | Usage examples |
| doc | `docs/feature/003_field_formatting_macros.md` | Feature description |
| doc | `docs/api/001_fallback_conversion_api.md` | Fallback conversion API used internally |
| doc | `docs/invariant/004_declarative_macros_only.md` | Declarative macro constraint |
