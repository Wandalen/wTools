# Variant: Table Minimal

### Scope

- **Purpose**: Drive test coverage for the minimal table output variant.
- **Responsibility**: Documents test cases for the minimal variant in `docs/variant/002_table_minimal.md`.
- **In Scope**: Space-separated output with no header separator, no borders, ASCII charset, compact rendering.
- **Out of Scope**: Formatter internals (see `../formatter/`), attribute schema (see `../data_structure/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| VT-1 | output has no borders and no header separator | ✅ |
| VT-2 | column separator is double space | ✅ |
| VT-3 | no separator line between header and data | ✅ |
| VT-4 | empty table produces minimal output | ✅ |

---

### VT-1: output has no borders and no header separator

- **Given:** A `TableView` with headers `["Name", "Age"]` and rows `[["Alice", "30"]]`.
- **When:** Formatted with `TableConfig::minimal()`.
- **Then:** No `|`, `+`, `-` separator line, or box-drawing characters appear; output is only header row and data rows.

---

### VT-2: column separator is double space

- **Given:** A `TableView` with headers `["A", "B"]` and one row `["x", "y"]`.
- **When:** Formatted with `TableConfig::minimal()`.
- **Then:** Columns are separated by at least 2 spaces; alignment is maintained across header and data rows.

---

### VT-3: no separator line between header and data

- **Given:** A `TableView` with headers `["Name"]` and one row `["Alice"]`.
- **When:** Formatted with `TableConfig::minimal()`.
- **Then:** The header row is immediately followed by the data row; no dash line or separator line exists between them; total non-empty line count is 2 (header + data).

---

### VT-4: empty table produces minimal output

- **Given:** A `TableView` with headers `["Col"]` and zero data rows.
- **When:** Formatted with `TableConfig::minimal()`.
- **Then:** Output is empty or contains only the header row; no separator lines or empty table markers appear.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/variant/002_table_minimal.md`](../../../docs/variant/002_table_minimal.md) | Source variant doc — minimal preset attributes and example |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_styles_presets.rs`](../../table_styles_presets.rs) | Preset configuration and output tests |
