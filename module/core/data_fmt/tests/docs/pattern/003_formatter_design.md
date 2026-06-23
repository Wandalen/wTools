# Pattern: Formatter Design

### Scope

- **Purpose**: Drive test coverage for the formatter trait hierarchy and TableShapedView decoupling pattern.
- **Responsibility**: Documents test cases for the formatter design in `docs/pattern/003_formatter_design.md`.
- **In Scope**: Format trait implementation coverage, TableShapedView decoupling, dual output surface (format/write_to).
- **Out of Scope**: Per-formatter configuration (see `../api/`), per-variant output (see `../variant/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| PT-1 | Format trait implemented by 9 formatters | ✅ |
| PT-2 | TableShapedView decouples formatters from tree internals | ✅ |
| PT-3 | Dual output surface available | ✅ |

---

### PT-1: Format trait implemented by 9 formatters

- **Given:** All formatter types exported from the crate.
- **When:** Attempting to call `Format::fmt(&formatter, &table_view)` on each formatter.
- **Then:** `TableFormatter`, `ExpandedFormatter`, `LogfmtFormatter`, `HtmlFormatter`, `SqlFormatter`, `JsonFormatter`, `YamlFormatter`, `TomlFormatter`, and `TextFormatter` all accept the call; `TreeFormatter` does not implement `Format`.

---

### PT-2: TableShapedView decouples formatters from tree internals

- **Given:** A `TreeNode<String>` with table-shaped structure (root → row children → column-named leaf children).
- **When:** `TableShapedView` is used to extract headers and rows from the tree.
- **Then:** The extracted headers match the column names; rows are flat string vectors; the formatter receives flat data without needing to traverse tree nodes directly.

---

### PT-3: Dual output surface available

- **Given:** A `TableFormatter` with default config and a `TableView`.
- **When:** Calling `format()` (returns String) and `write_to()` (writes to `io::Write`).
- **Then:** Both methods produce identical output content; `format()` returns `Ok(String)`; `write_to()` writes the same bytes to a buffer; no formatter lacks either output mode.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/pattern/003_formatter_design.md`](../../../docs/pattern/003_formatter_design.md) | Source pattern doc — trait hierarchy, TableShapedView decoupling, output surface |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/pattern_formatter_test.rs`](../../pattern_formatter_test.rs) | Spec tests for PT-1..PT-3 formatter design patterns |
