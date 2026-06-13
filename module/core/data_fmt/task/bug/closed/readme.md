# Closed Bugs

### Scope

- **Purpose**: Archive of all closed (fixed) bugs for data_fmt.
- **Responsibility**: One file per BUG-NNN; all bugs are confirmed fixed and tested.
- **In Scope**: Fixed bugs with root cause, fix location, pitfall, and test reference.
- **Out of Scope**: Open bugs (would live in `../open/` if created), feature tasks.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `001_unicode_display_width.md` | BUG-001: CJK/emoji display width in column alignment |
| `002_word_wrap_triple.md` | BUG-002: Three word-wrap bugs (tab-width=0, chunk trim, avail-per-line) |
| `003_unicode_box_separator_mismatch.md` | BUG-003: unicode_box header separator paired with non-Unicode column separator |
| `004_ascii_grid_corners.md` | BUG-004: AsciiGrid header separator used '|' instead of '+' at corners |
| `005_horizontal_rule_alignment.md` | BUG-005: Header separator padding wider than data rows |
| `006_bare_fold_no_wrap.md` | BUG-006: Bare fold style did not wrap when joined line exceeded terminal |
| `007_fold_point_zero.md` | BUG-007: Fold point zero produced empty primary header row |
| `008_empty_table_newlines.md` | BUG-008: Zero-column table emitted bare newlines instead of empty string |
| `009_multiline_color_bleed.md` | BUG-009: Row color wrap bled across sub-lines in multiline rows |
| `010_ansi_color_per_line.md` | BUG-010: Sub-row ANSI color wrap bled across newline boundaries |
| `011_multiline_width_calculation.md` | BUG-011: Column width used total string length instead of max single-line width |
| `012_expanded_render_method.md` | BUG-012: ExpandedFormatter accessed raw cell data bypassing ANSI color render |
| `013_multiline_detail_indent.md` | BUG-013: Multi-line detail indent prefix missing on continuation lines |
