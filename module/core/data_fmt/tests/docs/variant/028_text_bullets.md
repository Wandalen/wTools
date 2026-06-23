# Variant: Text Bullets

### Scope

- **Purpose**: Drive test coverage for the bullet-point text output variant.
- **Responsibility**: Documents test cases for the Bullets text variant in `docs/variant/028_text_bullets.md`.
- **In Scope**: Bullet character prefix, one item per row, plain text output, no borders or alignment.
- **Out of Scope**: Formatter internals (see `../formatter/`), attribute schema (see `../data_structure/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| VT-1 | each row rendered with bullet prefix | ✅ |
| VT-2 | no border or alignment characters | ✅ |
| VT-3 | multi-column rows formatted as key-value | ✅ |
| VT-4 | empty table produces no bullets | ✅ |

---

### VT-1: each row rendered with bullet prefix

- **Given:** A `TableView` with headers `["Name", "Age"]` and rows `[["Alice", "30"], ["Bob", "25"]]`.
- **When:** Formatted with `TextFormatter` using `TextVariant::Bullets`.
- **Then:** Each row is rendered with a bullet character prefix (e.g., `•` or `-`); two bullet items appear in the output.

---

### VT-2: no border or alignment characters

- **Given:** A `TableView` with headers `["A", "B"]` and one row.
- **When:** Formatted with `TextVariant::Bullets`.
- **Then:** No `|`, `+`, `-` border characters or column alignment whitespace appear; output is plain bullet-point text.

---

### VT-3: multi-column rows formatted as key-value

- **Given:** A `TableView` with headers `["Name", "City"]` and one row `["Alice", "NYC"]`.
- **When:** Formatted with `TextVariant::Bullets`.
- **Then:** Header names are used as labels for cell values; each bullet item includes field information from all columns.

---

### VT-4: empty table produces no bullets

- **Given:** A `TableView` with headers `["Col"]` and zero data rows.
- **When:** Formatted with `TextVariant::Bullets`.
- **Then:** Output is empty; no bullet items appear; no header-only output.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/variant/028_text_bullets.md`](../../../docs/variant/028_text_bullets.md) | Source variant doc — Text Bullets attributes and example |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/text.rs`](../../text.rs) | Text formatter test implementation |
| [`tests/variant_028_text_bullets_test.rs`](../../variant_028_text_bullets_test.rs) | Spec tests for VT-1..VT-4 text bullets variant |
