# Feature: reflect_tools Integration

### Scope

- **Purpose**: Enable generic table formatting for any type that exposes its fields through the reflection interface.
- **Responsibility**: Documents the reflect_tools integration — its role in table formatting, the re-exported interface, and all associated artifacts.
- **In Scope**: Field iteration via the field reflection interface, iterator adapter for reflection-based traversal, re-exports from reflect_tools.
- **Out of Scope**: reflect_tools internals (→ reflect_tools crate), table layout details (→ feature/002), fallback conversion (→ feature/001).

### Design

format_tools re-exports the field reflection interfaces from reflect_tools, making them available to callers without requiring a direct dependency on reflect_tools. This integration enables the table formatting engine to iterate over any struct's fields generically, without knowing the struct's concrete type at compile time.

A type implementing the field reflection interface can be formatted as a table by the engine directly — no manual field extraction is needed. The engine retrieves field names and values dynamically and feeds them into the table input structure.

This integration is optional: raw vector-based table construction is available when reflection is not needed or when the type does not implement the field reflection interface.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Re-exports field reflection and iterator interfaces from reflect_tools |
| source | `src/format/as_table.rs` | Table conversion trait for reflection-based formatting |
| test | `tests/inc/fields_test.rs` | Field reflection interface integration tests |
| test | `tests/inc/collection_test.rs` | Collection integration with reflection |
| doc | `docs/feature/002_table_formatting.md` | Table formatting that consumes reflected fields |
| doc | `docs/api/003_table_formatting_api.md` | AsTable trait API |
