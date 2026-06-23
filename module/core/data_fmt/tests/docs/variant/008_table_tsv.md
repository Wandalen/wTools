# Variant: Table TSV

### Scope

- **Purpose**: Drive test coverage for the TSV table output variant.
- **Responsibility**: Documents test cases for the tsv variant in `docs/variant/008_table_tsv.md`.
- **In Scope**: Tab-separated values, no borders, no header separator, machine parseability.
- **Out of Scope**: Formatter internals (see `../formatter/`), attribute schema (see `../data_structure/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| VT-1 | output is tab-separated with no borders | ✅ |
| VT-2 | column separator is tab character | ✅ |
| VT-3 | no header separator line | ✅ |
| VT-4 | empty table produces header-only TSV | ✅ |

---

### VT-1: output is tab-separated with no borders

- **Given:** A `TableView` with headers `["Name", "Age"]` and rows `[["Alice", "30"]]`.
- **When:** Formatted with `TableConfig::tsv()`.
- **Then:** Fields are separated by tab characters (`\t`); no `|`, `+`, `-`, or alignment spaces appear between fields.

---

### VT-2: column separator is tab character

- **Given:** A `TableView` with headers `["A", "B"]` and one row `["x", "y"]`.
- **When:** Formatted with `TableConfig::tsv()`.
- **Then:** Exactly one tab character appears between each pair of adjacent fields on every line.

---

### VT-3: no header separator line

- **Given:** A `TableView` with headers `["key", "val"]` and one row.
- **When:** Formatted with `TableConfig::tsv()`.
- **Then:** The header row is immediately followed by the data row; no dash, equals, or other separator line appears between them.

---

### VT-4: empty table produces header-only TSV

- **Given:** A `TableView` with headers `["Col"]` and zero data rows.
- **When:** Formatted with `TableConfig::tsv()`.
- **Then:** Output contains the header line with column names and no data lines; output is valid TSV.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/variant/008_table_tsv.md`](../../../docs/variant/008_table_tsv.md) | Source variant doc — tsv preset attributes and example |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_styles_presets.rs`](../../table_styles_presets.rs) | Preset configuration and output tests |
