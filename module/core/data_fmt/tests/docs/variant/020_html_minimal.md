# Variant: HTML Minimal

### Scope

- **Purpose**: Drive test coverage for the minimal HTML table output variant.
- **Responsibility**: Documents test cases for the Minimal HTML variant in `docs/variant/020_html_minimal.md`.
- **In Scope**: Valid HTML table structure, semantic elements (thead/tbody/th/td), no CSS classes, UTF-8.
- **Out of Scope**: Formatter internals (see `../formatter/`), attribute schema (see `../data_structure/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| VT-1 | output is valid HTML table structure | ⏳ |
| VT-2 | header cells use th elements | ⏳ |
| VT-3 | no CSS classes or framework-specific attributes | ⏳ |
| VT-4 | empty table produces valid HTML | ⏳ |

---

### VT-1: output is valid HTML table structure

- **Given:** A `TableView` with headers `["Name", "Age"]` and one row `["Alice", "30"]`.
- **When:** Formatted with `HtmlFormatter` using `HtmlVariant::Minimal`.
- **Then:** Output contains `<table>`, `<thead>`, `<tbody>`, `<tr>`, `<th>`, and `<td>` elements; the structure is well-formed HTML.

---

### VT-2: header cells use th elements

- **Given:** A `TableView` with headers `["A", "B"]` and one row.
- **When:** Formatted with `HtmlVariant::Minimal`.
- **Then:** Header cells are rendered as `<th>` elements inside `<thead>`; data cells are `<td>` inside `<tbody>`.

---

### VT-3: no CSS classes or framework-specific attributes

- **Given:** A `TableView` with headers `["X"]` and one row.
- **When:** Formatted with `HtmlVariant::Minimal`.
- **Then:** The output contains no `class=` attributes; no Bootstrap, Tailwind, or custom CSS class names appear; the HTML is unstyled.

---

### VT-4: empty table produces valid HTML

- **Given:** A `TableView` with headers `["Col"]` and zero data rows.
- **When:** Formatted with `HtmlVariant::Minimal`.
- **Then:** Output contains `<table>` and `<thead>` with header row; `<tbody>` is empty or absent; HTML is well-formed.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/variant/020_html_minimal.md`](../../../docs/variant/020_html_minimal.md) | Source variant doc — HTML Minimal attributes and example |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/html.rs`](../../html.rs) | HTML formatter test implementation |
