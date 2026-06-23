# Formatter: TextFormatter

### Scope

- **Purpose**: Drive test coverage for the TextFormatter output contract.
- **Responsibility**: Documents test cases for the `TextFormatter` struct described in `docs/formatter/010_text_formatter.md`.
- **In Scope**: Six text variant outputs (bullets, numbered, sections, keyvalue, compact, cli_help), shared feature flag, Format trait dispatch, empty data handling.
- **Out of Scope**: Per-variant visual details (see `tests/docs/variant/`), CLI framework integration.

### Case Index

| ID | Name | Status |
|----|------|--------|
| FM-47 | bullets variant produces bullet-prefixed lines | ✅ |
| FM-48 | numbered variant produces numbered lines | ✅ |
| FM-49 | sections variant produces section-headed blocks | ✅ |
| FM-50 | keyvalue variant produces key-value pair lines | ✅ |
| FM-51 | compact variant produces minimal whitespace output | ✅ |
| FM-52 | cli_help variant produces CLI help formatted output | ✅ |
| FM-53 | Format trait dispatch returns well-formed string | ✅ |
| FM-54 | empty data produces minimal or empty output | ✅ |

---

### FM-47: bullets variant produces bullet-prefixed lines

- **Given:** A `TableView` with headers `["item", "qty"]` and two rows `[["apple", "3"], ["banana", "5"]]`.
- **When:** `TextFormatter::new(TextVariant::Bullets)` formats the view.
- **Then:** Each row appears as a bullet-prefixed line; the output contains bullet markers (e.g., `- ` or `* `); field values from headers and row data are included.

---

### FM-48: numbered variant produces numbered lines

- **Given:** A `TableView` with headers `["item"]` and rows `[["first"], ["second"], ["third"]]`.
- **When:** `TextFormatter::new(TextVariant::Numbered)` formats the view.
- **Then:** Each row is prefixed with its sequential number (e.g., `1.`, `2.`, `3.`); the numbering starts at 1.

---

### FM-49: sections variant produces section-headed blocks

- **Given:** A `TableView` with headers `["name", "desc"]` and two rows.
- **When:** `TextFormatter::new(TextVariant::Sections)` formats the view.
- **Then:** Each row appears as a distinct section with a heading; fields are rendered under each section heading.

---

### FM-50: keyvalue variant produces key-value pair lines

- **Given:** A `TableView` with headers `["host", "port"]` and one row `["localhost", "8080"]`.
- **When:** `TextFormatter::new(TextVariant::KeyValue)` formats the view.
- **Then:** The output contains `host` and `port` as keys paired with their values; each key-value pair appears on its own line.

---

### FM-51: compact variant produces minimal whitespace output

- **Given:** A `TableView` with headers `["a", "b"]` and one row `["1", "2"]`.
- **When:** `TextFormatter::new(TextVariant::Compact)` formats the view.
- **Then:** The output uses minimal whitespace between fields; no decorative borders or separators appear.

---

### FM-52: cli_help variant produces CLI help formatted output

- **Given:** A `TableView` with headers `["flag", "description"]` and two rows `[["--verbose", "Enable verbose output"], ["--quiet", "Suppress output"]]`.
- **When:** `TextFormatter::new(TextVariant::CliHelp)` formats the view.
- **Then:** The output resembles CLI help formatting; flags are left-aligned; descriptions are aligned or indented consistently.

---

### FM-53: Format trait dispatch returns well-formed string

- **Given:** A `TableView` with headers `["a"]` and rows `[["1"]]`.
- **When:** The `Format::fmt` method is called on a `TextFormatter` instance (bullets variant).
- **Then:** The return value is `Ok(String)`; no `FormatError` is returned.

---

### FM-54: empty data produces minimal or empty output

- **Given:** A `TableView` with headers `["col"]` and zero data rows.
- **When:** `TextFormatter::new(TextVariant::Bullets)` formats the view.
- **Then:** The output is empty or contains only header information; no row content appears.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/formatter/010_text_formatter.md`](../../../docs/formatter/010_text_formatter.md) | Source formatter doc — trait, variant enum, 6 text style variants |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/text_tests.rs`](../../text_tests.rs) | TextFormatter test implementation |
