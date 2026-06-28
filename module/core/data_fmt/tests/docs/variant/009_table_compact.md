# Variant: Table Compact

### Scope

- **Purpose**: Drive test coverage for the compact table output variant.
- **Responsibility**: Documents test cases for the compact variant in `docs/variant/009_table_compact.md`.
- **In Scope**: Single-space separation, no header separator, no borders, minimal output overhead.
- **Out of Scope**: Formatter internals (see `../formatter/`), attribute schema (see `../data_structure/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| VT-1 | output uses single-space column separation | ✅ |
| VT-2 | no header separator line | ✅ |
| VT-3 | no border characters present | ✅ |
| VT-4 | empty table produces minimal output | ✅ |

---

### VT-1: output uses single-space column separation

- **Given:** A `TableView` with headers `["Name", "Age"]` and rows `[["Alice", "30"]]`.
- **When:** Formatted with `TableConfig::compact()`.
- **Then:** Columns are separated by single-space padding; less whitespace than `plain()` which uses double-space.

---

### VT-2: no header separator line

- **Given:** A `TableView` with headers `["A", "B"]` and one row.
- **When:** Formatted with `TableConfig::compact()`.
- **Then:** No separator line appears between header and data rows; the header row is followed directly by data.

---

### VT-3: no border characters present

- **Given:** A `TableView` with headers `["X"]` and one row `["1"]`.
- **When:** Formatted with `TableConfig::compact()`.
- **Then:** No `|`, `+`, `-`, or box-drawing characters appear anywhere in the output; output is pure data with minimal spacing.

---

### VT-4: empty table produces minimal output

- **Given:** A `TableView` with headers `["Col"]` and zero data rows.
- **When:** Formatted with `TableConfig::compact()`.
- **Then:** Output is empty or contains only the header row; no decorative elements appear.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/variant/009_table_compact.md`](../../../docs/variant/009_table_compact.md) | Source variant doc — compact preset attributes and example |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_styles_presets.rs`](../../table_styles_presets.rs) | Preset configuration and output tests |
| [`tests/variant_009_table_compact_test.rs`](../../variant_009_table_compact_test.rs) | Spec tests for VT-1..VT-4 — compact variant |
