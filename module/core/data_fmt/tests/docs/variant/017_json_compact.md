# Variant: JSON Compact

### Scope

- **Purpose**: Drive test coverage for the compact JSON output variant.
- **Responsibility**: Documents test cases for the Compact JSON variant in `docs/variant/017_json_compact.md`.
- **In Scope**: Valid JSON output, single-line format, minimal whitespace, machine-optimized.
- **Out of Scope**: Formatter internals (see `../formatter/`), attribute schema (see `../data_structure/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| VT-1 | output is valid parseable JSON | ⏳ |
| VT-2 | output is single-line with minimal whitespace | ⏳ |
| VT-3 | compact and pretty produce equivalent data | ⏳ |
| VT-4 | empty table produces valid compact JSON | ⏳ |

---

### VT-1: output is valid parseable JSON

- **Given:** A `TableView` with headers `["Name", "Age"]` and rows `[["Alice", "30"]]`.
- **When:** Formatted with `JsonFormatter` in compact mode.
- **Then:** The output is valid JSON per RFC 8259; parsing succeeds; the data content matches the input.

---

### VT-2: output is single-line with minimal whitespace

- **Given:** A `TableView` with headers `["A", "B"]` and one row.
- **When:** Formatted with `JsonFormatter` in compact mode.
- **Then:** The entire JSON output fits on a single line (or minimal lines); no indentation whitespace; output is smaller than pretty-printed equivalent.

---

### VT-3: compact and pretty produce equivalent data

- **Given:** The same `TableView` formatted with both compact and pretty modes.
- **When:** Both outputs are parsed as JSON.
- **Then:** The parsed data structures are identical; only formatting differs (whitespace and newlines).

---

### VT-4: empty table produces valid compact JSON

- **Given:** A `TableView` with headers `["Col"]` and zero data rows.
- **When:** Formatted with `JsonFormatter` in compact mode.
- **Then:** Output is a valid compact JSON structure with empty rows; parseable without error.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/variant/017_json_compact.md`](../../../docs/variant/017_json_compact.md) | Source variant doc — JSON Compact attributes and example |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/json.rs`](../../json.rs) | JSON formatter test implementation |
