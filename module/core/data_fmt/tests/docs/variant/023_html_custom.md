# Variant: HTML Custom

### Scope

- **Purpose**: Drive test coverage for the custom-CSS HTML table output variant.
- **Responsibility**: Documents test cases for the Custom HTML variant in `docs/variant/023_html_custom.md`.
- **In Scope**: User-provided CSS class string, HtmlVariant::Custom constructor, valid HTML structure.
- **Out of Scope**: Formatter internals (see `../formatter/`), attribute schema (see `../data_structure/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| VT-1 | custom CSS class applied to table element | ✅ |
| VT-2 | valid HTML table structure | ✅ |
| VT-3 | user-provided class string appears verbatim | ✅ |
| VT-4 | empty table produces valid HTML with custom class | ✅ |

---

### VT-1: custom CSS class applied to table element

- **Given:** A `TableView` with headers `["Name"]` and one row.
- **When:** Formatted with `HtmlFormatter` using `HtmlVariant::Custom("my-table dark-theme".into())`.
- **Then:** The `<table>` element contains `class="my-table dark-theme"`; the user-provided string is used as-is.

---

### VT-2: valid HTML table structure

- **Given:** A `TableView` with headers `["A", "B"]` and one row.
- **When:** Formatted with `HtmlVariant::Custom("custom".into())`.
- **Then:** Output contains `<thead>`, `<tbody>`, `<th>`, `<td>` in valid nesting; the custom class does not break HTML structure.

---

### VT-3: user-provided class string appears verbatim

- **Given:** A custom class string `"data-grid responsive shadow-lg"`.
- **When:** Passed to `HtmlVariant::Custom` and used to format a table.
- **Then:** The exact string appears in the `class` attribute; no modification, escaping, or class injection occurs.

---

### VT-4: empty table produces valid HTML with custom class

- **Given:** A `TableView` with headers `["Col"]` and zero data rows.
- **When:** Formatted with `HtmlVariant::Custom("empty-table".into())`.
- **Then:** Output is well-formed HTML with `class="empty-table"` on the `<table>` element; `<tbody>` is empty.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/variant/023_html_custom.md`](../../../docs/variant/023_html_custom.md) | Source variant doc — HTML Custom attributes and example |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/html.rs`](../../html.rs) | HTML formatter test implementation |
