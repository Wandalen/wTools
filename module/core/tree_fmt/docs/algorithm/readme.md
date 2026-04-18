# Algorithm Doc Entity

### Scope

- **Purpose:** Document non-trivial algorithms used by formatters with pseudocode and complexity.
- **Responsibility:** Registry and overview of all algorithm doc instances.
- **In Scope:** Multiline cell rendering, word wrapping, tree column alignment.
- **Out of Scope:** Simple formatting logic, configuration details (see `api/config_types`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Multiline Cell Rendering](001_multiline_cell_rendering.md) | Two-pass measure/render algorithm for multiline table cells | ✅ |
| 002 | [Word Wrapping](002_word_wrapping.md) | Word/hard/hybrid break strategies for text wrapping | ✅ |
| 003 | [Tree Column Alignment](003_tree_column_alignment.md) | Two-pass prefix-aware column alignment for tree output | ✅ |
