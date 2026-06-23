# Variant: Table Plain

### Scope

- **Purpose**: Drive test coverage for the plain table output variant.
- **Responsibility**: Documents test cases for the plain variant in `docs/variant/001_table_plain.md`.
- **In Scope**: Space-separated output, dash header separator, no borders, ASCII charset, empty table behavior.
- **Out of Scope**: Formatter internals (see `../formatter/`), attribute schema (see `../data_structure/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| VT-1 | output is space-separated with no borders | ⏳ |
| VT-2 | column separator is double space | ⏳ |
| VT-3 | header separator is dashes | ⏳ |
| VT-4 | empty table produces minimal output | ⏳ |

---

### VT-1: output is space-separated with no borders

- **Given:** A `TableView` with headers `["Name", "Age"]` and rows `[["Alice", "30"], ["Bob", "25"]]`.
- **When:** Formatted with `TableConfig::plain()`.
- **Then:** No `|`, `+`, or box-drawing characters appear; columns are aligned with space padding; output is pure ASCII.

---

### VT-2: column separator is double space

- **Given:** A `TableView` with headers `["A", "B"]` and one row `["x", "y"]`.
- **When:** Formatted with `TableConfig::plain()`.
- **Then:** Columns are separated by at least 2 spaces; no pipe or tab characters appear between columns.

---

### VT-3: header separator is dashes

- **Given:** A `TableView` with headers `["Name", "Age"]` and one row.
- **When:** Formatted with `TableConfig::plain()`.
- **Then:** A line of dash characters (`-`) appears between the header row and the first data row; dash count matches column widths.

---

### VT-4: empty table produces minimal output

- **Given:** A `TableView` with headers `["Col"]` and zero data rows.
- **When:** Formatted with `TableConfig::plain()`.
- **Then:** Output is empty or contains only the header; no data rows appear; no trailing separators.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/variant/001_table_plain.md`](../../../docs/variant/001_table_plain.md) | Source variant doc — plain preset attributes and example |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_styles_presets.rs`](../../table_styles_presets.rs) | Preset configuration and output tests |
