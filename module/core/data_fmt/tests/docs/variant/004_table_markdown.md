# Variant: Table Markdown

### Scope

- **Purpose**: Drive test coverage for the Markdown table output variant.
- **Responsibility**: Documents test cases for the markdown variant in `docs/variant/004_table_markdown.md`.
- **In Scope**: Pipe-delimited rows, Markdown separator line, GFM table syntax compliance, machine parseability.
- **Out of Scope**: Formatter internals (see `../formatter/`), attribute schema (see `../data_structure/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| VT-1 | every line starts and ends with pipe | ✅ |
| VT-2 | Markdown separator line between header and data | ✅ |
| VT-3 | output is valid GFM table syntax | ✅ |
| VT-4 | empty table produces header-only Markdown | ✅ |

---

### VT-1: every line starts and ends with pipe

- **Given:** A `TableView` with headers `["Name", "Age"]` and rows `[["Alice", "30"]]`.
- **When:** Formatted with `TableConfig::markdown()`.
- **Then:** Every non-separator line starts with `|` and ends with `|`; columns are pipe-delimited.

---

### VT-2: Markdown separator line between header and data

- **Given:** A `TableView` with headers `["A", "B"]` and one row.
- **When:** Formatted with `TableConfig::markdown()`.
- **Then:** A separator line matching `|---|---|` pattern (pipes and dashes) appears between header and data; each column has at least 3 dashes.

---

### VT-3: output is valid GFM table syntax

- **Given:** A `TableView` with headers `["key", "val"]` and rows `[["a", "b"], ["c", "d"]]`.
- **When:** Formatted with `TableConfig::markdown()`.
- **Then:** The output is parseable as a GitHub Flavored Markdown table; column count is consistent across all lines; no ASCII grid characters (`+`) appear.

---

### VT-4: empty table produces header-only Markdown

- **Given:** A `TableView` with headers `["Col"]` and zero data rows.
- **When:** Formatted with `TableConfig::markdown()`.
- **Then:** Output contains the header row and separator line; no data rows; the output is still valid Markdown table syntax.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/variant/004_table_markdown.md`](../../../docs/variant/004_table_markdown.md) | Source variant doc — markdown preset attributes and example |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_styles_presets.rs`](../../table_styles_presets.rs) | Preset configuration and output tests |
| [`tests/variant_004_table_markdown_test.rs`](../../variant_004_table_markdown_test.rs) | Spec tests for VT-1..VT-4 — markdown variant |
