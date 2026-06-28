# Variant: Text Compact

### Scope

- **Purpose**: Drive test coverage for the compact text output variant.
- **Responsibility**: Documents test cases for the Compact text variant in `docs/variant/032_text_compact.md`.
- **In Scope**: Comma-separated values within records, minimal overhead, machine-partial parseability.
- **Out of Scope**: Formatter internals (see `../formatter/`), attribute schema (see `../data_structure/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| VT-1 | fields separated by commas within record | ✅ |
| VT-2 | minimal output overhead | ✅ |
| VT-3 | multiple rows produce separate lines | ✅ |
| VT-4 | empty table produces no output | ✅ |

---

### VT-1: fields separated by commas within record

- **Given:** A `TableView` with headers `["Name", "Age"]` and one row `["Alice", "30"]`.
- **When:** Formatted with `TextFormatter` using `TextVariant::Compact`.
- **Then:** Fields within each record are separated by commas; header names are used as labels.

---

### VT-2: minimal output overhead

- **Given:** A `TableView` with headers `["A", "B"]` and one row `["x", "y"]`.
- **When:** Formatted with `TextVariant::Compact`.
- **Then:** Output has minimal whitespace; no borders, grid lines, or decorative elements; output is smaller than other text variants for the same data.

---

### VT-3: multiple rows produce separate lines

- **Given:** A `TableView` with headers `["Name"]` and rows `[["Alice"], ["Bob"]]`.
- **When:** Formatted with `TextVariant::Compact`.
- **Then:** Each row produces a separate output record; records are on separate lines.

---

### VT-4: empty table produces no output

- **Given:** A `TableView` with headers `["Col"]` and zero data rows.
- **When:** Formatted with `TextVariant::Compact`.
- **Then:** Output is empty; no records, headers, or decorative elements appear.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/variant/032_text_compact.md`](../../../docs/variant/032_text_compact.md) | Source variant doc — Text Compact attributes and example |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/text.rs`](../../text.rs) | Text formatter test implementation |
| [`tests/variant_032_text_compact_test.rs`](../../variant_032_text_compact_test.rs) | Spec tests for VT-1..VT-4 — text_compact variant |
