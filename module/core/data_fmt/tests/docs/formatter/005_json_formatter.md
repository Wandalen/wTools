# Formatter: JsonFormatter

### Scope

- **Purpose**: Drive test coverage for the JsonFormatter output contract.
- **Responsibility**: Documents test cases for the `JsonFormatter` struct described in `docs/formatter/005_json_formatter.md`.
- **In Scope**: Pretty-printed output, compact single-line output, special character escaping in JSON strings, Format trait dispatch, empty data handling, serde_json dependency behavior.
- **Out of Scope**: Per-variant visual details (see `tests/docs/variant/`), serde_json internals.

### Case Index

| ID | Name | Status |
|----|------|--------|
| FM-17 | pretty print produces indented JSON array | ⏳ |
| FM-18 | compact mode produces single-line JSON | ⏳ |
| FM-19 | special characters are JSON-escaped | ⏳ |
| FM-20 | Format trait dispatch returns well-formed string | ⏳ |
| FM-21 | empty data produces empty JSON array | ⏳ |

---

### FM-17: pretty print produces indented JSON array

- **Given:** A `TableView` with headers `["name", "age"]` and one row `["Alice", "30"]`.
- **When:** `JsonFormatter::pretty()` formats the view via the `Format` trait.
- **Then:** The output is a JSON array containing one object `{"name": "Alice", "age": "30"}`; the output contains newlines and indentation (multi-line); the JSON is valid (parseable by any JSON parser).

---

### FM-18: compact mode produces single-line JSON

- **Given:** A `TableView` with headers `["k"]` and rows `[["v1"], ["v2"]]`.
- **When:** `JsonFormatter::compact()` formats the view.
- **Then:** The entire output fits on a single line (no intermediate newlines within the JSON structure); the output is a valid JSON array of two objects; field values are string type.

---

### FM-19: special characters are JSON-escaped

- **Given:** A `TableView` with headers `["text"]` and one row containing a value with a double quote, backslash, and newline: `"line1\nline2 \"quoted\" path\\dir"`.
- **When:** `JsonFormatter::pretty()` formats the view.
- **Then:** The double quote is escaped as `\"`; the backslash is escaped as `\\`; the newline is escaped as `\n`; the output is valid JSON.

---

### FM-20: Format trait dispatch returns well-formed string

- **Given:** A `TableView` with headers `["a"]` and rows `[["1"]]`.
- **When:** The `Format::fmt` method is called on a `JsonFormatter` instance (pretty mode).
- **Then:** The return value is `Ok(String)` containing a valid JSON array; no `FormatError` is returned; the JSON contains one object with key `"a"` and string value `"1"`.

---

### FM-21: empty data produces empty JSON array

- **Given:** A `TableView` with headers `["col"]` and zero data rows.
- **When:** `JsonFormatter::pretty()` formats the view.
- **Then:** The output is the JSON empty array `[]`; no object elements appear; the output is valid JSON.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/formatter/005_json_formatter.md`](../../../docs/formatter/005_json_formatter.md) | Source formatter doc — trait, serde dependency, pretty/compact variants |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/json_tests.rs`](../../json_tests.rs) | JsonFormatter test implementation |
