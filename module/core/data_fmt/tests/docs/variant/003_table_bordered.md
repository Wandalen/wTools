# Variant: Table Bordered

### Scope

- **Purpose**: Drive test coverage for the bordered table output variant.
- **Responsibility**: Documents test cases for the bordered variant in `docs/variant/003_table_bordered.md`.
- **In Scope**: ASCII pipe borders, grid-style header separator, outer border lines, column pipe separators.
- **Out of Scope**: Formatter internals (see `../formatter/`), attribute schema (see `../data_structure/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| VT-1 | output has pipe column separators | ✅ |
| VT-2 | ASCII grid header separator present | ✅ |
| VT-3 | outer border lines surround the table | ✅ |
| VT-4 | empty table produces bordered header only | ✅ |

---

### VT-1: output has pipe column separators

- **Given:** A `TableView` with headers `["Name", "Age"]` and rows `[["Alice", "30"]]`.
- **When:** Formatted with `TableConfig::bordered()`.
- **Then:** Every data line contains `|` characters separating columns; the header row also uses `|` separators.

---

### VT-2: ASCII grid header separator present

- **Given:** A `TableView` with headers `["A", "B"]` and one row.
- **When:** Formatted with `TableConfig::bordered()`.
- **Then:** A horizontal rule of `+` and `-` characters appears between the header and data rows; the rule spans the full table width.

---

### VT-3: outer border lines surround the table

- **Given:** A `TableView` with headers `["X"]` and one row `["1"]`.
- **When:** Formatted with `TableConfig::bordered()`.
- **Then:** The first and last output lines are horizontal border rules; the output is fully enclosed in an ASCII grid frame.

---

### VT-4: empty table produces bordered header only

- **Given:** A `TableView` with headers `["Col"]` and zero data rows.
- **When:** Formatted with `TableConfig::bordered()`.
- **Then:** Output contains the bordered header row with outer borders; no data rows; the table frame is complete even without data.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/variant/003_table_bordered.md`](../../../docs/variant/003_table_bordered.md) | Source variant doc — bordered preset attributes and example |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_styles_presets.rs`](../../table_styles_presets.rs) | Preset configuration and output tests |
