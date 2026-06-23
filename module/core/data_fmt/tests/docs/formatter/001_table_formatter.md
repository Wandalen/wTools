# Formatter: TableFormatter

### Scope

- **Purpose**: Drive test coverage for the TableFormatter output contract.
- **Responsibility**: Documents test cases for the `TableFormatter` struct described in `docs/formatter/001_table_formatter.md`.
- **In Scope**: Plain output rendering, bordered output rendering, Format trait dispatch, empty table handling, config preset selection, column separator emission, header row rendering.
- **Out of Scope**: Per-variant visual details (see `tests/docs/variant/`), word-wrap algorithm (see `tests/docs/algorithm/`), multiline cell splitting (see `tests/docs/algorithm/001`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| FM-1 | plain config produces unbordered output | ✅ |
| FM-2 | bordered config produces box-drawing characters | ✅ |
| FM-3 | Format trait dispatch returns well-formed string | ✅ |
| FM-4 | empty table produces header-only output | ✅ |
| FM-5 | markdown config produces pipe-delimited rows | ✅ |
| FM-6 | csv config produces comma-separated values | ✅ |
| FM-7 | with_config applies the given TableConfig preset | ✅ |

---

### FM-1: plain config produces unbordered output

- **Given:** A `TableView` with headers `["name", "age"]` and one row `["Alice", "30"]`.
- **When:** `TableFormatter::with_config(TableConfig::plain())` formats the view via the `Format` trait.
- **Then:** The output contains the header and data rows separated by whitespace alignment only; no `|`, `+`, or box-drawing characters appear in the output; the output ends with a trailing newline.

---

### FM-2: bordered config produces box-drawing characters

- **Given:** A `TableView` with headers `["id", "value"]` and one row `["1", "hello"]`.
- **When:** `TableFormatter::with_config(TableConfig::bordered())` formats the view.
- **Then:** The output contains `|` column separators on every data line; a horizontal rule line appears between header and data; the first and last lines are horizontal border lines.

---

### FM-3: Format trait dispatch returns well-formed string

- **Given:** A `TableView` with headers `["x"]` and rows `[["1"], ["2"]]`.
- **When:** The `Format::fmt` method is called on a `TableFormatter` instance (default config).
- **Then:** The return value is `Ok(String)` containing at least 2 data lines; the result is valid UTF-8; no `FormatError` is returned.

---

### FM-4: empty table produces header-only output

- **Given:** A `TableView` with headers `["col_a", "col_b"]` and zero data rows.
- **When:** `TableFormatter` formats the view with any config preset.
- **Then:** The output contains the header row text; no data rows appear below the header; the output is non-empty (not an empty string).

---

### FM-5: markdown config produces pipe-delimited rows

- **Given:** A `TableView` with headers `["key", "val"]` and one row `["a", "b"]`.
- **When:** `TableFormatter::with_config(TableConfig::markdown())` formats the view.
- **Then:** Every non-separator line starts and ends with `|`; a separator line of `|---|---|` (or similar dashes) appears between header and data; the output is valid Markdown table syntax.

---

### FM-6: csv config produces comma-separated values

- **Given:** A `TableView` with headers `["name", "city"]` and rows `[["Alice", "New York"], ["Bob", "Paris"]]`.
- **When:** `TableFormatter::with_config(TableConfig::csv())` formats the view.
- **Then:** The first line is `name,city`; subsequent lines contain comma-separated field values; fields containing commas are quoted; no box-drawing or alignment whitespace appears.

---

### FM-7: with_config applies the given TableConfig preset

- **Given:** Two `TableFormatter` instances: one with `TableConfig::plain()`, one with `TableConfig::unicode_box()`.
- **When:** Both format the same `TableView` with headers `["a"]` and one row `["1"]`.
- **Then:** The plain output contains no Unicode box characters; the unicode_box output contains box-drawing characters from the Unicode Box Drawing block (U+2500..U+257F); the two outputs differ in structure.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/formatter/001_table_formatter.md`](../../../docs/formatter/001_table_formatter.md) | Source formatter doc — trait, input, variant selection |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_tests.rs`](../../table_tests.rs) | TableFormatter test implementation |
