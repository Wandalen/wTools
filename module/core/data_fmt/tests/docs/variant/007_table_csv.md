# Variant: Table CSV

### Scope

- **Purpose**: Drive test coverage for the CSV table output variant.
- **Responsibility**: Documents test cases for the csv variant in `docs/variant/007_table_csv.md`.
- **In Scope**: Comma-separated values, no borders, RFC 4180 compliance, quoting for fields with commas.
- **Out of Scope**: Formatter internals (see `../formatter/`), attribute schema (see `../data_structure/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| VT-1 | output is comma-separated with no borders | ✅ |
| VT-2 | first line is header row | ✅ |
| VT-3 | fields containing commas are quoted | ✅ |
| VT-4 | empty table produces header-only CSV | ✅ |

---

### VT-1: output is comma-separated with no borders

- **Given:** A `TableView` with headers `["Name", "Age"]` and rows `[["Alice", "30"]]`.
- **When:** Formatted with `TableConfig::csv()`.
- **Then:** Fields are separated by commas; no `|`, `+`, `-`, or alignment whitespace appears; output is machine-parseable.

---

### VT-2: first line is header row

- **Given:** A `TableView` with headers `["key", "val"]` and one row `["a", "b"]`.
- **When:** Formatted with `TableConfig::csv()`.
- **Then:** The first line is `key,val`; the second line is `a,b`; no separator line between header and data.

---

### VT-3: fields containing commas are quoted

- **Given:** A `TableView` with headers `["Name", "City"]` and one row `["Alice", "New York, NY"]`.
- **When:** Formatted with `TableConfig::csv()`.
- **Then:** The city field is enclosed in double quotes: `"New York, NY"`; fields without commas are unquoted.

---

### VT-4: empty table produces header-only CSV

- **Given:** A `TableView` with headers `["Col"]` and zero data rows.
- **When:** Formatted with `TableConfig::csv()`.
- **Then:** Output contains the header line `Col` and no data lines; output is valid CSV.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/variant/007_table_csv.md`](../../../docs/variant/007_table_csv.md) | Source variant doc — csv preset attributes and example |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_styles_presets.rs`](../../table_styles_presets.rs) | Preset configuration and output tests |
| [`tests/variant_007_table_csv_test.rs`](../../variant_007_table_csv_test.rs) | Spec tests for VT-1..VT-4 — csv variant |
