# API: Data Types

### Scope

- **Purpose**: Drive test coverage for the data-types API contracts in `docs/api/001_data_types.md`.
- **Responsibility**: Documents API contract test cases for `TreeNode`, `TableView`, `TableMetadata`, `DataType`, `ColumnData`, `TreeSymbols`, and `TableShapedView`.
- **In Scope**: Construction methods, field accessors, `TableShapedView` extraction, `ColumnData` helpers, `DataType` default.
- **Out of Scope**: Behavioral invariants (see `../invariant/`); builder patterns (see `../api/002_builders.md`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| AP-1 | TreeNode::new stores name and data; children start empty | ✅ |
| AP-2 | TableView::new constructs view with empty row_details | ✅ |
| AP-3 | TableMetadata::new sets column names; types default to DataType::String | ✅ |
| AP-4 | ColumnData::new stores columns; len() and is_empty() return correct values | ✅ |
| AP-5 | DataType::default() returns DataType::String | ✅ |
| AP-6 | TableShapedView::extract_headers returns column names from table-shaped tree | ✅ |
| AP-7 | TreeNode children vec starts empty; push produces expected count | ✅ |

---

### AP-1: TreeNode::new stores name and data; children start empty

- **Given:** `TreeNode::new("root", Some("payload"))` is constructed.
- **When:** The fields are inspected.
- **Then:** `node.name == "root"`; `node.data == Some("payload")`; `node.children.is_empty()`.

---

### AP-2: TableView::new constructs view with empty row_details

- **Given:** A `TableMetadata::new(vec!["A".to_string(), "B".to_string()])` and one row
  `vec!["x".to_string(), "y".to_string()]`.
- **When:** `TableView::new(metadata, rows)` is called.
- **Then:** `view.rows().len() == 1`; `view.row_details().is_empty()` (no details by default);
  the single row contains `["x", "y"]`.

---

### AP-3: TableMetadata::new sets column names; types default to DataType::String

- **Given:** `TableMetadata::new(vec!["Col1".to_string(), "Col2".to_string()])`.
- **When:** The metadata is inspected.
- **Then:** `metadata.column_names()` returns `["Col1", "Col2"]`; all entries in
  `metadata.column_types()` equal `DataType::String`; length of `column_types` equals
  `column_names` length.

---

### AP-4: ColumnData::new stores columns; len() and is_empty() return correct values

- **Given:** `ColumnData::new(vec!["a".to_string(), "b".to_string(), "c".to_string()])`.
- **When:** `len()` and `is_empty()` are called.
- **Then:** `len() == 3`; `is_empty() == false`; `ColumnData::new(vec![]).is_empty() == true`.

---

### AP-5: DataType::default() returns DataType::String

- **Given:** `DataType::default()` is called.
- **When:** The result is compared to `DataType::String`.
- **Then:** They are equal; `DataType::String` is the default variant.

---

### AP-6: TableShapedView::extract_headers returns column names from table-shaped tree

- **Given:** A table-shaped `TreeNode` built via `RowBuilder::new(vec!["X".to_string(),
  "Y".to_string()]).add_row(...)`.
- **When:** `extract_headers()` is called on the resulting tree.
- **Then:** Returns `Some(["X", "Y"])`; the header order matches the original builder order.

---

### AP-7: TreeNode children vec starts empty; push produces expected count

- **Given:** A `TreeNode::new("root", None::<String>)` with no children.
- **When:** Two child nodes are pushed into `node.children`.
- **Then:** `node.children.len() == 2`; children are accessible by index; no panic occurs.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/api/001_data_types.md`](../../../docs/api/001_data_types.md) | Source API spec — all data type definitions and operations |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/data.rs`](../../data.rs) | Data type API test cases |
| [`tests/column_data.rs`](../../column_data.rs) | ColumnData API test cases |
