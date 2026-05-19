# Invariant: Data Model

### Scope

- **Purpose**: Drive test coverage for the data model structural invariants.
- **Responsibility**: Documents test cases for the data model invariants in `docs/invariant/001_data_model.md`.
- **In Scope**: Row length validation (panic on mismatch), empty builder output, single-row rendering, parallel row/row_details vectors, `TableShapedView` extraction, empty tree output.
- **Out of Scope**: ANSI/unicode invariants (see `invariant/002`); auto-wrap backward compatibility (see `invariant/003`).

### Enforcement Mechanism

`src/table_tree.rs:50-58` — `validate_row_length()` calls `assert!()` with message
`"row length {len} doesn't match headers length {headers_len}"`.

### Case Index

| ID | Name | Status |
|----|------|--------|
| IN-1 | RowBuilder panics when row is shorter than headers | ✅ |
| IN-2 | RowBuilder panics when row is longer than headers | ✅ |
| IN-3 | empty RowBuilder (no rows) renders to empty string | ✅ |
| IN-4 | single-row table renders without error | ✅ |
| IN-5 | row_details length always equals rows length (parallel vectors) | ✅ |
| IN-6 | TableShapedView extracts headers and rows from display-capable tree | ✅ |
| IN-7 | empty tree formatted without tree-structure artifacts | ✅ |

---

### IN-1: RowBuilder panics when row is shorter than headers

- **Given:** A `RowBuilder` constructed with 3 header names (`"A"`, `"B"`, `"C"`).
- **When:** `add_row` is called with a row containing only 2 cells.
- **Then:** `assert!` fires immediately with the message
  `"row length 2 doesn't match headers length 3"`; the panic propagates to the
  caller; no partial row is stored.

---

### IN-2: RowBuilder panics when row is longer than headers

- **Given:** A `RowBuilder` constructed with 3 header names (`"A"`, `"B"`, `"C"`).
- **When:** `add_row` is called with a row containing 4 cells.
- **Then:** `assert!` fires immediately with the message
  `"row length 4 doesn't match headers length 3"`; the panic propagates to the
  caller; no partial row is stored.

---

### IN-3: empty `RowBuilder` (no rows) renders to empty string

- **Given:** A `RowBuilder` with 2 headers and no rows added, built with `.build()`.
- **When:** Rendered with `TableFormatter`.
- **Then:** Output is `""`; no partial table structure, header row, or separator is
  emitted; no panic.
- **Note:** Covered by `empty_table_renders_to_empty_string` in `tests/data.rs`.
  Implementation: `format_internal` early-exits when both `headers` and `rows` are
  empty (`src/formatters/table/mod.rs`); `.build()` on an empty `RowBuilder` produces
  a root `TreeNode` with zero children since headers are not stored until `add_row`.

---

### IN-4: single-row table renders without error

- **Given:** A `RowBuilder` with 2 headers and exactly one row added.
- **When:** Rendered with `TableFormatter`.
- **Then:** Output contains exactly one data row with the correct cell values;
  no panic; the table structure (header + separator + data row) is well-formed.
- **Note:** Covered by `single_row_table_renders_without_error` in `tests/data.rs`
  (asserts cell values present, exactly 3 non-empty lines).

---

### IN-5: row_details length always equals rows length (parallel vectors)

- **Given:** A `RowBuilder` where `add_row` is called N times, with and without
  `row_detail` set on each row.
- **When:** The builder produces its internal representation.
- **Then:** The `row_details` vector has the same length as the `rows` vector; rows
  without an explicit detail have `None` entries; the two vectors remain in sync
  throughout — no index-out-of-bounds access can occur when pairing a row with its
  detail during rendering.

---

### IN-6: TableShapedView extracts headers and rows from display-capable tree

- **Given:** A tree structure implementing the `TableShaped` display trait, with a
  known set of column names and rows.
- **When:** `TableShapedView` is constructed from the tree and passed to `TableFormatter`.
- **Then:** The extracted headers match the column definitions; each extracted row
  contains the same cell values as the source tree node; no rows are missing or
  duplicated; the rendered table is structurally valid (column count consistent
  across header and all data rows).

---

### IN-7: empty tree formatted without tree-structure artifacts

- **Given:** A `TreeNode` with no children (root-only tree) passed to
  `TreeFormatter::format()` or `TreeFormatter::format_aligned()`.
- **When:** Rendered.
- **Then:** No `├──`, `└──`, or `│` connector characters are emitted; output is
  minimal — either an empty string or at most one line containing only the root
  node name; no column-separator artifacts appear; no panic occurs.
- **Note:** Cross-reference: `algorithm/003_tree_column_alignment.md` AC-4 documents
  this as an algorithm edge case for `AlignedTreeFormatter`.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/invariant/001_data_model.md`](../../../docs/invariant/001_data_model.md) | Source invariant spec — row length enforcement, empty builder, parallel vectors |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/data.rs`](../../data.rs) | Invariant test implementation |
