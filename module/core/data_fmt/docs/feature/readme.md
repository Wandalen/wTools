# Feature Doc Entity

### Scope

- **Purpose**: Document what data_fmt capabilities do and how to use them.
- **Responsibility**: Registry and overview of all feature doc instances.
- **In Scope**: Table formatting, word wrap, unified format interface, color themes, auto-fit, API cleanup v0.3.0.
- **Out of Scope**: API signatures (see `api/`), variant output examples (see `variant/`).

### Infrastructure

| File | Responsibility |
|------|----------------|
| `procedure.md` | Operational procedure for creating and updating feature doc instances |

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Table Formatting](001_table_formatting.md) | Table styles, multiline cells, truncation, borders, sub-row details | ✅ |
| 002 | [Word Wrap](002_word_wrap.md) | WrapConfig, BreakStrategy, Overflow behavior contracts | ✅ |
| 003 | [Unified Format Interface](003_unified_format_interface.md) | Format trait, TableView canonical format, feature flags | ✅ |
| 004 | [Color Themes](004_color_themes.md) | Predefined color themes and custom theme creation | ✅ |
| 005 | [Auto-Fit](005_auto_fit.md) | Terminal-aware auto-fit: cell wrapping (✅), column folding (✅) | ✅ |
| 006 | [API Cleanup v0.3.0](006_api_cleanup_v030.md) | Remove deprecated paths, add ExpandedFormatter Format impl | 🔄 |
