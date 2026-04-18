# Trait Doc Entity

### Scope

- **Purpose:** Document interface contracts connecting input types to formatters.
- **Responsibility:** Registry and overview of all trait doc instances.
- **In Scope:** `Format`, `TableShapedFormatter`, `TableShapedView` — signatures, implementors, coverage gaps.
- **Out of Scope:** Formatter implementation details (see `feature/`), variant output (see `variant/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Format](001_format.md) | Modern unified trait accepting `&TableView` | ✅ |
| 002 | [TableShapedFormatter](002_table_shaped_formatter.md) | Legacy trait accepting `&TreeNode<String>` | ✅ |
| 003 | [TableShapedView](003_table_shaped_view.md) | Input-side trait for extracting tabular data from trees | ✅ |
