# Variant: HTML Bootstrap

### Scope

- **Purpose**: Drive test coverage for the Bootstrap-styled HTML table output variant.
- **Responsibility**: Documents test cases for the Bootstrap HTML variant in `docs/variant/021_html_bootstrap.md`.
- **In Scope**: Bootstrap CSS classes, semantic HTML structure, table-striped/table-bordered classes.
- **Out of Scope**: Formatter internals (see `../formatter/`), attribute schema (see `../data_structure/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| VT-1 | output contains Bootstrap CSS classes | ✅ |
| VT-2 | valid HTML table structure | ✅ |
| VT-3 | table element has Bootstrap class attribute | ✅ |
| VT-4 | empty table produces valid Bootstrap HTML | ✅ |

---

### VT-1: output contains Bootstrap CSS classes

- **Given:** A `TableView` with headers `["Name", "Age"]` and one row.
- **When:** Formatted with `HtmlFormatter` using `HtmlVariant::Bootstrap`.
- **Then:** The `<table>` element contains a `class` attribute with Bootstrap class names (e.g., `table`, `table-striped`).

---

### VT-2: valid HTML table structure

- **Given:** A `TableView` with headers `["A", "B"]` and one row.
- **When:** Formatted with `HtmlVariant::Bootstrap`.
- **Then:** Output contains `<thead>`, `<tbody>`, `<th>`, `<td>` elements; the structure is identical to Minimal except for CSS classes.

---

### VT-3: table element has Bootstrap class attribute

- **Given:** A `TableView` with headers `["X"]` and one row.
- **When:** Formatted with `HtmlVariant::Bootstrap`.
- **Then:** The `<table>` tag includes `class="..."` with Bootstrap-specific values; no Tailwind or custom classes appear.

---

### VT-4: empty table produces valid Bootstrap HTML

- **Given:** A `TableView` with headers `["Col"]` and zero data rows.
- **When:** Formatted with `HtmlVariant::Bootstrap`.
- **Then:** Output is well-formed HTML with Bootstrap classes; `<tbody>` is empty; header row is present.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/variant/021_html_bootstrap.md`](../../../docs/variant/021_html_bootstrap.md) | Source variant doc — HTML Bootstrap attributes and example |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/html.rs`](../../html.rs) | HTML formatter test implementation |
