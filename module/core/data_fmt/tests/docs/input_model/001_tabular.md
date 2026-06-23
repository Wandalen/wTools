# Input Model: Tabular

### Scope

- **Purpose**: Drive test coverage for the tabular input model shape.
- **Responsibility**: Documents test cases for the tabular data model in `docs/input_model/001_tabular.md`.
- **In Scope**: Header schema definition, row-length invariant, row details parallel vector, column order stability, downstream formatter consumption.
- **Out of Scope**: Rust type details (see `../input_type/`), builder API (see `../builder/`), formatter output correctness (see `../formatter/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| IM-1 | headers define column schema | ⏳ |
| IM-2 | every row has same cell count as headers | ⏳ |
| IM-3 | row details parallel to rows | ⏳ |
| IM-4 | column order stable across rows | ⏳ |

---

### IM-1: headers define column schema

- **Given:** A `RowBuilder` created with headers `["Name", "Age", "City"]`.
- **When:** `build_view()` is called after adding rows.
- **Then:** `TableView.metadata` contains exactly 3 column names in order: `"Name"`, `"Age"`, `"City"`; headers are ordered and define the schema for all rows.

---

### IM-2: every row has same cell count as headers

- **Given:** A `RowBuilder` with 3 headers and multiple rows added, each with exactly 3 cells.
- **When:** `build_view()` is called.
- **Then:** Every row in `TableView.rows` has length 3; the invariant is enforced at insertion time (adding a row with a different cell count panics).

---

### IM-3: row details parallel to rows

- **Given:** A `RowBuilder` with 2 headers; 3 rows added, one with detail annotation, two without.
- **When:** `build_view()` is called.
- **Then:** `TableView.row_details` has length 3 (same as `rows`); entry with annotation is `Some(...)`, entries without are `None`; no index mismatch is possible.

---

### IM-4: column order stable across rows

- **Given:** A `RowBuilder` with headers `["X", "Y", "Z"]` and 3 rows added.
- **When:** `build_view()` is called.
- **Then:** For every row, cell at index 0 corresponds to column `"X"`, index 1 to `"Y"`, index 2 to `"Z"`; column order is identical across all rows and matches the header order.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/input_model/001_tabular.md`](../../../docs/input_model/001_tabular.md) | Source input model doc — tabular data shape, invariants, downstream connections |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/data.rs`](../../data.rs) | Data model test implementation |
