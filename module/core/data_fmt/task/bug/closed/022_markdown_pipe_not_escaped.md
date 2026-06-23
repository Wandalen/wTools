# BUG-022: Markdown Cells Containing Pipe Not Escaped

- **Status:** Closed (Fixed)
- **Root Cause:** Markdown cell text containing `|` was emitted raw, indistinguishable
  from column separator pipe, producing extra columns and corrupting table structure.
- **Fix Location:** `src/formatters/table/row_rendering.rs` — added pipe escaping
  (`|` → `\|`) in cell preparation phase of both `format_row` and `format_row_colored`
  when `config.is_markdown()` is true. Config accessor `is_markdown()` added to
  `src/config/table_config.rs`.
- **Pitfall:** Only escape in Markdown mode — other table styles emit `|` as border
  character separately via the rendering pipeline, not embedded in cell text. Must escape
  BEFORE color wrapping in `format_row_colored`.
- **Test Reference:** `tests/corner_case_bug_reproducer_test.rs` —
  `bug_022_markdown_pipe_in_cell_not_escaped`, `bug_022_markdown_pipe_in_header`
  tagged `bug_reproducer(BUG-022)`.
