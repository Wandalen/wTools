# Variant: JSON Pretty

### Scope

- **Purpose**: Drive test coverage for the pretty-printed JSON output variant.
- **Responsibility**: Documents test cases for the Pretty JSON variant in `docs/variant/016_json_pretty.md`.
- **In Scope**: Valid JSON output, indentation with newlines, RFC 8259 compliance, backslash escaping.
- **Out of Scope**: Formatter internals (see `../formatter/`), attribute schema (see `../data_structure/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| VT-1 | output is valid parseable JSON | ⏳ |
| VT-2 | output is indented with newlines | ⏳ |
| VT-3 | special characters are backslash-escaped | ⏳ |
| VT-4 | empty table produces valid JSON structure | ⏳ |

---

### VT-1: output is valid parseable JSON

- **Given:** A `TableView` with headers `["Name", "Age"]` and rows `[["Alice", "30"]]`.
- **When:** Formatted with `JsonFormatter` in pretty mode.
- **Then:** The output is valid JSON per RFC 8259; parsing with any JSON parser succeeds; the structure contains the row data keyed by header names.

---

### VT-2: output is indented with newlines

- **Given:** A `TableView` with headers `["key", "val"]` and one row.
- **When:** Formatted with `JsonFormatter` in pretty mode (default).
- **Then:** The output spans multiple lines; nested structures are indented; the output is human-readable.

---

### VT-3: special characters are backslash-escaped

- **Given:** A `TableView` with a cell containing `"hello \"world\""` (embedded quotes).
- **When:** Formatted with `JsonFormatter`.
- **Then:** The quotes are escaped with backslashes in the JSON output; the output remains valid JSON.

---

### VT-4: empty table produces valid JSON structure

- **Given:** A `TableView` with headers `["Col"]` and zero data rows.
- **When:** Formatted with `JsonFormatter` in pretty mode.
- **Then:** Output is a valid JSON structure with an empty rows array; the output parses without error.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/variant/016_json_pretty.md`](../../../docs/variant/016_json_pretty.md) | Source variant doc — JSON Pretty attributes and example |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/json.rs`](../../json.rs) | JSON formatter test implementation |
