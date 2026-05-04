# API Doc Entity

### Scope

- **Purpose**: Document the public programmatic interfaces exposed by format_tools to external callers.
- **Responsibility**: Index of API doc instances, each describing one public interface group — its operations, error conditions, and compatibility guarantees.
- **In Scope**: Macros, interfaces, and type groups that form the public API surface of format_tools.
- **Out of Scope**: Internal implementation details (→ data_structure/), design patterns (→ pattern/), behavioral requirements (→ feature/).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Fallback Conversion API](001_fallback_conversion_api.md) | Macro and interface for primary/fallback string conversion | ✅ |
| 002 | [Field Macros API](002_field_macros_api.md) | Macros for struct field extraction and formatting | ✅ |
| 003 | [Table Formatting API](003_table_formatting_api.md) | Interfaces and utilities for table output formatting | ✅ |
| 004 | [Wrapper Types API](004_wrapper_types_api.md) | Data-free strategy markers for formatter dispatch | ✅ |
