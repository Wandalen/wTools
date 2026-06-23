# Variant: Text Numbered

### Scope

- **Purpose**: Drive test coverage for the numbered-list text output variant.
- **Responsibility**: Documents test cases for the Numbered text variant in `docs/variant/029_text_numbered.md`.
- **In Scope**: Sequential numbering prefix, one item per row, plain text output.
- **Out of Scope**: Formatter internals (see `../formatter/`), attribute schema (see `../data_structure/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| VT-1 | each row has sequential number prefix | ✅ |
| VT-2 | numbering starts at 1 | ✅ |
| VT-3 | multi-column rows include all fields | ✅ |
| VT-4 | empty table produces no numbered items | ✅ |

---

### VT-1: each row has sequential number prefix

- **Given:** A `TableView` with headers `["Name"]` and rows `[["Alice"], ["Bob"], ["Charlie"]]`.
- **When:** Formatted with `TextFormatter` using `TextVariant::Numbered`.
- **Then:** Each row is prefixed with its sequential number (e.g., `1.`, `2.`, `3.`); three numbered items appear.

---

### VT-2: numbering starts at 1

- **Given:** A `TableView` with headers `["X"]` and one row `["value"]`.
- **When:** Formatted with `TextVariant::Numbered`.
- **Then:** The first (and only) item is prefixed with `1`; numbering is 1-based, not 0-based.

---

### VT-3: multi-column rows include all fields

- **Given:** A `TableView` with headers `["Name", "Age"]` and one row `["Alice", "30"]`.
- **When:** Formatted with `TextVariant::Numbered`.
- **Then:** The numbered item includes information from both columns; header names are used as labels.

---

### VT-4: empty table produces no numbered items

- **Given:** A `TableView` with headers `["Col"]` and zero data rows.
- **When:** Formatted with `TextVariant::Numbered`.
- **Then:** Output is empty; no numbered items appear.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/variant/029_text_numbered.md`](../../../docs/variant/029_text_numbered.md) | Source variant doc — Text Numbered attributes and example |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/text.rs`](../../text.rs) | Text formatter test implementation |
