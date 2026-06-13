# API: Builders

### Scope

- **Purpose**: Drive test coverage for the builder API contracts in `docs/api/002_builders.md`.
- **Responsibility**: Documents API contract test cases for `RowBuilder`, `TreeBuilder`, `FlattenConfig`, and the flatten functions.
- **In Scope**: `RowBuilder` fluent and mutable chains, `build_view()` terminal operation, `TreeBuilder` path insertion, `FlattenConfig` field defaults, `flatten_to_table_tree` column output.
- **Out of Scope**: Invariant enforcement (see `../invariant/001_data_model.md`); data-types API (see `001_data_types.md`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| AP-1 | RowBuilder::build_view produces TableView with correct headers and rows | ✅ |
| AP-2 | RowBuilder fluent chain accumulates rows without mutation | ✅ |
| AP-3 | RowBuilder mutable API produces same result as fluent API | ✅ |
| AP-4 | TreeBuilder::insert creates intermediate directory nodes automatically | ✅ |
| AP-5 | TreeBuilder::build returns root TreeNode with all inserted paths | ✅ |
| AP-6 | flatten_to_table_tree produces path/name/depth/data columns | ✅ |
| AP-7 | FlattenConfig defaults all fields to true; custom names override defaults | ✅ |

---

### AP-1: RowBuilder::build_view produces TableView with correct headers and rows

- **Given:** A `RowBuilder::new(vec!["A".to_string(), "B".to_string()])` with two rows added.
- **When:** `build_view()` is called.
- **Then:** The returned `TableView` has metadata with column names `["A", "B"]`; `rows().len() == 2`;
  each row has 2 cells matching the values passed to `add_row`.

---

### AP-2: RowBuilder fluent chain accumulates rows without mutation

- **Given:** A `RowBuilder` created fresh each step via `let rb = rb.add_row(row)`.
- **When:** `build_view()` is called after two `add_row` calls in a fluent chain.
- **Then:** The `TableView` contains both rows; intermediate builder values are consumed
  without the caller holding references.

---

### AP-3: RowBuilder mutable API produces same result as fluent API

- **Given:** Two `RowBuilder` instances — one using `add_row` (fluent) and one using
  `add_row_mut` (mutable) — both adding the same rows.
- **When:** Both are finalized via `build_view()`.
- **Then:** Both `TableView` results contain identical headers and row content.

---

### AP-4: TreeBuilder::insert creates intermediate directory nodes automatically

- **Given:** A `TreeBuilder::new("root")` and a call `insert(&["a", "b", "leaf"], 42u64)`.
- **When:** `build()` is called.
- **Then:** The root has one child `"a"` (directory); `"a"` has one child `"b"` (directory);
  `"b"` has one child `"leaf"` with `data == Some(42)`; no panic occurs.

---

### AP-5: TreeBuilder::build returns root TreeNode with all inserted paths

- **Given:** Two `insert` calls on the same `TreeBuilder` — `(&["x"], 1)` and `(&["y"], 2)`.
- **When:** `build()` is called.
- **Then:** The root has exactly two children `"x"` and `"y"`, both with `data == Some(_)`;
  insertion order is preserved.

---

### AP-6: flatten_to_table_tree produces path/name/depth/data columns

- **Given:** A simple `TreeNode` hierarchy with two levels passed to `flatten_to_table_tree`.
- **When:** The returned `TableView` is inspected.
- **Then:** The metadata has 4 column names: `"path"`, `"name"`, `"depth"`, `"data"`;
  each row represents one tree node with the correct values in each column.

---

### AP-7: FlattenConfig defaults all fields to true; custom names override defaults

- **Given:** A `FlattenConfig::default()` (all fields true, default column names).
- **When:** Field values and column names are inspected.
- **Then:** `include_path`, `include_name`, `include_depth`, `include_data` are all `true`;
  default column name for path is `"path"`, for name is `"name"`, etc.
  Setting custom column names via `FlattenConfig` produces those names in the resulting
  `TableView` metadata.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/api/002_builders.md`](../../../docs/api/002_builders.md) | Source API spec — builder signatures, FlattenConfig, flatten functions |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/builder.rs`](../../builder.rs) | Builder API test cases |
| [`tests/flatten_config.rs`](../../flatten_config.rs) | FlattenConfig test cases |
| [`tests/fluent_api.rs`](../../fluent_api.rs) | Fluent builder chain test cases |
