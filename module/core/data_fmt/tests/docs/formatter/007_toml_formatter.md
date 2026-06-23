# Formatter: TomlFormatter

### Scope

- **Purpose**: Drive test coverage for the TomlFormatter output contract.
- **Responsibility**: Documents test cases for the `TomlFormatter` struct described in `docs/formatter/007_toml_formatter.md`.
- **In Scope**: TOML array-of-inline-tables output, special character handling, Format trait dispatch, empty data handling, toml crate dependency behavior.
- **Out of Scope**: Per-variant visual details (see `tests/docs/variant/`), toml crate internals.

### Case Index

| ID | Name | Status |
|----|------|--------|
| FM-27 | standard output produces TOML array of inline tables | ⏳ |
| FM-28 | special characters are TOML-escaped | ⏳ |
| FM-29 | Format trait dispatch returns well-formed string | ⏳ |
| FM-30 | empty data produces empty TOML array | ⏳ |
| FM-31 | multi-row input produces one inline table per row | ⏳ |

---

### FM-27: standard output produces TOML array of inline tables

- **Given:** A `TableView` with headers `["name", "age"]` and one row `["Alice", "30"]`.
- **When:** `TomlFormatter::new()` formats the view via the `Format` trait.
- **Then:** The output is a valid TOML document; the data appears as an array of inline tables; the first table contains keys `name` and `age` with their corresponding values.

---

### FM-28: special characters are TOML-escaped

- **Given:** A `TableView` with headers `["text"]` and one row containing `"line1\nline2\ttab"`.
- **When:** `TomlFormatter::new()` formats the view.
- **Then:** The newline and tab are properly escaped in the TOML output; the output is valid TOML parseable by any compliant parser.

---

### FM-29: Format trait dispatch returns well-formed string

- **Given:** A `TableView` with headers `["a"]` and rows `[["1"]]`.
- **When:** The `Format::fmt` method is called on a `TomlFormatter` instance.
- **Then:** The return value is `Ok(String)` containing valid TOML; no `FormatError` is returned.

---

### FM-30: empty data produces empty TOML array

- **Given:** A `TableView` with headers `["col"]` and zero data rows.
- **When:** `TomlFormatter::new()` formats the view.
- **Then:** The output represents an empty TOML structure; no inline table elements appear.

---

### FM-31: multi-row input produces one inline table per row

- **Given:** A `TableView` with headers `["k"]` and rows `[["v1"], ["v2"], ["v3"]]`.
- **When:** `TomlFormatter::new()` formats the view.
- **Then:** The output contains exactly 3 inline tables in a TOML array; each has key `k` with the corresponding value.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/formatter/007_toml_formatter.md`](../../../docs/formatter/007_toml_formatter.md) | Source formatter doc — trait, serde dependency, single variant |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/toml_tests.rs`](../../toml_tests.rs) | TomlFormatter test implementation |
