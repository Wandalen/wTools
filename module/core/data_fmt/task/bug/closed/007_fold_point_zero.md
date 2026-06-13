# BUG-007: Fold Point Zero Produced Empty Primary Header Row

- **Status:** ✅ Closed (Fixed)
- **Affects:** `src/formatters/table/auto_fit.rs` — `calculate_fold_point`

## Root Cause

When the first column alone exceeded terminal width, `calculate_fold_point` returned `0`.
Fold point `0` means "zero columns in primary table", producing an empty header row with
no visible column names — violating Invariant 1 (header row must never be empty).

## Fix Location

`src/formatters/table/auto_fit.rs` — `calculate_fold_point` early-return path.
`Fix(BUG-007)`: `return i.max(1)` — always return at least `1` so the primary table
emits at least one column.

## Pitfall

Never return `0` from `calculate_fold_point`. The minimum meaningful fold point is `1`;
returning `0` causes the formatter to produce a table with no columns, which is
undefined output per the data model invariant.

## Test Reference

`tests/auto_fold_test.rs` — `bug_reproducer(BUG-007)`:
`bug_reproducer_fold_point_zero`
