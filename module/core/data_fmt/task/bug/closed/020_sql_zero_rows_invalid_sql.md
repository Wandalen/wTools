# BUG-020: SqlFormatter Zero Rows Produces Invalid SQL

- **Status:** Closed (Fixed)
- **Root Cause:** `SqlFormatter::format` always emitted `INSERT INTO ... VALUES;` even with
  zero data rows, producing invalid SQL (`VALUES;` without any row tuples).
- **Fix Location:** `src/formatters/sql.rs` — early return `Ok(String::new())` when
  `data.rows.is_empty()`.
- **Pitfall:** Guard on rows, not columns — a headers-only table has nothing to insert.
- **Test Reference:** `tests/corner_case_bug_reproducer_test.rs` —
  `bug_020_sql_zero_rows_invalid_sql` tagged `bug_reproducer(BUG-020)`.
