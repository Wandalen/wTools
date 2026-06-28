# Variant: Table Grid

### Scope

- **Purpose**: Drive test coverage for the grid table output variant.
- **Responsibility**: Documents test cases for the grid variant in `docs/variant/005_table_grid.md`.
- **In Scope**: Full ASCII grid with pipe columns, horizontal rules between all rows, grid intersections.
- **Out of Scope**: Formatter internals (see `../formatter/`), attribute schema (see `../data_structure/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| VT-1 | pipe column separators on every line | ✅ |
| VT-2 | horizontal rules between all rows | ✅ |
| VT-3 | grid intersections use + characters | ✅ |
| VT-4 | empty table produces grid header only | ✅ |

---

### VT-1: pipe column separators on every line

- **Given:** A `TableView` with headers `["Name", "Age"]` and rows `[["Alice", "30"]]`.
- **When:** Formatted with `TableConfig::grid()`.
- **Then:** Every data and header line contains `|` separators; the output has a full grid appearance.

---

### VT-2: horizontal rules between all rows

- **Given:** A `TableView` with headers `["A", "B"]` and rows `[["1", "2"], ["3", "4"]]`.
- **When:** Formatted with `TableConfig::grid()`.
- **Then:** A horizontal rule line appears between the header and data rows, and between each pair of data rows; rules span the full table width.

---

### VT-3: grid intersections use + characters

- **Given:** A `TableView` with headers `["X", "Y"]` and one row.
- **When:** Formatted with `TableConfig::grid()`.
- **Then:** Horizontal rule lines contain `+` at column boundaries; the intersection of vertical pipes and horizontal dashes is marked with `+`.

---

### VT-4: empty table produces grid header only

- **Given:** A `TableView` with headers `["Col"]` and zero data rows.
- **When:** Formatted with `TableConfig::grid()`.
- **Then:** Output contains the bordered header row with grid frame; no data rows; the grid structure is complete.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/variant/005_table_grid.md`](../../../docs/variant/005_table_grid.md) | Source variant doc — grid preset attributes and example |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_styles_presets.rs`](../../table_styles_presets.rs) | Preset configuration and output tests |
| [`tests/variant_005_table_grid_test.rs`](../../variant_005_table_grid_test.rs) | Spec tests for VT-1..VT-4 — grid variant |
