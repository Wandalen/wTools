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
| IC-3 | empty table renders to empty string | ✅ |
| IC-4 | single-row table renders without error | ✅ |

---

### IC-1: RowBuilder panics when row is shorter than headers

**Given:** A `RowBuilder` constructed with 3 header names (`"A"`, `"B"`, `"C"`).
**When:** `add_row` is called with a row containing only 2 cells.
**Then:** `assert!` fires immediately with the message
`"row length 2 doesn't match headers length 3"`; the panic propagates to the
caller; no partial row is stored.

---

### IC-2: RowBuilder panics when row is longer than headers

**Given:** A `RowBuilder` constructed with 3 header names (`"A"`, `"B"`, `"C"`).
**When:** `add_row` is called with a row containing 4 cells.
**Then:** `assert!` fires immediately with the message
`"row length 4 doesn't match headers length 3"`; the panic propagates to the
caller; no partial row is stored.

---

### IC-3: empty table renders to empty string

**Given:** A `RowBuilder` with 2 headers and no rows added, built with `.build()`.
**When:** Rendered with `TableFormatter`.
**Then:** Output is an empty string `""`; no panic; no stray header row or
separator lines are emitted.
**Note:** Covered by `empty_table_renders_to_empty_string` in `tests/data.rs`.
Implementation fix: `format_internal` early-exits when both headers and rows are
empty (`src/formatters/table/mod.rs`).

---

### IC-4: single-row table renders without error

**Given:** A `RowBuilder` with 2 headers and exactly one row added.
**When:** Rendered with `TableFormatter`.
**Then:** Output contains exactly one data row with the correct cell values;
no panic; the table structure (header + separator + data row) is well-formed.
**Note:** Covered by `single_row_table_renders_without_error` in `tests/data.rs`
(asserts cell values present, exactly 3 non-empty lines).
