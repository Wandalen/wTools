# Trait: TableShapedView

### Scope

- **Purpose**: Drive test coverage for the TableShapedView trait contract.
- **Responsibility**: Documents test cases for the TableShapedView trait in `docs/trait/003_table_shaped_view.md`.
- **In Scope**: `extract_headers` return value, `is_table_shaped` structural check, `to_rows` matrix extraction, blanket impl for display-capable tree nodes, trait object dispatch, edge cases (empty tree, mismatched columns).
- **Out of Scope**: TableView direct construction (see `../api/001_data_types.md`); formatter rendering of extracted data (see `../feature/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| TR-7 | extract_headers returns column names from first row | ✅ |
| TR-8 | is_table_shaped returns true for uniform structure | ✅ |
| TR-9 | to_rows returns cell values as string matrix | ✅ |
| TR-10 | trait object dispatch through dyn TableShapedView | ✅ |
| TR-11 | extract_headers returns None on empty tree | ✅ |
| TR-12 | is_table_shaped returns false on mismatched columns | ✅ |

---

### TR-7: extract_headers returns column names from first row

- **Given:** A `TreeNode<String>` root with one child row containing two children named `"Name"` and `"Age"`.
- **When:** `extract_headers()` is called on the root.
- **Then:** The result is `Some(vec!["Name", "Age"])`; the header names match the child node names of the first row exactly; no data values appear in the headers.

---

### TR-8: is_table_shaped returns true for uniform structure

- **Given:** A `TreeNode<String>` root with three child rows, each having two children named `"Col1"` and `"Col2"` with string data values.
- **When:** `is_table_shaped()` is called on the root.
- **Then:** The result is `true`; the method confirms all rows share identical column names and column count.

---

### TR-9: to_rows returns cell values as string matrix

- **Given:** A `TreeNode<String>` root with two child rows; row 1 has cells `("Name", "Alice")` and `("Age", "30")`; row 2 has cells `("Name", "Bob")` and `("Age", "25")`.
- **When:** `to_rows()` is called on the root.
- **Then:** The result is `vec![vec!["Alice", "30"], vec!["Bob", "25"]]`; each inner vec has length 2; the outer vec has length 2; cell values come from `data` fields, not `name` fields.

---

### TR-10: trait object dispatch through dyn TableShapedView

- **Given:** A `TreeNode<String>` root with one row and two columns, stored behind a `&dyn TableShapedView` reference.
- **When:** `extract_headers()`, `is_table_shaped()`, and `to_rows()` are called through the trait object.
- **Then:** All three methods return the same values as direct calls on the concrete type; the trait is object-safe for this usage pattern.

---

### TR-11: extract_headers returns None on empty tree

- **Given:** A `TreeNode<String>` root with no children (empty `children` vec).
- **When:** `extract_headers()` is called on the root.
- **Then:** The result is `None`; no panic occurs; `is_table_shaped()` on the same tree returns `true` (empty tree is trivially table-shaped per the implementation contract).

---

### TR-12: is_table_shaped returns false on mismatched columns

- **Given:** A `TreeNode<String>` root with two child rows; row 1 has children named `"A"` and `"B"`; row 2 has children named `"A"` and `"C"` (different second column name).
- **When:** `is_table_shaped()` is called on the root.
- **Then:** The result is `false`; the structural mismatch in column names is detected; `extract_headers()` still returns the first row's headers regardless.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/trait/003_table_shaped_view.md`](../../../docs/trait/003_table_shaped_view.md) | Source trait spec — signature, methods, blanket impl, pipeline role |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/formatters.rs`](../../formatters.rs) | View extraction tests |
| [`tests/data.rs`](../../data.rs) | TreeNode and TableShapedView data structure tests |
