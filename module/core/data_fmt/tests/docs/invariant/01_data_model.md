# Invariant Spec: Data Model

## Source
`docs/invariant/001_data_model.md`

## Test Implementation
`tests/data.rs`

## Enforcement Mechanism
`src/table_tree.rs:50-58` — `validate_row_length()` calls `assert!()` with message
`"row length {len} doesn't match headers length {headers_len}"`.

## Case Index

| ID | Name | Status |
|----|------|--------|
| IC-1 | RowBuilder panics when row is shorter than headers | ✅ |
| IC-2 | RowBuilder panics when row is longer than headers | ✅ |
| IC-3 | headers-only table renders header row and separator | ✅ |
| IC-4 | single-row table renders without error | ✅ |

---

### IC-1: RowBuilder panics when row is shorter than headers

- **Given:** A `RowBuilder` constructed with 3 header names (`"A"`, `"B"`, `"C"`).
- **When:** `add_row` is called with a row containing only 2 cells.
- **Then:** `assert!` fires immediately with the message
  `"row length 2 doesn't match headers length 3"`; the panic propagates to the
  caller; no partial row is stored.

---

### IC-2: RowBuilder panics when row is longer than headers

- **Given:** A `RowBuilder` constructed with 3 header names (`"A"`, `"B"`, `"C"`).
- **When:** `add_row` is called with a row containing 4 cells.
- **Then:** `assert!` fires immediately with the message
  `"row length 4 doesn't match headers length 3"`; the panic propagates to the
  caller; no partial row is stored.

---

### IC-3: headers-only table renders header row and separator

- **Given:** A `RowBuilder` with 2 headers and no rows added, built with `.build_view()`.
- **When:** Rendered with `TableFormatter`.
- **Then:** Output contains the header row and separator line (at most 2 lines); no data
  rows are emitted; no panic. Only a truly empty table (zero columns) renders to `""`.
- **Note:** Covered by `empty_table_renders_to_empty_string` in `tests/data.rs`.
  Also validated by T17 (`auto_wrap_test`) and T20 (`auto_fold_test`).
  Implementation: `format_internal` early-exits when `headers.is_empty()`
  (`src/formatters/table/mod.rs`). Prior guard was `rows.is_empty()`, which suppressed
  headers — changed so callers always see the column structure for empty result sets.

---

### IC-4: single-row table renders without error

- **Given:** A `RowBuilder` with 2 headers and exactly one row added.
- **When:** Rendered with `TableFormatter`.
- **Then:** Output contains exactly one data row with the correct cell values;
  no panic; the table structure (header + separator + data row) is well-formed.
- **Note:** Covered by `single_row_table_renders_without_error` in `tests/data.rs`
  (asserts cell values present, exactly 3 non-empty lines).
