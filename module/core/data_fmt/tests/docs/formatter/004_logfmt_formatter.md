# Formatter: LogfmtFormatter

### Scope

- **Purpose**: Drive test coverage for the LogfmtFormatter output contract.
- **Responsibility**: Documents test cases for the `LogfmtFormatter` struct described in `docs/formatter/004_logfmt_formatter.md`.
- **In Scope**: Basic key=value output, special character escaping, empty value handling, Format trait dispatch, multi-row output, header-derived key names.
- **Out of Scope**: Per-variant visual details (see `tests/docs/variant/`), Logfmt parser compatibility beyond basic syntax.

### Case Index

| ID | Name | Status |
|----|------|--------|
| FM-13 | basic key-value pairs on single line | ⏳ |
| FM-14 | special characters in values are quoted | ⏳ |
| FM-15 | empty values produce key with empty quotes | ⏳ |
| FM-16 | Format trait dispatch returns well-formed string | ⏳ |
| FM-17 | multi-row input produces one logfmt line per row | ⏳ |

---

### FM-13: basic key-value pairs on single line

- **Given:** A `TableView` with headers `["level", "msg"]` and one row `["info", "started"]`.
- **When:** `LogfmtFormatter::new()` formats the view via the `Format` trait.
- **Then:** The output contains a single line with `level=info msg=started`; key-value pairs are separated by spaces; no trailing space appears before the newline.

---

### FM-14: special characters in values are quoted

- **Given:** A `TableView` with headers `["msg"]` and one row `["hello world"]` (value contains a space).
- **When:** `LogfmtFormatter::new()` formats the view.
- **Then:** The value is wrapped in double quotes: `msg="hello world"`; the quotes are part of the output; values without spaces or special characters remain unquoted.

---

### FM-15: empty values produce key with empty quotes

- **Given:** A `TableView` with headers `["tag", "value"]` and one row `["x", ""]` (second field is empty string).
- **When:** `LogfmtFormatter::new()` formats the view.
- **Then:** The empty field renders as `value=""` (key followed by equals sign and empty double quotes); the non-empty field renders normally as `tag=x`.

---

### FM-16: Format trait dispatch returns well-formed string

- **Given:** A `TableView` with headers `["a", "b"]` and rows `[["1", "2"]]`.
- **When:** The `Format::fmt` method is called on a `LogfmtFormatter` instance.
- **Then:** The return value is `Ok(String)` containing valid logfmt syntax; the result is valid UTF-8; no `FormatError` is returned.

---

### FM-17: multi-row input produces one logfmt line per row

- **Given:** A `TableView` with headers `["id"]` and rows `[["1"], ["2"], ["3"]]`.
- **When:** `LogfmtFormatter::new()` formats the view.
- **Then:** The output contains exactly 3 non-empty lines; each line is a complete logfmt record with `id=N`; lines are separated by `\n`.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/formatter/004_logfmt_formatter.md`](../../../docs/formatter/004_logfmt_formatter.md) | Source formatter doc — trait, input, single variant |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/logfmt_tests.rs`](../../logfmt_tests.rs) | LogfmtFormatter test implementation |
