# Algorithm Doc Entity

### Scope

- **Purpose**: Document non-trivial algorithms used by formatters with pseudocode and complexity.
- **Responsibility**: Registry and overview of all algorithm doc instances.
- **In Scope**: Multiline cell rendering, word wrapping, tree column alignment, budget allocation, fold detection, CLI help alignment, heading rendering.
- **Out of Scope**: Simple formatting logic, configuration details (see `api/config_types`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Multiline Cell Rendering](001_multiline_cell_rendering.md) | Two-pass measure/render algorithm for multiline table cells | ✅ |
| 002 | [Word Wrapping](002_word_wrapping.md) | Word/hard/hybrid break strategies for text wrapping | ✅ |
| 003 | [Tree Column Alignment](003_tree_column_alignment.md) | Two-pass prefix-aware column alignment for tree output | ✅ |
| 004 | [Budget Allocation](004_budget_allocation.md) | Terminal-width-aware column budget allocation and flex classification | ✅ |
| 005 | [Column Fold Detection](005_column_fold_detection.md) | Fold point detection and continuation line partitioning | ✅ |
| 006 | [CLI Help Alignment](006_cli_help_alignment.md) | Two-pass section detection and description alignment for CLI help output | ✅ |
| 007 | [Heading Rendering](007_heading_rendering.md) | Four-step titled-rule assembly with lead prefix and clamped trailing rule width | ✅ |
