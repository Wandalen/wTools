# Variant: Text Sections

### Scope

- **Purpose**: Drive test coverage for the sections-based text output variant.
- **Responsibility**: Documents test cases for the Sections text variant in `docs/variant/030_text_sections.md`.
- **In Scope**: Section headers with underlines, grouped field display, record-per-row layout.
- **Out of Scope**: Formatter internals (see `../formatter/`), attribute schema (see `../data_structure/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| VT-1 | each row rendered as a section with header | ✅ |
| VT-2 | section header has underline separator | ✅ |
| VT-3 | fields listed under section header | ✅ |
| VT-4 | empty table produces no sections | ✅ |

---

### VT-1: each row rendered as a section with header

- **Given:** A `TableView` with headers `["Name", "Age"]` and rows `[["Alice", "30"], ["Bob", "25"]]`.
- **When:** Formatted with `TextFormatter` using `TextVariant::Sections`.
- **Then:** Two sections appear in the output; each section represents one row of data.

---

### VT-2: section header has underline separator

- **Given:** A `TableView` with headers `["Name"]` and one row `["Alice"]`.
- **When:** Formatted with `TextVariant::Sections`.
- **Then:** The section header is followed by an underline or separator line; the separator visually separates the header from the field content.

---

### VT-3: fields listed under section header

- **Given:** A `TableView` with headers `["Name", "City"]` and one row `["Alice", "NYC"]`.
- **When:** Formatted with `TextVariant::Sections`.
- **Then:** Both fields appear under the section header with their column names as labels; the format is human-readable prose-style.

---

### VT-4: empty table produces no sections

- **Given:** A `TableView` with headers `["Col"]` and zero data rows.
- **When:** Formatted with `TextVariant::Sections`.
- **Then:** Output is empty; no section headers or field listings appear.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/variant/030_text_sections.md`](../../../docs/variant/030_text_sections.md) | Source variant doc — Text Sections attributes and example |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/text.rs`](../../text.rs) | Text formatter test implementation |
| [`tests/variant_030_text_sections_test.rs`](../../variant_030_text_sections_test.rs) | Spec tests for VT-1..VT-4 — text_sections variant |
