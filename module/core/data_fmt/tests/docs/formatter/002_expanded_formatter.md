# Formatter: ExpandedFormatter

### Scope

- **Purpose**: Drive test coverage for the ExpandedFormatter output contract.
- **Responsibility**: Documents test cases for the `ExpandedFormatter` struct described in `docs/formatter/002_expanded_formatter.md`.
- **In Scope**: Postgres-style vertical output, property-style output, Format trait dispatch, empty data handling, record separator emission, field label alignment.
- **Out of Scope**: Per-variant visual details (see `tests/docs/variant/`), tabular input model (see `tests/docs/input_model/001`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| FM-5 | postgres style renders record header and fields | ✅ |
| FM-6 | property style renders dotted key-value lines | ✅ |
| FM-7 | Format trait dispatch returns well-formed string | ✅ |
| FM-8 | empty data produces no records | ✅ |
| FM-9 | multi-row input produces numbered record separators | ✅ |

---

### FM-5: postgres style renders record header and fields

- **Given:** A `TableView` with headers `["name", "age"]` and one row `["Alice", "30"]`.
- **When:** `ExpandedFormatter::with_config(ExpandedConfig::postgres_style())` formats the view via the `Format` trait.
- **Then:** The output contains a record header line (e.g., `-[ RECORD 1 ]-`); each field appears on its own line as `field_name | value`; the field name column is left-aligned and padded to uniform width.

---

### FM-6: property style renders dotted key-value lines

- **Given:** A `TableView` with headers `["host", "port"]` and one row `["localhost", "8080"]`.
- **When:** `ExpandedFormatter::with_config(ExpandedConfig::property_style())` formats the view.
- **Then:** Each field appears as `header_name: value` (colon-separated); no record header banner is emitted; lines are separated by newlines without blank-line gaps between fields of the same record.

---

### FM-7: Format trait dispatch returns well-formed string

- **Given:** A `TableView` with headers `["k"]` and rows `[["v1"], ["v2"]]`.
- **When:** The `Format::fmt` method is called on an `ExpandedFormatter` instance (postgres_style config).
- **Then:** The return value is `Ok(String)` containing output for both rows; the result is valid UTF-8; no `FormatError` is returned.

---

### FM-8: empty data produces no records

- **Given:** A `TableView` with headers `["a", "b"]` and zero data rows.
- **When:** `ExpandedFormatter` formats the view with any config preset.
- **Then:** The output contains no record headers and no field lines; the result is either an empty string or contains only whitespace.

---

### FM-9: multi-row input produces numbered record separators

- **Given:** A `TableView` with headers `["x"]` and rows `[["1"], ["2"], ["3"]]`.
- **When:** `ExpandedFormatter::with_config(ExpandedConfig::postgres_style())` formats the view.
- **Then:** The output contains three record header lines numbered sequentially (RECORD 1, RECORD 2, RECORD 3); each record section contains exactly one field line; record sections are visually separated.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/formatter/002_expanded_formatter.md`](../../../docs/formatter/002_expanded_formatter.md) | Source formatter doc — trait, input, variant selection |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/expanded_tests.rs`](../../expanded_tests.rs) | ExpandedFormatter test implementation |
