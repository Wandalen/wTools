# Feature Test Spec Doc Entity

### Scope

- **Purpose**: Provide test coverage specifications for all feature doc instances.
- **Responsibility**: Registry and overview of all feature test spec instances.
- **In Scope**: FT-N / FC-N feature test cases in Given/When/Then format for all 7 feature elements; minimum 4 cases per spec; behavioral contracts that must hold across all supported use cases.
- **Out of Scope**: Algorithm correctness cases (see `../algorithm/`), invariant enforcement cases (see `../invariant/`), manual test procedures (see `tests/manual/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Table Formatting](001_table_formatting.md) | Feature spec for table formatting capabilities | ✅ |
| 002 | [Word Wrap](002_word_wrap.md) | Feature spec for word wrapping configuration and behavior | ✅ |
| 003 | [Unified Format Interface](003_unified_format_interface.md) | Feature spec for Format trait and TableView interchange | ✅ |
| 004 | [Color Themes](004_color_themes.md) | Feature spec for predefined and custom color themes | ✅ |
| 005 | [Auto-Fit](005_auto_fit.md) | Feature spec for terminal-aware auto-wrapping and column folding | ✅ |
| 006 | [API Cleanup v0.3.0](006_api_cleanup_v030.md) | Feature spec for v0.3.0 breaking-change boundary validation | ✅ |
| 007 | [Table Heading](007_table_caption.md) | Feature spec for titled rule heading above the table | 9 ✅ |
