# Feature Doc Entity

### Scope

- **Purpose**: Navigate all artifacts for user-facing formatting capabilities in format_tools.
- **Responsibility**: Index of feature doc instances, each pointing to source, test, and design artifacts for one capability.
- **In Scope**: User-facing formatting behaviors implemented in format_tools.
- **Out of Scope**: Implementation-level API contracts (→ api/), structural constraints (→ invariant/), data layouts (→ data_structure/).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Fallback String Conversion](001_fallback_string_conversion.md) | Convert values to strings via prioritized strategy chain | ✅ |
| 002 | [Table Formatting](002_table_formatting.md) | Format structured data as grid, records, or key-value layout | ✅ |
| 003 | [Field Formatting Macros](003_field_formatting_macros.md) | Extract and format struct fields with automatic key naming | ✅ |
| 004 | [Text Manipulation](004_text_manipulation.md) | Wrap text and render markdown math expressions | ✅ |
| 005 | [reflect_tools Integration](005_reflect_integration.md) | Iterate struct fields at runtime via reflection | ✅ |
