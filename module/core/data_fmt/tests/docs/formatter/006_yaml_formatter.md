# Formatter: YamlFormatter

### Scope

- **Purpose**: Drive test coverage for the YamlFormatter output contract.
- **Responsibility**: Documents test cases for the `YamlFormatter` struct described in `docs/formatter/006_yaml_formatter.md`.
- **In Scope**: YAML sequence-of-mappings output, special character handling, Format trait dispatch, empty data handling, serde_yaml_ng dependency behavior.
- **Out of Scope**: Per-variant visual details (see `tests/docs/variant/`), serde_yaml_ng internals.

### Case Index

| ID | Name | Status |
|----|------|--------|
| FM-22 | standard output produces YAML sequence of mappings | ⏳ |
| FM-23 | special characters are YAML-safe | ⏳ |
| FM-24 | Format trait dispatch returns well-formed string | ⏳ |
| FM-25 | empty data produces empty YAML sequence | ⏳ |
| FM-26 | multi-row input produces one mapping per row | ⏳ |

---

### FM-22: standard output produces YAML sequence of mappings

- **Given:** A `TableView` with headers `["name", "age"]` and one row `["Alice", "30"]`.
- **When:** `YamlFormatter::new()` formats the view via the `Format` trait.
- **Then:** The output is a YAML document; each row appears as a mapping with keys matching the headers; the first mapping contains `name: Alice` and `age: '30'` (or `"30"`).

---

### FM-23: special characters are YAML-safe

- **Given:** A `TableView` with headers `["text"]` and one row containing `"colon: here\nnewline"`.
- **When:** `YamlFormatter::new()` formats the view.
- **Then:** The colon and newline are properly quoted or escaped in the YAML output; the output is valid YAML parseable by any compliant parser.

---

### FM-24: Format trait dispatch returns well-formed string

- **Given:** A `TableView` with headers `["a"]` and rows `[["1"]]`.
- **When:** The `Format::fmt` method is called on a `YamlFormatter` instance.
- **Then:** The return value is `Ok(String)` containing valid YAML; no `FormatError` is returned.

---

### FM-25: empty data produces empty YAML sequence

- **Given:** A `TableView` with headers `["col"]` and zero data rows.
- **When:** `YamlFormatter::new()` formats the view.
- **Then:** The output represents an empty YAML sequence (`[]` or blank); no mapping elements appear.

---

### FM-26: multi-row input produces one mapping per row

- **Given:** A `TableView` with headers `["k"]` and rows `[["v1"], ["v2"], ["v3"]]`.
- **When:** `YamlFormatter::new()` formats the view.
- **Then:** The output contains exactly 3 YAML mappings; each mapping has key `k` with the corresponding value.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/formatter/006_yaml_formatter.md`](../../../docs/formatter/006_yaml_formatter.md) | Source formatter doc — trait, serde dependency, single variant |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/yaml_tests.rs`](../../yaml_tests.rs) | YamlFormatter test implementation |
