# Formatter: HtmlFormatter

### Scope

- **Purpose**: Drive test coverage for the HtmlFormatter output contract.
- **Responsibility**: Documents test cases for the `HtmlFormatter` struct described in `docs/formatter/008_html_formatter.md`.
- **In Scope**: HTML table element output, variant-specific CSS class attributes, custom class injection, Format trait dispatch, empty data handling, HTML entity escaping.
- **Out of Scope**: Per-variant visual details (see `tests/docs/variant/`), CSS framework behavior.

### Case Index

| ID | Name | Status |
|----|------|--------|
| FM-32 | minimal variant produces bare HTML table | ⏳ |
| FM-33 | bootstrap variant adds Bootstrap CSS classes | ⏳ |
| FM-34 | tailwind variant adds Tailwind CSS classes | ⏳ |
| FM-35 | custom variant injects user-provided CSS classes | ⏳ |
| FM-36 | Format trait dispatch returns well-formed string | ⏳ |
| FM-37 | empty data produces header-only HTML table | ⏳ |
| FM-38 | HTML special characters are entity-escaped | ⏳ |

---

### FM-32: minimal variant produces bare HTML table

- **Given:** A `TableView` with headers `["name", "age"]` and one row `["Alice", "30"]`.
- **When:** `HtmlFormatter::with_variant(HtmlVariant::Minimal)` formats the view.
- **Then:** The output contains `<table>`, `<thead>`, `<tbody>`, `<tr>`, `<th>`, and `<td>` elements; no CSS framework class attributes appear on the table element.

---

### FM-33: bootstrap variant adds Bootstrap CSS classes

- **Given:** A `TableView` with headers `["x"]` and one row `["1"]`.
- **When:** `HtmlFormatter::with_variant(HtmlVariant::Bootstrap)` formats the view.
- **Then:** The `<table>` element contains Bootstrap-specific CSS class attributes (e.g., `class="table"`); the output is valid HTML.

---

### FM-34: tailwind variant adds Tailwind CSS classes

- **Given:** A `TableView` with headers `["x"]` and one row `["1"]`.
- **When:** `HtmlFormatter::with_variant(HtmlVariant::Tailwind)` formats the view.
- **Then:** The `<table>` element contains Tailwind-specific CSS class attributes; the output is valid HTML.

---

### FM-35: custom variant injects user-provided CSS classes

- **Given:** A `TableView` with headers `["x"]` and one row `["1"]`; a custom class string `"my-table striped"`.
- **When:** `HtmlFormatter::with_variant(HtmlVariant::Custom)` is configured with the custom class string and formats the view.
- **Then:** The `<table>` element contains `class="my-table striped"`; no framework-specific classes appear.

---

### FM-36: Format trait dispatch returns well-formed string

- **Given:** A `TableView` with headers `["a"]` and rows `[["1"]]`.
- **When:** The `Format::fmt` method is called on an `HtmlFormatter` instance (minimal variant).
- **Then:** The return value is `Ok(String)` containing valid HTML; no `FormatError` is returned.

---

### FM-37: empty data produces header-only HTML table

- **Given:** A `TableView` with headers `["col"]` and zero data rows.
- **When:** `HtmlFormatter::with_variant(HtmlVariant::Minimal)` formats the view.
- **Then:** The output contains `<thead>` with one `<th>` element; `<tbody>` is empty or contains no `<tr>` elements.

---

### FM-38: HTML special characters are entity-escaped

- **Given:** A `TableView` with headers `["text"]` and one row containing `"<b>bold</b> & \"quoted\""`.
- **When:** `HtmlFormatter::with_variant(HtmlVariant::Minimal)` formats the view.
- **Then:** Angle brackets are escaped as `&lt;`/`&gt;`; ampersand is escaped as `&amp;`; quotes are escaped as `&quot;`; the output is valid HTML.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/formatter/008_html_formatter.md`](../../../docs/formatter/008_html_formatter.md) | Source formatter doc — trait, variant enum, 4 CSS theme variants |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/html_tests.rs`](../../html_tests.rs) | HtmlFormatter test implementation |
