# Builder: RowBuilder

### Scope

- **Purpose**: Drive test coverage for the RowBuilder construction helper.
- **Responsibility**: Documents test cases for the RowBuilder API in `docs/builder/001_row_builder.md`.
- **In Scope**: Basic row construction, empty table edge case, headers-with-rows round-trip, fluent chaining, mutable builder usage, row-detail annotation, named-row insertion, row-length validation.
- **Out of Scope**: Formatter output correctness (see `../algorithm/`); TableView internals (see `../input_type/001_table_view.md`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| BL-1 | basic single-row construction | ⏳ |
| BL-2 | empty table with zero rows | ⏳ |
| BL-3 | headers and multiple rows round-trip | ⏳ |
| BL-4 | fluent chaining preserves insertion order | ⏳ |
| BL-5 | add_row_mut in a loop | ⏳ |
| BL-6 | row with detail annotation | ⏳ |
| BL-7 | named rows via add_row_with_name | ⏳ |
| BL-8 | mismatched row length panics | ⏳ |

---

### BL-1: basic single-row construction

- **Given:** A `RowBuilder` created with headers `["Name", "Age"]`.
- **When:** One row `["Alice", "30"]` is added via `add_row` and `build_view()` is called.
- **Then:** The resulting `TableView` has exactly 1 row; `rows[0][0].render()` returns `"Alice"`; `rows[0][1].render()` returns `"30"`; `row_details[0]` is `None`.

---

### BL-2: empty table with zero rows

- **Given:** A `RowBuilder` created with headers `["X", "Y"]`.
- **When:** No rows are added and `build_view()` is called immediately.
- **Then:** The resulting `TableView` has 0 rows; `rows` is empty; `row_details` is empty; headers contain `"X"` and `"Y"`.

---

### BL-3: headers and multiple rows round-trip

- **Given:** A `RowBuilder` created with headers `["A", "B", "C"]`.
- **When:** Three rows are added: `["1","2","3"]`, `["4","5","6"]`, `["7","8","9"]`; then `build_view()` is called.
- **Then:** The resulting `TableView` has exactly 3 rows; each row has length 3; cell values match the insertion data in order; `row_details` has length 3 with all entries `None`.

---

### BL-4: fluent chaining preserves insertion order

- **Given:** A `RowBuilder` created with headers `["Col"]`.
- **When:** Five rows are added in a single fluent chain: `.add_row(["a"]).add_row(["b"]).add_row(["c"]).add_row(["d"]).add_row(["e"])`, then `build_view()` is called.
- **Then:** `rows[0][0].render()` is `"a"`, `rows[1][0].render()` is `"b"`, through `rows[4][0].render()` is `"e"`; insertion order is exactly preserved.

---

### BL-5: add_row_mut in a loop

- **Given:** A `RowBuilder` created with headers `["Value"]`; a loop that iterates 10 times.
- **When:** Each iteration calls `add_row_mut` with `[i.to_string()]` for `i` in `0..10`; then `build_view()` is called.
- **Then:** The resulting `TableView` has exactly 10 rows; `rows[0][0].render()` is `"0"` and `rows[9][0].render()` is `"9"`; order matches loop iteration order.

---

### BL-6: row with detail annotation

- **Given:** A `RowBuilder` created with headers `["Name"]`.
- **When:** `add_row_with_detail` is called with row `["Alice"]` and detail `Some("extra info")`; a second row is added via `add_row` with `["Bob"]` (no detail); then `build_view()` is called.
- **Then:** `row_details[0]` is `Some(DecoratedText::from("extra info"))`; `row_details[1]` is `None`; both rows are present in `rows` at their expected indices.

---

### BL-7: named rows via add_row_with_name

- **Given:** A `RowBuilder` created with headers `["Score"]`.
- **When:** Two rows are added: `add_row_with_name("Alice", ["95"])` and `add_row_with_name("Bob", ["87"])`; then `build_view()` is called.
- **Then:** The resulting `TableView` has exactly 2 rows; cell values are `"95"` and `"87"` respectively; rows are in insertion order.

---

### BL-8: mismatched row length panics

- **Given:** A `RowBuilder` created with headers `["A", "B"]` (2 columns).
- **When:** `add_row` is called with a 3-element vector `["1", "2", "3"]`.
- **Then:** The call panics with a message containing `"row length 3 doesn't match headers length 2"`; no `TableView` is produced.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/builder/001_row_builder.md`](../../../docs/builder/001_row_builder.md) | Source builder spec — construction API, invariants, usage |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/builder.rs`](../../builder.rs) | Builder test implementation |
