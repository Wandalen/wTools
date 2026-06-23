# Variant: Logfmt Standard

### Scope

- **Purpose**: Drive test coverage for the logfmt structured logging output variant.
- **Responsibility**: Documents test cases for the Standard logfmt variant in `docs/variant/015_logfmt_standard.md`.
- **In Scope**: key=value format, space-separated pairs, quoting for values with spaces, one record per line.
- **Out of Scope**: Formatter internals (see `../formatter/`), attribute schema (see `../data_structure/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| VT-1 | output is key=value pairs per line | ✅ |
| VT-2 | pairs are space-separated | ✅ |
| VT-3 | values with spaces are quoted | ✅ |
| VT-4 | empty table produces empty output | ✅ |

---

### VT-1: output is key=value pairs per line

- **Given:** A `TableView` with headers `["Name", "Age"]` and one row `["Alice", "30"]`.
- **When:** Formatted with `LogfmtFormatter`.
- **Then:** Output contains `Name=Alice` and `Age=30` on the same line; each row produces one line of key=value pairs.

---

### VT-2: pairs are space-separated

- **Given:** A `TableView` with headers `["A", "B", "C"]` and one row `["1", "2", "3"]`.
- **When:** Formatted with `LogfmtFormatter`.
- **Then:** The output line contains `A=1 B=2 C=3` with single spaces between pairs; no commas or other separators.

---

### VT-3: values with spaces are quoted

- **Given:** A `TableView` with headers `["Name", "City"]` and one row `["Alice", "New York"]`.
- **When:** Formatted with `LogfmtFormatter`.
- **Then:** The city value is quoted: `City="New York"`; values without spaces remain unquoted.

---

### VT-4: empty table produces empty output

- **Given:** A `TableView` with headers `["Col"]` and zero data rows.
- **When:** Formatted with `LogfmtFormatter`.
- **Then:** Output is empty; no header line appears (logfmt has no header concept); no trailing whitespace.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/variant/015_logfmt_standard.md`](../../../docs/variant/015_logfmt_standard.md) | Source variant doc — logfmt Standard attributes and example |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/logfmt.rs`](../../logfmt.rs) | Logfmt formatter test implementation |
| [`tests/variant_015_logfmt_test.rs`](../../variant_015_logfmt_test.rs) | Spec tests for VT-1..VT-4 logfmt variant |
