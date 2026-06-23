# Variant: Table Unicode Box

### Scope

- **Purpose**: Drive test coverage for the Unicode box-drawing table output variant.
- **Responsibility**: Documents test cases for the unicode_box variant in `docs/variant/006_table_unicode_box.md`.
- **In Scope**: Unicode box-drawing border chars, │ column separator, Unicode charset requirement, rounded or sharp corners.
- **Out of Scope**: Formatter internals (see `../formatter/`), attribute schema (see `../data_structure/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| VT-1 | output uses Unicode box-drawing characters | ✅ |
| VT-2 | column separator is │ (U+2502) | ✅ |
| VT-3 | header separator uses Unicode horizontal lines | ✅ |
| VT-4 | empty table produces box-drawing header only | ✅ |

---

### VT-1: output uses Unicode box-drawing characters

- **Given:** A `TableView` with headers `["Name", "Age"]` and rows `[["Alice", "30"]]`.
- **When:** Formatted with `TableConfig::unicode_box()`.
- **Then:** The output contains characters from the Unicode Box Drawing block (U+2500..U+257F); no ASCII `+`, `|`, or `-` are used for borders.

---

### VT-2: column separator is │ (U+2502)

- **Given:** A `TableView` with headers `["A", "B"]` and one row.
- **When:** Formatted with `TableConfig::unicode_box()`.
- **Then:** Columns are separated by `│` (U+2502) on data and header lines; the ASCII pipe `|` does not appear.

---

### VT-3: header separator uses Unicode horizontal lines

- **Given:** A `TableView` with headers `["X"]` and one row.
- **When:** Formatted with `TableConfig::unicode_box()`.
- **Then:** The separator between header and data uses `─` (U+2500) with corner/junction characters from the Box Drawing block; ASCII dashes are not used.

---

### VT-4: empty table produces box-drawing header only

- **Given:** A `TableView` with headers `["Col"]` and zero data rows.
- **When:** Formatted with `TableConfig::unicode_box()`.
- **Then:** Output contains the header enclosed in box-drawing characters; the box frame is complete even without data rows.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/variant/006_table_unicode_box.md`](../../../docs/variant/006_table_unicode_box.md) | Source variant doc — unicode_box preset attributes and example |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_styles_presets.rs`](../../table_styles_presets.rs) | Preset configuration and output tests |
