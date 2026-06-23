# Input Type: TableView

### Scope

- **Purpose**: Drive test coverage for the `TableView` Rust struct as canonical tabular input type.
- **Responsibility**: Documents test cases for the TableView type in `docs/input_type/001_table_view.md`.
- **In Scope**: Struct fields (metadata, rows, row_details), construction via RowBuilder, direct construction, Format trait consumption by 9 formatters.
- **Out of Scope**: Conceptual data shape (see `../input_model/`), formatter output correctness (see `../formatter/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| IV-1 | construct TableView via build_view | ✅ |
| IV-2 | metadata carries column names and type classifications | ✅ |
| IV-3 | row_details parallels rows in length | ✅ |
| IV-4 | 9 of 10 formatters accept TableView via Format trait | ✅ |

---

### IV-1: construct TableView via build_view

- **Given:** A `RowBuilder` with headers `["Name", "Age"]` and one row `["Alice", "30"]` added.
- **When:** `build_view()` is called.
- **Then:** The result is a `TableView`; `rows` has length 1; `rows[0]` has length 2; cell values match `"Alice"` and `"30"`.

---

### IV-2: metadata carries column names and type classifications

- **Given:** A `TableView` constructed with headers `["id", "name", "active"]`.
- **When:** Inspecting `TableView.metadata`.
- **Then:** `metadata` contains 3 column entries; column names are `"id"`, `"name"`, `"active"` in order; `DataType` classifications are available per column.

---

### IV-3: row_details parallels rows in length

- **Given:** A `RowBuilder` with 2 headers; 5 rows added via `add_row`, plus 1 row added via `add_row_with_detail`.
- **When:** `build_view()` is called.
- **Then:** `row_details.len() == rows.len() == 6`; the detailed row has `Some(...)` in `row_details`; all other entries are `None`.

---

### IV-4: 9 of 10 formatters accept TableView via Format trait

- **Given:** A `TableView` with headers `["x"]` and one row `["1"]`.
- **When:** Passing the view to each formatter that implements `Format`.
- **Then:** `TableFormatter`, `ExpandedFormatter`, `LogfmtFormatter`, `HtmlFormatter`, `SqlFormatter`, `JsonFormatter`, `YamlFormatter`, `TomlFormatter`, and `TextFormatter` all return `Ok(String)`; `TreeFormatter` does not implement `Format` and cannot accept `TableView`.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/input_type/001_table_view.md`](../../../docs/input_type/001_table_view.md) | Source input type doc — TableView struct definition, components, formatter coverage |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/data.rs`](../../data.rs) | Data model test implementation |
| [`tests/input_type_test.rs`](../../input_type_test.rs) | Spec tests for IV-1..IV-4 TableView type |
