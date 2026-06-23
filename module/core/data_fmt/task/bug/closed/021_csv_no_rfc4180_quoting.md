# BUG-021: CSV Values Containing Commas Not Quoted (RFC 4180)

- **Status:** Closed (Fixed)
- **Root Cause:** CSV cell text containing commas was emitted raw without RFC 4180 quoting,
  making in-value commas indistinguishable from column separators.
- **Fix Location:** `src/formatters/table/row_rendering.rs` — added `csv_quote()` helper
  called in both `format_row` and `format_row_colored` CSV paths. Wraps cells in
  double-quotes and doubles internal `"` when text contains `,` or `"`.
- **Pitfall:** Apply quoting AFTER newline escaping. Only CSV (comma separator) needs
  RFC 4180 quoting — TSV does not.
- **Test Reference:** `tests/corner_case_bug_reproducer_test.rs` —
  `bug_021_csv_comma_in_value_not_quoted`, `bug_021_csv_double_quote_in_value`,
  `bug_021_tsv_comma_not_quoted` tagged `bug_reproducer(BUG-021)`.
