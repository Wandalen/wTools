# Variant: HTML Tailwind

### Scope

- **Purpose**: Drive test coverage for the Tailwind-styled HTML table output variant.
- **Responsibility**: Documents test cases for the Tailwind HTML variant in `docs/variant/022_html_tailwind.md`.
- **In Scope**: Tailwind CSS utility classes, semantic HTML structure, responsive table design.
- **Out of Scope**: Formatter internals (see `../formatter/`), attribute schema (see `../data_structure/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| VT-1 | output contains Tailwind CSS utility classes | ✅ |
| VT-2 | valid HTML table structure | ✅ |
| VT-3 | no Bootstrap or custom CSS classes | ✅ |
| VT-4 | empty table produces valid Tailwind HTML | ✅ |

---

### VT-1: output contains Tailwind CSS utility classes

- **Given:** A `TableView` with headers `["Name", "Age"]` and one row.
- **When:** Formatted with `HtmlFormatter` using `HtmlVariant::Tailwind`.
- **Then:** HTML elements contain `class` attributes with Tailwind utility class names (e.g., `min-w-full`, `divide-y`).

---

### VT-2: valid HTML table structure

- **Given:** A `TableView` with headers `["A", "B"]` and one row.
- **When:** Formatted with `HtmlVariant::Tailwind`.
- **Then:** Output contains `<table>`, `<thead>`, `<tbody>`, `<th>`, `<td>` elements in valid nesting.

---

### VT-3: no Bootstrap or custom CSS classes

- **Given:** A `TableView` with headers `["X"]` and one row.
- **When:** Formatted with `HtmlVariant::Tailwind`.
- **Then:** No Bootstrap class names (e.g., `table-striped`) appear; classes are exclusively Tailwind utilities.

---

### VT-4: empty table produces valid Tailwind HTML

- **Given:** A `TableView` with headers `["Col"]` and zero data rows.
- **When:** Formatted with `HtmlVariant::Tailwind`.
- **Then:** Output is well-formed HTML with Tailwind classes; `<tbody>` is empty; header row present.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/variant/022_html_tailwind.md`](../../../docs/variant/022_html_tailwind.md) | Source variant doc — HTML Tailwind attributes and example |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/html.rs`](../../html.rs) | HTML formatter test implementation |
| [`tests/variant_022_html_tailwind_test.rs`](../../variant_022_html_tailwind_test.rs) | Spec tests for VT-1..VT-4 — html_tailwind variant |
