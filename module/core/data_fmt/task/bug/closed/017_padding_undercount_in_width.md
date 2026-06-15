# BUG-017: Padding undercount in compute_total_row_width

- **Status**: Fixed
- **Severity**: Medium
- **Component**: `src/formatters/table/auto_fit.rs`

### Root Cause

Three functions (`compute_total_row_width`, `compute_column_budgets`, `determine_fold_point`) computed outer padding as `inner_padding * 2` (2 total units). But `format_single_line_row` in `rendering.rs` applies `inner_padding` spaces before AND after every cell — making the actual padding `inner_padding * 2 * N` where N is the number of columns. The undercount caused heading lines to be narrower than table body rows for any style with `inner_padding > 0` (bordered, markdown, grid, unicode_box).

### Fix Applied

- `compute_total_row_width`: `inner_padding * 2` → `inner_padding * 2 * column_widths.len()`
- `compute_column_budgets`: same change in overhead calculation
- `determine_fold_point`: moved padding into per-column accumulation inside the loop (`pad_per_col * (i + 1)`)

### Pitfall

Any overhead formula that accounts for cell padding must use per-column multiplication, not a flat constant. The rendering loop applies padding around every cell, not just at the outer edges.

### Test

`tests/table_caption_test.rs::heading_on_bordered_table_display_width_matches` (bug_reproducer)
